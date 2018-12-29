use crate::bits::Bits;
use crate::{ffi, header};
use crate::{HDR_SIZE, MAX_FRAME_SYNC_MATCHES, MAX_FREE_FORMAT_FRAME_SIZE};

#[derive(Copy, Clone)]
pub struct Scratch {
    pub bits: BitsProxy,
    pub grbuf: [f32; 576 * 2],
    pub scf: [f32; 40],
    pub syn: [f32; 64 * 33],
    pub ist_pos: [[u8; 39]; 2],
}

// used to avoid self referencial structs
#[derive(Copy, Clone)]
pub struct BitsProxy {
    pub position: usize,
    pub len: usize,
    pub maindata: [u8; 2815],
}

impl BitsProxy {
    pub fn with_bits<F, T>(&mut self, fun: F) -> T
    where
        F: FnOnce(&mut Bits) -> T,
    {
        let mut bits = Bits::new_with_pos(&self.maindata, self.position);
        let res = fun(&mut bits);
        self.position = bits.position;
        res
    }
}

impl Default for BitsProxy {
    fn default() -> Self {
        BitsProxy {
            position: 0,
            len: 2815,
            maindata: [0; 2815],
        }
    }
}

impl Default for Scratch {
    fn default() -> Self {
        Scratch {
            bits: BitsProxy::default(),
            grbuf: [0.0; 576 * 2],
            scf: [0.0; 40],
            syn: [0.0; 64 * 33],
            ist_pos: [[0; 39]; 2],
        }
    }
}

pub fn find_frame(mp3: &[u8], free_format_bytes: &mut i32, ptr_frame_bytes: &mut i32) -> i32 {
    let valid_frames = mp3
        .windows(HDR_SIZE as _)
        .enumerate()
        .filter(|(_, hdr)| header::is_valid(hdr))
        .map(|(pos, _)| pos);
    for pos in valid_frames {
        let mp3_view = &mp3[pos..];
        let mut frame_bytes = header::frame_bytes(mp3_view, *free_format_bytes);
        let mut frame_and_padding = frame_bytes + header::padding(mp3_view);

        let mut k = HDR_SIZE;
        while frame_bytes == 0
            && k < MAX_FREE_FORMAT_FRAME_SIZE
            && pos as i32 + 2 * k < mp3.len() as i32 - HDR_SIZE
        {
            if header::compare(mp3_view, &mp3_view[(k as _)..]) {
                let fb = k - header::padding(mp3_view);
                let nextfb = fb + header::padding(&mp3_view[(k as _)..]);
                if pos as i32 + k + nextfb + HDR_SIZE < mp3.len() as i32
                    && header::compare(mp3_view, &mp3_view[((k + nextfb) as _)..])
                {
                    frame_and_padding = k;
                    frame_bytes = fb;
                    *free_format_bytes = fb;
                }
            }
            k += 1;
        }

        if (frame_bytes != 0
            && pos as i32 + frame_and_padding <= mp3.len() as i32
            && match_frame(mp3_view, frame_bytes))
            || (pos == 0 && frame_and_padding == mp3.len() as i32)
        {
            *ptr_frame_bytes = frame_and_padding;
            return pos as i32;
        }
        *free_format_bytes = 0;
    }
    *ptr_frame_bytes = 0;
    // match c version behavior, returns 0 when len < 4
    mp3.len().saturating_sub(HDR_SIZE as _) as i32
}

pub fn match_frame(hdr: &[u8], frame_bytes: i32) -> bool {
    let mut i = 0;
    for nmatch in 0..MAX_FRAME_SYNC_MATCHES {
        i += (header::frame_bytes(&hdr[i..], frame_bytes) + header::padding(&hdr[i..])) as usize;
        if i + HDR_SIZE as usize > hdr.len() {
            return nmatch > 0;
        } else if !header::compare(hdr, &hdr[i..]) {
            return false;
        }
    }
    true
}

