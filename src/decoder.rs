use crate::header;
use crate::{HDR_SIZE, MAX_FRAME_SYNC_MATCHES, MAX_FREE_FORMAT_FRAME_SIZE};

#[derive(Copy, Clone)]
pub struct Decoder {
    pub mdct_overlap: [[f32; 288]; 2],
    pub qmf_state: [f32; 960],
    pub reserv: i32,
    pub free_format_bytes: i32,
    pub header: [u8; 4],
    pub reserv_buf: [u8; 511],
}

#[derive(Copy, Clone, Default)]
pub struct FrameInfo {
    pub frame_bytes: i32,
    pub channels: i32,
    pub hz: i32,
    pub layer: i32,
    pub bitrate_kbps: i32,
}

#[derive(Copy, Clone)]
pub struct Scratch {
    pub grbuf: [f32; 576 * 2],
    pub scf: [f32; 40],
    pub syn: [f32; 64 * 33],
    pub ist_pos: [[u8; 39]; 2],
}

impl Default for Scratch {
    fn default() -> Self {
        Scratch {
            grbuf: [0.0; 576 * 2],
            scf: [0.0; 40],
            syn: [0.0; 64 * 33],
            ist_pos: [[0; 39]; 2],
        }
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Decoder {
            mdct_overlap: [[0.; 288]; 2],
            qmf_state: [0.; 960],
            reserv: 0,
            free_format_bytes: 0,
            header: [0; 4],
            reserv_buf: [0; 511],
        }
    }
}

