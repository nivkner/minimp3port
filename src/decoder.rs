use crate::bits::Bits;
use crate::{ffi, header};
use crate::{HDR_SIZE, MAX_FRAME_SYNC_MATCHES, MAX_FREE_FORMAT_FRAME_SIZE};

#[derive(Copy, Clone)]
pub struct Scratch {
    pub bits: BitsProxy,
    pub grbuf: [[f32; 576]; 2],
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
            grbuf: [[0.0; 576]; 2],
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
        unsafe { ffi::mp3d_DCT_II(grbuf.as_mut_ptr().add(576 * i), nbands as _) };
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