pub fn synth_granule(
    qmf_state: &mut [f32],
    grbuf: &mut [f32],
    nbands: usize,
    nch: usize,
    pcm: &mut [i16],
    lins: &mut [f32],
) {
    for i in 0..nch {
        dct_2(&mut grbuf[(576 * i)..], nbands);
    }
    lins[..(15 * 64)].copy_from_slice(&qmf_state[..(15 * 64)]);
    for i in (0..nbands).step_by(2) {
        unsafe {
            ffi::mp3d_synth(
                grbuf.as_mut_ptr().add(i),
                pcm.as_mut_ptr().add(32 * nch * i),
                nch as _,
                lins.as_mut_ptr().add(i * 64),
            )
        }
    }
    if nch == 1 {
        for (qmf, lin) in qmf_state
            .iter_mut()
            .zip(lins.iter().skip(nbands * 64))
            .take(15 * 64)
            .step_by(2)
        {
            *qmf = *lin;
        }
    } else {
        qmf_state[..(15 * 64)].copy_from_slice(&lins[(nbands * 64)..(nbands * 64 + 15 * 64)]);
    };
}

#[allow(clippy::excessive_precision)]
pub fn dct_2(grbuf: &mut [f32], n: usize) {
    let g_sec: [libc::c_float; 24] = [
        10.190_008_16,
        0.500_603_02,
        0.502_419_29,
        3.407_608_51,
        0.505_470_93,
        0.522_498_61,
        2.057_780_98,
        0.515_447_32,
        0.566_944_06,
        1.484_164_60,
        0.531_042_58,
        0.646_821_80,
        1.169_439_91,
        0.553_103_92,
        0.788_154_60,
        0.972_568_21,
        0.582_934_98,
        1.060_677_65,
        0.839_349_63,
        0.622_504_12,
        1.722_447_16,
        0.744_536_28,
        0.674_808_32,
        5.101_148_61,
    ];
    for k in 0..n {
        let mut t: [[f32; 8]; 4] = [[0.; 8]; 4];
        let w = &mut grbuf[k..];
        for i in 0..8 {
            let x0 = w[i * 18];
            let x1 = w[(15 - i) * 18];
            let x2 = w[(16 + i) * 18];
            let x3 = w[(31 - i) * 18];
            let t0 = x0 + x3;
            let t1 = x1 + x2;
            let t2 = (x1 - x2) * g_sec[3 * i];
            let t3 = (x0 - x3) * g_sec[3 * i + 1];
            t[0][i] = t0 + t1;
            t[1][i] = (t0 - t1) * g_sec[3 * i + 2];
            t[2][i] = t3 + t2;
            t[3][i] = (t3 - t2) * g_sec[3 * i + 2];
        }
        for x in t.iter_mut().take(4) {
            let mut x0_0 = x[0];
            let mut x1_0 = x[1];
            let mut x2_0 = x[2];
            let mut x3_0 = x[3];
            let mut x4 = x[4];
            let mut x5 = x[5];
            let mut x6 = x[6];
            let mut x7 = x[7];
            let mut xt = x0_0 - x7;
            x0_0 += x7;
            x7 = x1_0 - x6;
            x1_0 += x6;
            x6 = x2_0 - x5;
            x2_0 += x5;
            x5 = x3_0 - x4;
            x3_0 += x4;
            x4 = x0_0 - x3_0;
            x0_0 += x3_0;
            x3_0 = x1_0 - x2_0;
            x1_0 += x2_0;
            x[0] = x0_0 + x1_0;
            x[4] = (x0_0 - x1_0) * 0.707_106_77;
            x5 += x6;
            x6 = (x6 + x7) * 0.707_106_77;
            x7 += xt;
            x3_0 = (x3_0 + x4) * 0.707_106_77;
            /* rotate by PI/8 */
            x5 -= x7 * 0.198_912_367;
            x7 += x5 * 0.382_683_432;
            x5 -= x7 * 0.198_912_367;
            x0_0 = xt - x6;
            xt += x6;
            x[1] = (xt + x7) * 0.509_795_61;
            x[2] = (x4 + x3_0) * 0.541_196_11;
            x[3] = (x0_0 - x5) * 0.601_344_88;
            x[5] = (x0_0 + x5) * 0.899_976_19;
            x[6] = (x4 - x3_0) * 1.306_563_02;
            x[7] = (xt - x7) * 2.562_915_56;
        }
        for i in 0..7 {
            let offset = 4 * 18 * i;
            w[offset] = t[0][i];
            w[offset + 18] = t[2][i] + t[3][i] + t[3][i + 1];
            w[offset + 2 * 18] = t[1][i] + t[1][i + 1];
            w[offset + 3 * 18] = t[2][i + 1] + t[3][i] + t[3][i + 1];
        }
        let w = &mut w[(4 * 18 * 7)..];
        w[0] = t[0][7];
        w[18] = t[2][7] + t[3][7];
        w[2 * 18] = t[1][7];
        w[3 * 18] = t[3][7];
    }
}