pub fn find_frame(mp3: &[u8], free_format_bytes: &mut i32, ptr_frame_bytes: &mut i32) -> i32 {
    let valid_frames = mp3
        .windows(HDR_SIZE)
        .enumerate()
        .filter(|(_, hdr)| header::is_valid(hdr))
        .map(|(pos, _)| pos);
    for pos in valid_frames {
        let mp3_view = &mp3[pos..];
        let mut frame_bytes = header::frame_bytes(mp3_view, *free_format_bytes);
        let mut frame_and_padding = frame_bytes + header::padding(mp3_view);

        let mut k = HDR_SIZE as i32;
        while frame_bytes == 0
            && k < MAX_FREE_FORMAT_FRAME_SIZE
            && pos as i32 + 2 * k < mp3.len() as i32 - HDR_SIZE as i32
        {
            if header::compare(mp3_view, &mp3_view[(k as _)..]) {
                let fb = k - header::padding(mp3_view);
                let nextfb = fb + header::padding(&mp3_view[(k as _)..]);
                if pos as i32 + k + nextfb + (HDR_SIZE as i32) < mp3.len() as i32
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
    mp3.len().saturating_sub(HDR_SIZE) as i32
}

pub fn match_frame(hdr: &[u8], frame_bytes: i32) -> bool {
    let mut i = 0;
    for nmatch in 0..MAX_FRAME_SYNC_MATCHES {
        i += (header::frame_bytes(&hdr[i..], frame_bytes) + header::padding(&hdr[i..])) as usize;
        if i + HDR_SIZE > hdr.len() {
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
        synth(
            &mut grbuf[i..],
            &mut pcm[(32 * nch * i)..],
            nch,
            &mut lins[(i * 64)..],
        )
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
    let g_sec: [f32; 24] = [
        10.190_008_16,
        0.500_603_02,
        0.502_419_29,
        3.407_608_51,
        0.505_470_93,
        0.522_498_61,
        2.057_780_98,
        0.515_447_32f32,
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
        0.674_808_32f32,
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

fn synth(x: &mut [f32], dst: &mut [i16], nch: usize, lins: &mut [f32]) {
    let g_win: [f32; 240] = [
        -1.0, 26.0, -31.0, 208.0, 218.0, 401.0, -519.0, 2063.0, 2000.0, 4788.0, -5517.0, 7134.0,
        5959.0, 35640.0, -39336.0, 74992.0, -1.0, 24.0, -35.0, 202.0, 222.0, 347.0, -581.0, 2080.0,
        1952.0, 4425.0, -5879.0, 7640.0, 5288.0, 33791.0, -41176.0, 74856.0, -1.0, 21.0, -38.0,
        196.0, 225.0, 294.0, -645.0, 2087.0, 1893.0, 4063.0, -6237.0, 8092.0, 4561.0, 31947.0,
        -43006.0, 74630.0, -1.0, 19.0, -41.0, 190.0, 227.0, 244.0, -711.0, 2085.0, 1822.0, 3705.0,
        -6589.0, 8492.0, 3776.0, 30112.0, -44821.0, 74313.0, -1.0, 17.0, -45.0, 183.0, 228.0,
        197.0, -779.0, 2075.0, 1739.0, 3351.0, -6935.0, 8840.0, 2935.0, 28289.0, -46617.0, 73908.0,
        -1.0, 16.0, -49.0, 176.0, 228.0, 153.0, -848.0, 2057.0, 1644.0, 3004.0, -7271.0, 9139.0,
        2037.0, 26482.0, -48390.0, 73415.0, -2.0, 14.0, -53.0, 169.0, 227.0, 111.0, -919.0, 2032.0,
        1535.0, 2663.0, -7597.0, 9389.0, 1082.0, 24694.0, -50137.0, 72835.0, -2.0, 13.0, -58.0,
        161.0, 224.0, 72.0, -991.0, 2001.0, 1414.0, 2330.0, -7910.0, 9592.0, 70.0, 22929.0,
        -51853.0, 72169.0, -2.0, 11.0, -63.0, 154.0, 221.0, 36.0, -1064.0, 1962.0, 1280.0, 2006.0,
        -8209.0, 9750.0, -998.0, 21189.0, -53534.0, 71420.0, -2.0, 10.0, -68.0, 147.0, 215.0, 2.0,
        -1137.0, 1919.0, 1131.0, 1692.0, -8491.0, 9863.0, -2122.0, 19478.0, -55178.0, 70590.0,
        -3.0, 9.0, -73.0, 139.0, 208.0, -29.0, -1210.0, 1870.0, 970.0, 1388.0, -8755.0, 9935.0,
        -3300.0, 17799.0, -56778.0, 69679.0, -3.0, 8.0, -79.0, 132.0, 200.0, -57.0, -1283.0,
        1817.0, 794.0, 1095.0, -8998.0, 9966.0, -4533.0, 16155.0, -58333.0, 68692.0, -4.0, 7.0,
        -85.0, 125.0, 189.0, -83.0, -1356.0, 1759.0, 605.0, 814.0, -9219.0, 9959.0, -5818.0,
        14548.0, -59838.0, 67629.0, -4.0, 7.0, -91.0, 117.0, 177.0, -106.0, -1428.0, 1698.0, 402.0,
        545.0, -9416.0, 9916.0, -7154.0, 12980.0, -61289.0, 66494.0, -5.0, 6.0, -97.0, 111.0,
        163.0, -127.0, -1498.0, 1634.0, 185.0, 288.0, -9585.0, 9838.0, -8540.0, 11455.0, -62684.0,
        65290.0,
    ];

    // offset is added instead of taking a subslice because outherwise the indexes are negative
    const OFFSET: usize = 15 * 64;

    // uses offset numbers because the slices overlap
    let x_extra = 576 * (nch - 1);
    let dst_extra = nch - 1;

    let zlin = &mut lins[(OFFSET)..];
    zlin[4 * 15] = x[18 * 16];
    zlin[4 * 15 + 1] = x[18 * 16 + x_extra];
    zlin[4 * 15 + 2] = x[0];
    zlin[4 * 15 + 3] = x[x_extra];
    zlin[4 * 31] = x[1 + 18 * 16];
    zlin[4 * 31 + 1] = x[1 + 18 * 16 + x_extra];
    zlin[4 * 31 + 2] = x[1];
    zlin[4 * 31 + 3] = x[1 + x_extra];
    synth_pair(&mut dst[dst_extra..], nch, &lins[(4 * 15 + 1)..]);
    synth_pair(
        &mut dst[(32 * nch + dst_extra)..],
        nch,
        &lins[(4 * 15 + 65)..],
    );
    synth_pair(dst, nch, &lins[(4 * 15)..]);
    synth_pair(&mut dst[(32 * nch)..], nch, &lins[(4 * 15 + 64)..]);

    #[inline]
    fn fun1(k: usize, i: usize, lins: &[f32], gwin: &[f32], a: &mut [f32], b: &mut [f32]) {
        let w0 = gwin[k * 2];
        let w1 = gwin[k * 2 + 1];
        let vz_offset = 4 * i + (15 - k) * 64;
        let vy_offset = 4 * i + k * 64;
        for j in 0..4 {
            b[j] += lins[vz_offset + j] * w1 + lins[vy_offset + j] * w0;
            a[j] += lins[vz_offset + j] * w0 - lins[vy_offset + j] * w1;
        }
    }

    #[inline]
    fn fun2(k: usize, i: usize, lins: &[f32], gwin: &[f32], a: &mut [f32], b: &mut [f32]) {
        let w0 = gwin[k * 2];
        let w1 = gwin[k * 2 + 1];
        let vz_offset = 4 * i + (15 - k) * 64;
        let vy_offset = 4 * i + k * 64;
        for j in 0..4 {
            b[j] += lins[vz_offset + j] * w1 + lins[vy_offset + j] * w0;
            a[j] += lins[vy_offset + j] * w1 - lins[vz_offset + j] * w0;
        }
    }

    for (i, gwin) in (0..15).rev().zip(g_win.chunks_exact(16)) {
        let mut a: [f32; 4] = [0.0; 4];
        let mut b: [f32; 4] = [0.0; 4];

        lins[(OFFSET + 4 * i)] = x[(18 * (31 - i))];
        lins[(OFFSET + 4 * i + 1)] = x[(18 * (31 - i)) + x_extra];
        lins[(OFFSET + 4 * i + 2)] = x[(1 + 18 * (31 - i))];
        lins[(OFFSET + 4 * i + 3)] = x[(1 + 18 * (31 - i)) + x_extra];
        lins[(OFFSET + 4 * (i + 16))] = x[(1 + 18 * (1 + i))];
        lins[(OFFSET + 4 * (i + 16) + 1)] = x[(1 + 18 * (1 + i)) + x_extra];
        lins[(OFFSET - 4 * (16 - i) + 2)] = x[(18 * (1 + i))];
        lins[(OFFSET - 4 * (16 - i) + 3)] = x[(18 * (1 + i)) + x_extra];

        fun1(0, i, lins, gwin, &mut a, &mut b);
        fun2(1, i, lins, gwin, &mut a, &mut b);
        fun1(2, i, lins, gwin, &mut a, &mut b);
        fun2(3, i, lins, gwin, &mut a, &mut b);
        fun1(4, i, lins, gwin, &mut a, &mut b);
        fun2(5, i, lins, gwin, &mut a, &mut b);
        fun1(6, i, lins, gwin, &mut a, &mut b);
        fun2(7, i, lins, gwin, &mut a, &mut b);

        dst[(15 - i) * nch + dst_extra] = scale_pcm(a[1]);
        dst[(17 + i) * nch + dst_extra] = scale_pcm(b[1]);
        dst[(15 - i) * nch] = scale_pcm(a[0]);
        dst[(17 + i) * nch] = scale_pcm(b[0]);
        dst[(47 - i) * nch + dst_extra] = scale_pcm(a[3]);
        dst[(49 + i) * nch + dst_extra] = scale_pcm(b[3]);
        dst[(47 - i) * nch] = scale_pcm(a[2]);
        dst[(49 + i) * nch] = scale_pcm(b[2]);
    }
}

fn scale_pcm(sample: f32) -> i16 {
    if sample >= 32766.5 {
        32767
    } else if sample <= -32767.5 {
        -32768
    } else {
        // round sample away for zero, to be compliant
        let s = (sample + 0.5) as i16;
        s - (s < 0) as i16
    }
}

fn synth_pair(pcm: &mut [i16], nch: usize, z: &[f32]) {
    let mut a = (z[14 * 64] - z[0]) * 29.0;
    a += (z[64] + z[13 * 64]) * 213.0;
    a += (z[12 * 64] - z[2 * 64]) * 459.0;
    a += (z[3 * 64] + z[11 * 64]) * 2037.0;
    a += (z[10 * 64] - z[4 * 64]) * 5153.0;
    a += (z[5 * 64] + z[9 * 64]) * 6574.0;
    a += (z[8 * 64] - z[6 * 64]) * 37489.0;
    a += z[7 * 64] * 75038.0;
    pcm[0] = scale_pcm(a);
    let z = &z[2..];
    a = z[14 * 64] * 104.0;
    a += z[12 * 64] * 1567.0;
    a += z[10 * 64] * 9727.0;
    a += z[8 * 64] * 64019.0;
    a += z[6 * 64] * -9975.0;
    a += z[4 * 64] * -45.0;
    a += z[2 * 64] * 146.0;
    a += z[0] * -5.0;
    pcm[16 * nch] = scale_pcm(a);
}
