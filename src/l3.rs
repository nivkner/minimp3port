use core::cmp;

use crate::bits::Bits;
use crate::{decoder, ffi, header};
use crate::{BITS_DEQUANTIZER_OUT, MAX_BITRESERVOIR_BYTES, MAX_SCFI, SHORT_BLOCK_TYPE};

#[derive(Copy, Clone, Default)]
pub struct GrInfo {
    // when the table is None, it corresponds to n_long_sfb == 0,
    // and n_short_sfb == 0
    pub sfb_table: Option<SFBTable>,
    pub part_23_length: u16,
    pub big_values: u16,
    pub scalefac_compress: u16,
    pub global_gain: u8,
    pub block_type: u8,
    pub mixed_block: bool,
    pub table_select: [u8; 3],
    pub region_count: [u8; 3],
    pub subblock_gain: [u8; 3],
    pub preflag: bool,
    pub scalefac_scale: u8,
    pub count1_table: u8,
    pub scfsi: u8,
}

// contains the table and describes the length
#[derive(Copy, Clone)]
pub enum SFBTable {
    // corresponds to n_long_sfb == 22
    Long(&'static [u8]),
    // corresponds to n_short_sfb == 39
    Short(&'static [u8]),
    // corresponds to n_short_sfb == 30
    // with the extra number as n_long_sfb
    Mixed(&'static [u8], u8),
}

#[rustfmt::skip]
static G_SCF_LONG: [[u8;23]; 8] = [
    [ 6,6,6,6,6,6,8,10,12,14,16,20,24,28,32,38,46,52,60,68,58,54,0 ],
    [ 12,12,12,12,12,12,16,20,24,28,32,40,48,56,64,76,90,2,2,2,2,2,0 ],
    [ 6,6,6,6,6,6,8,10,12,14,16,20,24,28,32,38,46,52,60,68,58,54,0 ],
    [ 6,6,6,6,6,6,8,10,12,14,16,18,22,26,32,38,46,54,62,70,76,36,0 ],
    [ 6,6,6,6,6,6,8,10,12,14,16,20,24,28,32,38,46,52,60,68,58,54,0 ],
    [ 4,4,4,4,4,4,6,6,8,8,10,12,16,20,24,28,34,42,50,54,76,158,0 ],
    [ 4,4,4,4,4,4,6,6,6,8,10,12,16,18,22,28,34,40,46,54,54,192,0 ],
    [ 4,4,4,4,4,4,6,6,8,10,12,16,20,24,30,38,46,56,68,84,102,26,0 ],
];

#[rustfmt::skip]
static G_SCF_SHORT: [[u8; 40]; 8] = [
    [ 4,4,4,4,4,4,4,4,4,6,6,6,8,8,8,10,10,10,12,12,12,14,14,14,18,18,18,24,24,24,30,30,30,40,40,40,18,18,18,0 ],
    [ 8,8,8,8,8,8,8,8,8,12,12,12,16,16,16,20,20,20,24,24,24,28,28,28,36,36,36,2,2,2,2,2,2,2,2,2,26,26,26,0 ],
    [ 4,4,4,4,4,4,4,4,4,6,6,6,6,6,6,8,8,8,10,10,10,14,14,14,18,18,18,26,26,26,32,32,32,42,42,42,18,18,18,0 ],
    [ 4,4,4,4,4,4,4,4,4,6,6,6,8,8,8,10,10,10,12,12,12,14,14,14,18,18,18,24,24,24,32,32,32,44,44,44,12,12,12,0 ],
    [ 4,4,4,4,4,4,4,4,4,6,6,6,8,8,8,10,10,10,12,12,12,14,14,14,18,18,18,24,24,24,30,30,30,40,40,40,18,18,18,0 ],
    [ 4,4,4,4,4,4,4,4,4,4,4,4,6,6,6,8,8,8,10,10,10,12,12,12,14,14,14,18,18,18,22,22,22,30,30,30,56,56,56,0 ],
    [ 4,4,4,4,4,4,4,4,4,4,4,4,6,6,6,6,6,6,10,10,10,12,12,12,14,14,14,16,16,16,20,20,20,26,26,26,66,66,66,0 ],
    [ 4,4,4,4,4,4,4,4,4,4,4,4,6,6,6,8,8,8,12,12,12,16,16,16,20,20,20,26,26,26,34,34,34,42,42,42,12,12,12,0 ],
];

#[rustfmt::skip]
static G_SCF_MIXED: [&'static [u8]; 8] = [
    &[ 6,6,6,6,6,6,6,6,6,8,8,8,10,10,10,12,12,12,14,14,14,18,18,18,24,24,24,30,30,30,40,40,40,18,18,18,0 ],
    &[ 12,12,12,4,4,4,8,8,8,12,12,12,16,16,16,20,20,20,24,24,24,28,28,28,36,36,36,2,2,2,2,2,2,2,2,2,26,26,26,0 ],
    &[ 6,6,6,6,6,6,6,6,6,6,6,6,8,8,8,10,10,10,14,14,14,18,18,18,26,26,26,32,32,32,42,42,42,18,18,18,0 ],
    &[ 6,6,6,6,6,6,6,6,6,8,8,8,10,10,10,12,12,12,14,14,14,18,18,18,24,24,24,32,32,32,44,44,44,12,12,12,0 ],
    &[ 6,6,6,6,6,6,6,6,6,8,8,8,10,10,10,12,12,12,14,14,14,18,18,18,24,24,24,30,30,30,40,40,40,18,18,18,0 ],
    &[ 4,4,4,4,4,4,6,6,4,4,4,6,6,6,8,8,8,10,10,10,12,12,12,14,14,14,18,18,18,22,22,22,30,30,30,56,56,56,0 ],
    &[ 4,4,4,4,4,4,6,6,4,4,4,6,6,6,6,6,6,10,10,10,12,12,12,14,14,14,16,16,16,20,20,20,26,26,26,66,66,66,0 ],
    &[ 4,4,4,4,4,4,6,6,4,4,4,6,6,6,8,8,8,12,12,12,16,16,16,20,20,20,26,26,26,34,34,34,42,42,42,12,12,12,0 ],
];

#[allow(clippy::unreadable_literal)]
static G_POW43: [f64; 145] = [
    0.0, -1.0, -2.519842, -4.326749, -6.349604, -8.549880, -10.902724, -13.390518, -16.000000,
    -18.720754, -21.544347, -24.463781, -27.473142, -30.567351, -33.741992, -36.993181, 0.0, 1.0,
    2.519842, 4.326749, 6.349604, 8.549880, 10.902724, 13.390518, 16.000000, 18.720754, 21.544347,
    24.463781, 27.473142, 30.567351, 33.741992, 36.993181, 40.317474, 43.711787, 47.173345,
    50.699631, 54.288352, 57.937408, 61.644865, 65.408941, 69.227979, 73.100443, 77.024898,
    81.000000, 85.024491, 89.097188, 93.216975, 97.382800, 101.593667, 105.848633, 110.146801,
    114.487321, 118.869381, 123.292209, 127.755065, 132.257246, 136.798076, 141.376907, 145.993119,
    150.646117, 155.335327, 160.060199, 164.820202, 169.614826, 174.443577, 179.305980, 184.201575,
    189.129918, 194.090580, 199.083145, 204.107210, 209.162385, 214.248292, 219.364564, 224.510845,
    229.686789, 234.892058, 240.126328, 245.389280, 250.680604, 256.000000, 261.347174, 266.721841,
    272.123723, 277.552547, 283.008049, 288.489971, 293.998060, 299.532071, 305.091761, 310.676898,
    316.287249, 321.922592, 327.582707, 333.267377, 338.976394, 344.709550, 350.466646, 356.247482,
    362.051866, 367.879608, 373.730522, 379.604427, 385.501143, 391.420496, 397.362314, 403.326427,
    409.312672, 415.320884, 421.350905, 427.402579, 433.475750, 439.570269, 445.685987, 451.822757,
    457.980436, 464.158883, 470.357960, 476.577530, 482.817459, 489.077615, 495.357868, 501.658090,
    507.978156, 514.317941, 520.677324, 527.056184, 533.454404, 539.871867, 546.308458, 552.764065,
    559.238575, 565.731879, 572.243870, 578.774440, 585.323483, 591.890898, 598.476581, 605.080431,
    611.702349, 618.342238, 625.000000, 631.675540, 638.368763, 645.079578,
];

pub fn read_side_info(bs: &mut Bits, gr: &mut [GrInfo], hdr: &[u8]) -> i32 {
    let mut sr_idx = header::get_my_sample_rate(hdr);
    if sr_idx != 0 {
        sr_idx -= 1
    }
    let mut gr_count = if header::is_mono(hdr) { 1 } else { 2 };
    let mut scfsi = 0;

    let main_data_begin = if header::test_mpeg1(hdr) {
        gr_count *= 2;
        let data = bs.get_bits(9);
        scfsi = bs.get_bits(7 + gr_count as u32);
        data
    } else {
        bs.get_bits(8 + gr_count as u32) >> gr_count
    };

    let mut part_23_sum = 0;
    let mut tables: i32;
    let mut scfsi = scfsi as i32;

    for i in 0..gr_count {
        let gr = &mut gr[i];
        if header::is_mono(hdr) {
            scfsi <<= 4;
        }
        gr.part_23_length = bs.get_bits(12) as _;
        part_23_sum += i32::from(gr.part_23_length);
        gr.big_values = bs.get_bits(9) as _;
        if gr.big_values > 288 {
            return -1;
        }

        gr.global_gain = bs.get_bits(8) as _;
        gr.scalefac_compress = bs.get_bits(if header::test_mpeg1(hdr) { 4 } else { 9 }) as _;
        gr.sfb_table = Some(SFBTable::Long(&G_SCF_LONG[sr_idx as usize]));

        if bs.get_bits(1) != 0 {
            gr.block_type = bs.get_bits(2) as _;
            if gr.block_type == 0 {
                return -1;
            }

            gr.mixed_block = bs.get_bits(1) != 0;
            gr.region_count[0] = 7;
            gr.region_count[1] = 255;
            if gr.block_type == SHORT_BLOCK_TYPE as _ {
                scfsi &= 0x0F0F;

                if !gr.mixed_block {
                    gr.region_count[0] = 8;
                    gr.sfb_table = Some(SFBTable::Short(&G_SCF_SHORT[sr_idx as usize]));
                } else {
                    let long_sfb_count = if header::test_mpeg1(hdr) { 8 } else { 6 };
                    gr.sfb_table = Some(SFBTable::Mixed(
                        G_SCF_MIXED[sr_idx as usize],
                        long_sfb_count,
                    ));
                }
            }

            tables = bs.get_bits(10) as _;
            tables <<= 5;

            for i in 0..3 {
                gr.subblock_gain[i] = bs.get_bits(3) as _
            }
        } else {
            tables = bs.get_bits(15) as _;

            gr.region_count[0] = bs.get_bits(4) as _;
            gr.region_count[1] = bs.get_bits(3) as _;
            gr.region_count[2] = 255;
        }

        gr.table_select[0] = (tables >> 10) as _;
        gr.table_select[1] = ((tables >> 5) & 31) as _;
        gr.table_select[2] = (tables & 31) as _;
        gr.preflag = if header::test_mpeg1(hdr) {
            bs.get_bits(1) != 0
        } else {
            gr.scalefac_compress >= 500
        };

        gr.scalefac_scale = bs.get_bits(1) as _;
        gr.count1_table = bs.get_bits(1) as _;
        gr.scfsi = ((scfsi >> 12) & 15) as _;
        scfsi <<= 4;
    }

    if part_23_sum + bs.position as i32 > bs.limit() as i32 + (main_data_begin as i32) * 8 {
        -1
    } else {
        main_data_begin as _
    }
}

pub fn restore_reservoir(
    decoder: &mut ffi::mp3dec_t,
    bs: &mut Bits,
    scratch_bits: &mut decoder::BitsProxy,
    main_data_begin: u32,
) -> bool {
    let frame_bytes = (bs.limit() - bs.position) / 8;
    let bytes_have = cmp::min(decoder.reserv, main_data_begin as i32) as usize;
    let reserve_start = cmp::max(0, decoder.reserv - main_data_begin as i32) as usize;
    scratch_bits.maindata[..bytes_have]
        .copy_from_slice(&decoder.reserv_buf[reserve_start..(reserve_start + bytes_have)]);
    let bs_bytes = bs.position / 8;
    scratch_bits.maindata[(bytes_have)..(bytes_have + frame_bytes)]
        .copy_from_slice(&bs.data[bs_bytes..(bs_bytes + frame_bytes)]);
    scratch_bits.len = bytes_have + frame_bytes;
    scratch_bits.position = 0;
    decoder.reserv >= main_data_begin as i32
}

pub fn save_reservoir(decoder: &mut ffi::mp3dec_t, bits: &mut decoder::BitsProxy) {
    let mut position = bits.position / 8;
    let mut remains = bits.len - position;
    if remains > MAX_BITRESERVOIR_BYTES {
        position += remains - MAX_BITRESERVOIR_BYTES;
        remains = MAX_BITRESERVOIR_BYTES;
    }
    if remains > 0 {
        decoder.reserv_buf[..remains]
            .copy_from_slice(&bits.maindata[position..(position + remains)])
    }
    decoder.reserv = remains as _;
}

pub fn decode_scalefactors(
    hdr: &[u8],
    ist_pos: &mut [u8],
    bs: &mut Bits,
    gr: &GrInfo,
    scf: &mut [f32],
    ch: i32,
) {
    let g_scf_partitions: [[u8; 28]; 3] = [
        [
            6, 5, 5, 5, 6, 5, 5, 5, 6, 5, 7, 3, 11, 10, 0, 0, 7, 7, 7, 0, 6, 6, 6, 3, 8, 8, 5, 0,
        ],
        [
            8, 9, 6, 12, 6, 9, 9, 9, 6, 9, 12, 6, 15, 18, 0, 0, 6, 15, 12, 0, 6, 12, 9, 6, 6, 18,
            9, 0,
        ],
        [
            9, 9, 6, 12, 9, 9, 9, 9, 9, 9, 12, 6, 18, 18, 0, 0, 12, 12, 12, 0, 12, 9, 9, 6, 15, 12,
            9, 0,
        ],
    ];

    let scf_partition = match gr.sfb_table {
        Some(SFBTable::Long(_)) | None => &g_scf_partitions[0],
        Some(SFBTable::Mixed(..)) => &g_scf_partitions[1],
        Some(SFBTable::Short(_)) => &g_scf_partitions[2],
    };

    let mut iscf: [u8; 40] = [0; 40];
    if header::test_mpeg1(hdr) {
        let g_scfc_decode: [u8; 16] = [0, 1, 2, 3, 12, 5, 6, 7, 9, 10, 11, 13, 14, 15, 18, 19];
        let part = g_scfc_decode[gr.scalefac_compress as usize] as u8;
        let scf_size = [part >> 2, part >> 2, part & 3, part & 3];
        read_scalefactors(
            &mut iscf,
            ist_pos,
            &scf_size,
            scf_partition,
            bs,
            gr.scfsi.into(),
        )
    } else {
        let mut scf_size = [0; 4];
        let g_mod: [u8; 24] = [
            5, 5, 4, 4, 5, 5, 4, 1, 4, 3, 1, 1, 5, 6, 6, 1, 4, 4, 4, 1, 4, 3, 1, 1,
        ];
        let ist: i32 = (header::test_1_stereo(hdr) && (ch != 0)) as i32;
        let mut sfc = i32::from(gr.scalefac_compress) >> ist;
        let mut k = ist as usize * 3 * 4;
        while sfc >= 0 {
            let mut modprod = 1;
            for i in (0..=3).rev() {
                scf_size[i] = (sfc / modprod % i32::from(g_mod[k + i])) as u8;
                modprod *= i32::from(g_mod[k + i]);
            }
            sfc -= modprod;
            k += 4
        }
        read_scalefactors(&mut iscf, ist_pos, &scf_size, &scf_partition[k..], bs, -16)
    }

    let scf_shift: i32 = i32::from(gr.scalefac_scale) + 1;
    let sfb_length = match gr.sfb_table {
        Some(SFBTable::Long(_)) | None => {
            if gr.preflag {
                let g_preamp: [u8; 10] = [1, 1, 1, 1, 2, 2, 3, 3, 3, 2];
                for (iscf, preamp) in iscf.iter_mut().skip(11).zip(g_preamp.iter()) {
                    *iscf += preamp
                }
            }
            22
        }
        Some(SFBTable::Mixed(_, extra)) => {
            let sh = 3 - scf_shift as u8;
            for iscf in iscf[(extra as usize)..].chunks_mut(3).take(30 / 3) {
                iscf[0] += gr.subblock_gain[0] << sh;
                iscf[1] += gr.subblock_gain[1] << sh;
                iscf[2] += gr.subblock_gain[2] << sh;
            }
            30 + extra as usize
        }
        Some(SFBTable::Short(_)) => {
            let sh = 3 - scf_shift as u8;
            for iscf in iscf.chunks_mut(3).take(39 / 3) {
                iscf[0] += gr.subblock_gain[0] << sh;
                iscf[1] += gr.subblock_gain[1] << sh;
                iscf[2] += gr.subblock_gain[2] << sh;
            }
            39
        }
    };
    let gain_exp = i32::from(gr.global_gain) + BITS_DEQUANTIZER_OUT * 4
        - 210
        - if header::is_ms_stereo(hdr) { 2 } else { 0 };
    let gain = unsafe { ffi::L3_ldexp_q2((1 << (MAX_SCFI / 4)) as f32, MAX_SCFI - gain_exp) };
    // the length of the scalefactor band, whichever type it is
    for i in 0..sfb_length {
        scf[i] = unsafe { ffi::L3_ldexp_q2(gain, i32::from(iscf[i]) << scf_shift) };
    }
}

fn read_scalefactors(
    mut scf: &mut [u8],
    mut ist_pos: &mut [u8],
    scf_size: &[u8],
    scf_count: &[u8],
    bitbuf: &mut Bits,
    mut scfsi: i32,
) {
    for i in (0..4).take_while(|&i| scf_count[i] != 0) {
        let cnt = scf_count[i] as usize;
        if 0 != scfsi & 8 {
            scf[..cnt].copy_from_slice(&ist_pos[..cnt]);
        } else {
            let bits: i32 = scf_size[i].into();
            let max_scf = if scfsi < 0 { (1 << bits) - 1 } else { -1 };
            for (ist_pos, scf) in ist_pos.iter_mut().zip(scf.iter_mut()).take(cnt) {
                let s = bitbuf.get_bits(bits as u32) as i32;
                *ist_pos = (if s == max_scf { -1 } else { s }) as u8;
                *scf = s as u8;
            }
        }
        ist_pos = &mut ist_pos[cnt..];
        scf = &mut scf[cnt..];
        scfsi *= 2
    }
    scf[..3].copy_from_slice(&[0; 3]);
}

pub unsafe fn decode(
    decoder: &mut ffi::mp3dec_t,
    scratch: &mut decoder::Scratch,
    native_gr_info: &[GrInfo],
    channel_num: usize,
) {
    for (channel, info) in native_gr_info.iter().enumerate().take(channel_num) {
        let ist_pos = &mut scratch.ist_pos;
        let scf = &mut scratch.scf;
        let grbuf = &mut scratch.grbuf;
        let layer3gr_limit = scratch.bits.position as i32 + i32::from(info.part_23_length);
        scratch.bits.with_bits(|mut bs| {
            decode_scalefactors(
                &decoder.header,
                &mut ist_pos[channel],
                &mut bs,
                info,
                scf,
                channel as _,
            );
            huffman(
                &mut grbuf[(channel * 576)..],
                &mut bs,
                info,
                scf,
                layer3gr_limit,
            );
        });
    }

    if header::test_1_stereo(&decoder.header) {
        intensity_stereo(
            &mut scratch.grbuf,
            &mut scratch.ist_pos[1],
            &native_gr_info[0],
            (native_gr_info[1].scalefac_compress & 1).into(),
            &decoder.header,
        );
    } else if header::is_ms_stereo(&decoder.header) {
        midside_stereo(&mut scratch.grbuf, 576);
    }

    for (channel, gr_info) in native_gr_info.iter().enumerate().take(channel_num) {
        let mut aa_bands = 31;
        let n_long_bands = if gr_info.mixed_block {
            2 << (header::get_my_sample_rate(&decoder.header) == 2) as usize
        } else {
            0
        };
        match gr_info.sfb_table {
            Some(SFBTable::Short(table)) => {
                aa_bands = n_long_bands - 1;
                ffi::L3_reorder(
                    scratch.grbuf[(channel * 576)..]
                        .as_mut_ptr()
                        .offset((n_long_bands * 18) as isize),
                    scratch.syn.as_mut_ptr(),
                    table.as_ptr(),
                );
            }
            Some(SFBTable::Mixed(table, extra)) => {
                aa_bands = n_long_bands - 1;
                ffi::L3_reorder(
                    scratch.grbuf[(channel * 576)..]
                        .as_mut_ptr()
                        .offset((n_long_bands * 18) as isize),
                    scratch.syn.as_mut_ptr(),
                    table[(extra as usize)..].as_ptr(),
                );
            }
            _ => (),
        }
        ffi::L3_antialias(scratch.grbuf[(channel * 576)..].as_mut_ptr(), aa_bands);
        ffi::L3_imdct_gr(
            scratch.grbuf[(channel * 576)..].as_mut_ptr(),
            decoder.mdct_overlap[channel].as_mut_ptr(),
            gr_info.block_type.into(),
            n_long_bands as u32,
        );
        ffi::L3_change_sign(scratch.grbuf[(channel * 576)..].as_mut_ptr());
    }
}

fn midside_stereo(data: &mut [f32], n: usize) {
    let (left_slice, right_slice) = data.split_at_mut(576);
    for (left, right) in left_slice.iter_mut().zip(right_slice.iter_mut()).take(n) {
        let a = *left;
        let b = *right;
        *left = a + b;
        *right = a - b;
    }
}

fn intensity_stereo(
    left: &mut [f32],
    ist_pos: &mut [u8],
    gr_info: &GrInfo,
    scalefac_next: i32,
    hdr: &[u8],
) {
    let mut max_band: [i32; 3] = [0; 3];
    let (has_short, table, n_sfb) = match gr_info.sfb_table {
        Some(SFBTable::Long(tab)) => (false, tab, 22),
        Some(SFBTable::Short(tab)) => (true, tab, 39),
        Some(SFBTable::Mixed(tab, extra)) => (true, tab, 30 + usize::from(extra)),
        None => (false, &[] as _, 0),
    };
    unsafe {
        ffi::L3_stereo_top_band(
            left[576..].as_mut_ptr(),
            table.as_ptr(),
            n_sfb as i32,
            max_band.as_mut_ptr(),
        );
    }

    if !has_short {
        let max_val = *max_band.iter().max().unwrap();
        max_band.iter_mut().for_each(|band| *band = max_val);
    }

    let default_pos = if header::test_mpeg1(hdr) { 3 } else { 0 };
    let max_blocks = if has_short { 3 } else { 1 };
    for (i, band) in max_band.iter().enumerate().take(max_blocks) {
        let itop = n_sfb - max_blocks + i;
        let prev = itop - max_blocks;
        ist_pos[itop] = if *band >= prev as i32 {
            default_pos
        } else {
            ist_pos[prev]
        };
    }
    unsafe {
        ffi::L3_stereo_process(
            left.as_mut_ptr(),
            ist_pos.as_mut_ptr(),
            table.as_ptr(),
            hdr.as_ptr(),
            max_band.as_mut_ptr(),
            scalefac_next,
        );
    }
}

pub fn huffman(
    dst: &mut [f32],
    bits: &mut Bits,
    gr_info: &GrInfo,
    scf: &[f32],
    layer3gr_limit: i32,
) {
    let tabs: [i16; 2164] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 785, 785, 785, 785, 784, 784, 784, 784, 513, 513, 513, 513, 513, 513, 513, 513, 256,
        256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, -255, 1313,
        1298, 1282, 785, 785, 785, 785, 784, 784, 784, 784, 769, 769, 769, 769, 256, 256, 256, 256,
        256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 290, 288, -255, 1313, 1298,
        1282, 769, 769, 769, 769, 529, 529, 529, 529, 529, 529, 529, 529, 528, 528, 528, 528, 528,
        528, 528, 528, 512, 512, 512, 512, 512, 512, 512, 512, 290, 288, -253, -318, -351, -367,
        785, 785, 785, 785, 784, 784, 784, 784, 769, 769, 769, 769, 256, 256, 256, 256, 256, 256,
        256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 819, 818, 547, 547, 275, 275, 275, 275,
        561, 560, 515, 546, 289, 274, 288, 258, -254, -287, 1329, 1299, 1314, 1312, 1057, 1057,
        1042, 1042, 1026, 1026, 784, 784, 784, 784, 529, 529, 529, 529, 529, 529, 529, 529, 769,
        769, 769, 769, 768, 768, 768, 768, 563, 560, 306, 306, 291, 259, -252, -413, -477, -542,
        1298, -575, 1041, 1041, 784, 784, 784, 784, 769, 769, 769, 769, 256, 256, 256, 256, 256,
        256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, -383, -399, 1107, 1092, 1106, 1061,
        849, 849, 789, 789, 1104, 1091, 773, 773, 1076, 1075, 341, 340, 325, 309, 834, 804, 577,
        577, 532, 532, 516, 516, 832, 818, 803, 816, 561, 561, 531, 531, 515, 546, 289, 289, 288,
        258, -252, -429, -493, -559, 1057, 1057, 1042, 1042, 529, 529, 529, 529, 529, 529, 529,
        529, 784, 784, 784, 784, 769, 769, 769, 769, 512, 512, 512, 512, 512, 512, 512, 512, -382,
        1077, -415, 1106, 1061, 1104, 849, 849, 789, 789, 1091, 1076, 1029, 1075, 834, 834, 597,
        581, 340, 340, 339, 324, 804, 833, 532, 532, 832, 772, 818, 803, 817, 787, 816, 771, 290,
        290, 290, 290, 288, 258, -253, -349, -414, -447, -463, 1329, 1299, -479, 1314, 1312, 1057,
        1057, 1042, 1042, 1026, 1026, 785, 785, 785, 785, 784, 784, 784, 784, 769, 769, 769, 769,
        768, 768, 768, 768, -319, 851, 821, -335, 836, 850, 805, 849, 341, 340, 325, 336, 533, 533,
        579, 579, 564, 564, 773, 832, 578, 548, 563, 516, 321, 276, 306, 291, 304, 259, -251, -572,
        -733, -830, -863, -879, 1041, 1041, 784, 784, 784, 784, 769, 769, 769, 769, 256, 256, 256,
        256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, -511, -527, -543, 1396,
        1351, 1381, 1366, 1395, 1335, 1380, -559, 1334, 1138, 1138, 1063, 1063, 1350, 1392, 1031,
        1031, 1062, 1062, 1364, 1363, 1120, 1120, 1333, 1348, 881, 881, 881, 881, 375, 374, 359,
        373, 343, 358, 341, 325, 791, 791, 1123, 1122, -703, 1105, 1045, -719, 865, 865, 790, 790,
        774, 774, 1104, 1029, 338, 293, 323, 308, -799, -815, 833, 788, 772, 818, 803, 816, 322,
        292, 307, 320, 561, 531, 515, 546, 289, 274, 288, 258, -251, -525, -605, -685, -765, -831,
        -846, 1298, 1057, 1057, 1312, 1282, 785, 785, 785, 785, 784, 784, 784, 784, 769, 769, 769,
        769, 512, 512, 512, 512, 512, 512, 512, 512, 1399, 1398, 1383, 1367, 1382, 1396, 1351,
        -511, 1381, 1366, 1139, 1139, 1079, 1079, 1124, 1124, 1364, 1349, 1363, 1333, 882, 882,
        882, 882, 807, 807, 807, 807, 1094, 1094, 1136, 1136, 373, 341, 535, 535, 881, 775, 867,
        822, 774, -591, 324, 338, -671, 849, 550, 550, 866, 864, 609, 609, 293, 336, 534, 534, 789,
        835, 773, -751, 834, 804, 308, 307, 833, 788, 832, 772, 562, 562, 547, 547, 305, 275, 560,
        515, 290, 290, -252, -397, -477, -557, -622, -653, -719, -735, -750, 1329, 1299, 1314,
        1057, 1057, 1042, 1042, 1312, 1282, 1024, 1024, 785, 785, 785, 785, 784, 784, 784, 784,
        769, 769, 769, 769, -383, 1127, 1141, 1111, 1126, 1140, 1095, 1110, 869, 869, 883, 883,
        1079, 1109, 882, 882, 375, 374, 807, 868, 838, 881, 791, -463, 867, 822, 368, 263, 852,
        837, 836, -543, 610, 610, 550, 550, 352, 336, 534, 534, 865, 774, 851, 821, 850, 805, 593,
        533, 579, 564, 773, 832, 578, 578, 548, 548, 577, 577, 307, 276, 306, 291, 516, 560, 259,
        259, -250, -2107, -2507, -2764, -2909, -2974, -3007, -3023, 1041, 1041, 1040, 1040, 769,
        769, 769, 769, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
        256, -767, -1052, -1213, -1277, -1358, -1405, -1469, -1535, -1550, -1582, -1614, -1647,
        -1662, -1694, -1726, -1759, -1774, -1807, -1822, -1854, -1886, 1565, -1919, -1935, -1951,
        -1967, 1731, 1730, 1580, 1717, -1983, 1729, 1564, -1999, 1548, -2015, -2031, 1715, 1595,
        -2047, 1714, -2063, 1610, -2079, 1609, -2095, 1323, 1323, 1457, 1457, 1307, 1307, 1712,
        1547, 1641, 1700, 1699, 1594, 1685, 1625, 1442, 1442, 1322, 1322, -780, -973, -910, 1279,
        1278, 1277, 1262, 1276, 1261, 1275, 1215, 1260, 1229, -959, 974, 974, 989, 989, -943, 735,
        478, 478, 495, 463, 506, 414, -1039, 1003, 958, 1017, 927, 942, 987, 957, 431, 476, 1272,
        1167, 1228, -1183, 1256, -1199, 895, 895, 941, 941, 1242, 1227, 1212, 1135, 1014, 1014,
        490, 489, 503, 487, 910, 1013, 985, 925, 863, 894, 970, 955, 1012, 847, -1343, 831, 755,
        755, 984, 909, 428, 366, 754, 559, -1391, 752, 486, 457, 924, 997, 698, 698, 983, 893, 740,
        740, 908, 877, 739, 739, 667, 667, 953, 938, 497, 287, 271, 271, 683, 606, 590, 712, 726,
        574, 302, 302, 738, 736, 481, 286, 526, 725, 605, 711, 636, 724, 696, 651, 589, 681, 666,
        710, 364, 467, 573, 695, 466, 466, 301, 465, 379, 379, 709, 604, 665, 679, 316, 316, 634,
        633, 436, 436, 464, 269, 424, 394, 452, 332, 438, 363, 347, 408, 393, 448, 331, 422, 362,
        407, 392, 421, 346, 406, 391, 376, 375, 359, 1441, 1306, -2367, 1290, -2383, 1337, -2399,
        -2415, 1426, 1321, -2431, 1411, 1336, -2447, -2463, -2479, 1169, 1169, 1049, 1049, 1424,
        1289, 1412, 1352, 1319, -2495, 1154, 1154, 1064, 1064, 1153, 1153, 416, 390, 360, 404, 403,
        389, 344, 374, 373, 343, 358, 372, 327, 357, 342, 311, 356, 326, 1395, 1394, 1137, 1137,
        1047, 1047, 1365, 1392, 1287, 1379, 1334, 1364, 1349, 1378, 1318, 1363, 792, 792, 792, 792,
        1152, 1152, 1032, 1032, 1121, 1121, 1046, 1046, 1120, 1120, 1030, 1030, -2895, 1106, 1061,
        1104, 849, 849, 789, 789, 1091, 1076, 1029, 1090, 1060, 1075, 833, 833, 309, 324, 532, 532,
        832, 772, 818, 803, 561, 561, 531, 560, 515, 546, 289, 274, 288, 258, -250, -1179, -1579,
        -1836, -1996, -2124, -2253, -2333, -2413, -2477, -2542, -2574, -2607, -2622, -2655, 1314,
        1313, 1298, 1312, 1282, 785, 785, 785, 785, 1040, 1040, 1025, 1025, 768, 768, 768, 768,
        -766, -798, -830, -862, -895, -911, -927, -943, -959, -975, -991, -1007, -1023, -1039,
        -1055, -1070, 1724, 1647, -1103, -1119, 1631, 1767, 1662, 1738, 1708, 1723, -1135, 1780,
        1615, 1779, 1599, 1677, 1646, 1778, 1583, -1151, 1777, 1567, 1737, 1692, 1765, 1722, 1707,
        1630, 1751, 1661, 1764, 1614, 1736, 1676, 1763, 1750, 1645, 1598, 1721, 1691, 1762, 1706,
        1582, 1761, 1566, -1167, 1749, 1629, 767, 766, 751, 765, 494, 494, 735, 764, 719, 749, 734,
        763, 447, 447, 748, 718, 477, 506, 431, 491, 446, 476, 461, 505, 415, 430, 475, 445, 504,
        399, 460, 489, 414, 503, 383, 474, 429, 459, 502, 502, 746, 752, 488, 398, 501, 473, 413,
        472, 486, 271, 480, 270, -1439, -1455, 1357, -1471, -1487, -1503, 1341, 1325, -1519, 1489,
        1463, 1403, 1309, -1535, 1372, 1448, 1418, 1476, 1356, 1462, 1387, -1551, 1475, 1340, 1447,
        1402, 1386, -1567, 1068, 1068, 1474, 1461, 455, 380, 468, 440, 395, 425, 410, 454, 364,
        467, 466, 464, 453, 269, 409, 448, 268, 432, 1371, 1473, 1432, 1417, 1308, 1460, 1355,
        1446, 1459, 1431, 1083, 1083, 1401, 1416, 1458, 1445, 1067, 1067, 1370, 1457, 1051, 1051,
        1291, 1430, 1385, 1444, 1354, 1415, 1400, 1443, 1082, 1082, 1173, 1113, 1186, 1066, 1185,
        1050, -1967, 1158, 1128, 1172, 1097, 1171, 1081, -1983, 1157, 1112, 416, 266, 375, 400,
        1170, 1142, 1127, 1065, 793, 793, 1169, 1033, 1156, 1096, 1141, 1111, 1155, 1080, 1126,
        1140, 898, 898, 808, 808, 897, 897, 792, 792, 1095, 1152, 1032, 1125, 1110, 1139, 1079,
        1124, 882, 807, 838, 881, 853, 791, -2319, 867, 368, 263, 822, 852, 837, 866, 806, 865,
        -2399, 851, 352, 262, 534, 534, 821, 836, 594, 594, 549, 549, 593, 593, 533, 533, 848, 773,
        579, 579, 564, 578, 548, 563, 276, 276, 577, 576, 306, 291, 516, 560, 305, 305, 275, 259,
        -251, -892, -2058, -2620, -2828, -2957, -3023, -3039, 1041, 1041, 1040, 1040, 769, 769,
        769, 769, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
        -511, -527, -543, -559, 1530, -575, -591, 1528, 1527, 1407, 1526, 1391, 1023, 1023, 1023,
        1023, 1525, 1375, 1268, 1268, 1103, 1103, 1087, 1087, 1039, 1039, 1523, -604, 815, 815,
        815, 815, 510, 495, 509, 479, 508, 463, 507, 447, 431, 505, 415, 399, -734, -782, 1262,
        -815, 1259, 1244, -831, 1258, 1228, -847, -863, 1196, -879, 1253, 987, 987, 748, -767, 493,
        493, 462, 477, 414, 414, 686, 669, 478, 446, 461, 445, 474, 429, 487, 458, 412, 471, 1266,
        1264, 1009, 1009, 799, 799, -1019, -1276, -1452, -1581, -1677, -1757, -1821, -1886, -1933,
        -1997, 1257, 1257, 1483, 1468, 1512, 1422, 1497, 1406, 1467, 1496, 1421, 1510, 1134, 1134,
        1225, 1225, 1466, 1451, 1374, 1405, 1252, 1252, 1358, 1480, 1164, 1164, 1251, 1251, 1238,
        1238, 1389, 1465, -1407, 1054, 1101, -1423, 1207, -1439, 830, 830, 1248, 1038, 1237, 1117,
        1223, 1148, 1236, 1208, 411, 426, 395, 410, 379, 269, 1193, 1222, 1132, 1235, 1221, 1116,
        976, 976, 1192, 1162, 1177, 1220, 1131, 1191, 963, 963, -1647, 961, 780, -1663, 558, 558,
        994, 993, 437, 408, 393, 407, 829, 978, 813, 797, 947, -1743, 721, 721, 377, 392, 844, 950,
        828, 890, 706, 706, 812, 859, 796, 960, 948, 843, 934, 874, 571, 571, -1919, 690, 555, 689,
        421, 346, 539, 539, 944, 779, 918, 873, 932, 842, 903, 888, 570, 570, 931, 917, 674, 674,
        -2575, 1562, -2591, 1609, -2607, 1654, 1322, 1322, 1441, 1441, 1696, 1546, 1683, 1593,
        1669, 1624, 1426, 1426, 1321, 1321, 1639, 1680, 1425, 1425, 1305, 1305, 1545, 1668, 1608,
        1623, 1667, 1592, 1638, 1666, 1320, 1320, 1652, 1607, 1409, 1409, 1304, 1304, 1288, 1288,
        1664, 1637, 1395, 1395, 1335, 1335, 1622, 1636, 1394, 1394, 1319, 1319, 1606, 1621, 1392,
        1392, 1137, 1137, 1137, 1137, 345, 390, 360, 375, 404, 373, 1047, -2751, -2767, -2783,
        1062, 1121, 1046, -2799, 1077, -2815, 1106, 1061, 789, 789, 1105, 1104, 263, 355, 310, 340,
        325, 354, 352, 262, 339, 324, 1091, 1076, 1029, 1090, 1060, 1075, 833, 833, 788, 788, 1088,
        1028, 818, 818, 803, 803, 561, 561, 531, 531, 816, 771, 546, 546, 289, 274, 288, 258, -253,
        -317, -381, -446, -478, -509, 1279, 1279, -811, -1179, -1451, -1756, -1900, -2028, -2189,
        -2253, -2333, -2414, -2445, -2511, -2526, 1313, 1298, -2559, 1041, 1041, 1040, 1040, 1025,
        1025, 1024, 1024, 1022, 1007, 1021, 991, 1020, 975, 1019, 959, 687, 687, 1018, 1017, 671,
        671, 655, 655, 1016, 1015, 639, 639, 758, 758, 623, 623, 757, 607, 756, 591, 755, 575, 754,
        559, 543, 543, 1009, 783, -575, -621, -685, -749, 496, -590, 750, 749, 734, 748, 974, 989,
        1003, 958, 988, 973, 1002, 942, 987, 957, 972, 1001, 926, 986, 941, 971, 956, 1000, 910,
        985, 925, 999, 894, 970, -1071, -1087, -1102, 1390, -1135, 1436, 1509, 1451, 1374, -1151,
        1405, 1358, 1480, 1420, -1167, 1507, 1494, 1389, 1342, 1465, 1435, 1450, 1326, 1505, 1310,
        1493, 1373, 1479, 1404, 1492, 1464, 1419, 428, 443, 472, 397, 736, 526, 464, 464, 486, 457,
        442, 471, 484, 482, 1357, 1449, 1434, 1478, 1388, 1491, 1341, 1490, 1325, 1489, 1463, 1403,
        1309, 1477, 1372, 1448, 1418, 1433, 1476, 1356, 1462, 1387, -1439, 1475, 1340, 1447, 1402,
        1474, 1324, 1461, 1371, 1473, 269, 448, 1432, 1417, 1308, 1460, -1711, 1459, -1727, 1441,
        1099, 1099, 1446, 1386, 1431, 1401, -1743, 1289, 1083, 1083, 1160, 1160, 1458, 1445, 1067,
        1067, 1370, 1457, 1307, 1430, 1129, 1129, 1098, 1098, 268, 432, 267, 416, 266, 400, -1887,
        1144, 1187, 1082, 1173, 1113, 1186, 1066, 1050, 1158, 1128, 1143, 1172, 1097, 1171, 1081,
        420, 391, 1157, 1112, 1170, 1142, 1127, 1065, 1169, 1049, 1156, 1096, 1141, 1111, 1155,
        1080, 1126, 1154, 1064, 1153, 1140, 1095, 1048, -2159, 1125, 1110, 1137, -2175, 823, 823,
        1139, 1138, 807, 807, 384, 264, 368, 263, 868, 838, 853, 791, 867, 822, 852, 837, 866, 806,
        865, 790, -2319, 851, 821, 836, 352, 262, 850, 805, 849, -2399, 533, 533, 835, 820, 336,
        261, 578, 548, 563, 577, 532, 532, 832, 772, 562, 562, 547, 547, 305, 275, 560, 515, 290,
        290, 288, 258,
    ];
    let tab32: [u8; 28] = [
        130, 162, 193, 209, 44, 28, 76, 140, 9, 9, 9, 9, 9, 9, 9, 9, 190, 254, 222, 238, 126, 94,
        157, 157, 109, 61, 173, 205,
    ];
    let tab33: [u8; 16] = [
        252, 236, 220, 204, 188, 172, 156, 140, 124, 108, 92, 76, 60, 44, 28, 12,
    ];
    let tabindex = [
        0, 32, 64, 98, 0, 132, 180, 218, 292, 364, 426, 538, 648, 746, 0, 1126, 1460, 1460, 1460,
        1460, 1460, 1460, 1460, 1460, 1842, 1842, 1842, 1842, 1842, 1842, 1842, 1842,
    ];
    let g_linbits: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 8, 10, 13, 4, 5, 6, 7, 8, 9,
        11, 13,
    ];

    fn peek_bits(bs_cache: u32, n: i32) -> u32 {
        bs_cache >> (32 - n)
    }

    fn flush_bits(bs_cache: &mut u32, bs_sh: &mut i32, n: i32) {
        *bs_cache <<= n;
        *bs_sh += n;
    }

    fn bspos(bs_next: usize, bs_sh: i32) -> f64 {
        bs_next as f64 * 8.0 - 24.0 + f64::from(bs_sh)
    }

    fn check_bits(bs: &Bits, bs_next: &mut usize, bs_sh: &mut i32, bs_cache: &mut u32) {
        while *bs_sh >= 0 {
            let byte = u32::from(bs.data[*bs_next]);
            *bs_next += 1;
            *bs_cache |= byte << *bs_sh;
            *bs_sh -= 8
        }
    }

    let mut one = 0.0;
    let mut ireg = 0;
    let mut big_val_cnt: i32 = gr_info.big_values.into();
    let mut sfb = 0;
    let mut bs_next = bits.position / 8;
    let mut scf_next = 0;
    let mut bs_cache = bits
        .data
        .iter()
        .skip(bs_next)
        .take(4)
        .fold(0, |acc, &byte| (acc * 256 + u32::from(byte)))
        << (bits.position & 7);
    let mut bs_sh = (bits.position as i32 & 7) - 8;
    bs_next += 4;
    let mut dst_pos = 0;
    let table = match gr_info.sfb_table {
        Some(SFBTable::Long(tab)) => tab,
        Some(SFBTable::Short(tab)) => tab,
        Some(SFBTable::Mixed(tab, _)) => tab,
        None => &[],
    };
    while big_val_cnt > 0 {
        let tab_num = gr_info.table_select[ireg] as usize;
        let mut sfb_cnt: i32 = gr_info.region_count[ireg].into();
        ireg += 1;
        let codebook = &tabs[tabindex[tab_num]..];
        let linbits: i32 = g_linbits[tab_num].into();
        loop {
            let np: i32 = (table[sfb as usize] / 2).into();
            sfb += 1;
            let mut pairs_to_decode = cmp::min(big_val_cnt, np);
            one = scf[scf_next];
            scf_next += 1;
            loop {
                let mut w: i32 = 5;
                let mut leaf: i32 = codebook[peek_bits(bs_cache, w) as usize].into();
                while leaf < 0 {
                    flush_bits(&mut bs_cache, &mut bs_sh, w);
                    w = leaf & 7;
                    leaf = codebook
                        [peek_bits(bs_cache, w).wrapping_sub((leaf >> 3) as u32) as usize]
                        .into()
                }
                flush_bits(&mut bs_cache, &mut bs_sh, leaf >> 8);
                for _ in 0..2 {
                    let mut lsb: i32 = leaf & 0xf;
                    if lsb == 15 && 0 != linbits {
                        lsb = (lsb as u32).wrapping_add(peek_bits(bs_cache, linbits)) as i32;
                        flush_bits(&mut bs_cache, &mut bs_sh, linbits);
                        check_bits(bits, &mut bs_next, &mut bs_sh, &mut bs_cache);
                        dst[dst_pos] = one
                            * unsafe { ffi::L3_pow_43(lsb) }
                            * if bs_cache > i32::max_value() as u32 {
                                -1.0
                            } else {
                                1.0
                            }
                    } else {
                        dst[dst_pos] = G_POW43[((16 + lsb) as u32)
                            .wrapping_sub((16 as u32).wrapping_mul(bs_cache >> 31))
                            as usize] as f32
                            * one
                    }
                    if 0 != lsb {
                        flush_bits(&mut bs_cache, &mut bs_sh, 1);
                    } else {
                        flush_bits(&mut bs_cache, &mut bs_sh, 0);
                    };

                    dst_pos += 1;
                    leaf >>= 4
                }
                check_bits(bits, &mut bs_next, &mut bs_sh, &mut bs_cache);
                pairs_to_decode -= 1;
                if 0 == pairs_to_decode {
                    break;
                }
            }
            big_val_cnt -= np;
            if !(big_val_cnt > 0 && {
                sfb_cnt -= 1;
                sfb_cnt >= 0
            }) {
                break;
            }
        }
    }
    let mut np = 1 - big_val_cnt;
    let mut reload_scalefactor = |one: &mut f32| {
        np -= 1;
        if 0 == np {
            let sfbtab = table[sfb as usize];
            sfb += 1;
            np = (sfbtab / 2).into();
            if 0 == np {
                return true;
            }
            let scf_val = scf[scf_next];
            scf_next += 1;
            *one = scf_val
        }
        false
    };
    let codebook_count1 = if 0 != gr_info.count1_table {
        &tab33 as &[u8]
    } else {
        &tab32
    };
    loop {
        let mut leaf_0: i32 = codebook_count1[peek_bits(bs_cache, 4) as usize].into();
        if 0 == leaf_0 & 8 {
            leaf_0 = codebook_count1
                [((leaf_0 >> 3) as u32).wrapping_add(bs_cache << 4 >> (32 - (leaf_0 & 3))) as usize]
                .into()
        }
        flush_bits(&mut bs_cache, &mut bs_sh, leaf_0 & 7);
        if bspos(bs_next, bs_sh) > layer3gr_limit.into() {
            break;
        }
        let mut deq_count1 = |one: f32, num: usize| {
            if 0 != leaf_0 & 128 >> num {
                dst[dst_pos + num] = if bs_cache > i32::max_value() as u32 {
                    -one
                } else {
                    one
                };
                flush_bits(&mut bs_cache, &mut bs_sh, 1);
            }
        };
        if reload_scalefactor(&mut one) {
            break;
        }
        deq_count1(one, 0);
        deq_count1(one, 1);
        if reload_scalefactor(&mut one) {
            break;
        }
        deq_count1(one, 2);
        deq_count1(one, 3);
        check_bits(bits, &mut bs_next, &mut bs_sh, &mut bs_cache);
        dst_pos += 4;
    }
    bits.position = layer3gr_limit as usize;
}
