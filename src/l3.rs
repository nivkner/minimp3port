use core::cmp;
use core::mem;

use crate::bits::Bits;
use crate::{decoder, ffi, header};
use crate::{BITS_DEQUANTIZER_OUT, MAX_BITRESERVOIR_BYTES, MAX_SCFI, SHORT_BLOCK_TYPE};

#[derive(Copy, Clone, Default)]
pub struct GrInfo {
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

impl GrInfo {
    pub fn apply_to_ffi(&self, gr: &mut ffi::L3_gr_info_t) {
        match self.sfb_table {
            Some(SFBTable::Long(slice)) => {
                gr.sfbtab = slice.as_ptr();
                gr.n_long_sfb = 22;
                gr.n_short_sfb = 0;
            }
            Some(SFBTable::Short(slice)) => {
                gr.sfbtab = slice.as_ptr();
                gr.n_long_sfb = 0;
                gr.n_short_sfb = 39;
            }
            Some(SFBTable::Mixed(slice, extra)) => {
                gr.sfbtab = slice.as_ptr();
                gr.n_long_sfb = extra;
                gr.n_short_sfb = 30;
            }
            None => {
                gr.n_long_sfb = 0;
                gr.n_short_sfb = 0;
            }
        }
        gr.mixed_block_flag = self.mixed_block as u8;
        gr.block_type = self.block_type;
        gr.part_23_length = self.part_23_length;
        gr.big_values = self.big_values;
        gr.scalefac_compress = self.scalefac_compress;
        gr.global_gain = self.global_gain;
        gr.table_select = self.table_select;
        gr.region_count = self.region_count;
        gr.subblock_gain = self.subblock_gain;
        gr.preflag = self.preflag as u8;
        gr.scalefac_scale = self.scalefac_scale;
        gr.count1_table = self.count1_table;
        gr.scfsi = self.scfsi;
    }
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
    gr: &ffi::L3_gr_info_t,
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
    // 0 == long, 1 == mixed, 2 == short
    let scf_partition: &[u8] =
        &g_scf_partitions[(0 != gr.n_short_sfb) as usize + (0 == gr.n_long_sfb) as usize];
    let mut iscf: [u8; 40] = [0; 40];
    let mut bs_copy = unsafe { bs.bs_copy() };
    if header::test_mpeg1(hdr) {
        let g_scfc_decode: [u8; 16] = [0, 1, 2, 3, 12, 5, 6, 7, 9, 10, 11, 13, 14, 15, 18, 19];
        let part = g_scfc_decode[gr.scalefac_compress as usize] as u8;
        let scf_size = [part >> 2, part >> 2, part & 3, part & 3];
        unsafe {
            ffi::L3_read_scalefactors(
                iscf.as_mut_ptr(),
                ist_pos.as_mut_ptr(),
                scf_size.as_ptr(),
                scf_partition.as_ptr(),
                &mut bs_copy,
                gr.scfsi.into(),
            )
        };
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
        unsafe {
            ffi::L3_read_scalefactors(
                iscf.as_mut_ptr(),
                ist_pos.as_mut_ptr(),
                scf_size.as_ptr(),
                scf_partition[k..].as_ptr(),
                &mut bs_copy,
                -16,
            )
        };
    }
    bs.position = bs_copy.pos as _;

    let scf_shift: i32 = i32::from(gr.scalefac_scale) + 1;
    // is Long
    if 0 != gr.n_short_sfb {
        let sh = 3 - scf_shift as u8;
        for i in (0..gr.n_short_sfb).step_by(3) {
            let step = (gr.n_long_sfb + i) as usize;
            iscf[step] += gr.subblock_gain[0] << sh;
            iscf[step + 1] += gr.subblock_gain[1] << sh;
            iscf[step + 2] += gr.subblock_gain[2] << sh;
        }
    } else if 0 != gr.preflag {
        let g_preamp: [u8; 10] = [1, 1, 1, 1, 2, 2, 3, 3, 3, 2];
        for (iscf, preamp) in iscf.iter_mut().skip(11).zip(g_preamp.iter()) {
            *iscf += preamp
        }
    }
    let gain_exp = i32::from(gr.global_gain) + BITS_DEQUANTIZER_OUT * 4
        - 210
        - if header::is_ms_stereo(hdr) { 2 } else { 0 };
    let gain = unsafe { ffi::L3_ldexp_q2((1 << (MAX_SCFI / 4)) as f32, MAX_SCFI - gain_exp) };
    // the length of the scalefactor band, whichever type it is
    for i in 0..(gr.n_long_sfb as usize + gr.n_short_sfb as usize) {
        scf[i] = unsafe { ffi::L3_ldexp_q2(gain, i32::from(iscf[i]) << scf_shift) };
    }
}

pub unsafe fn decode(
    decoder: &mut ffi::mp3dec_t,
    scratch: &mut decoder::Scratch,
    native_gr_info: &[GrInfo],
    channel_num: usize,
) {
    let mut gr_info: [ffi::L3_gr_info_t; 4] = mem::zeroed();
    for (native, ffi) in native_gr_info.iter().zip(&mut gr_info) {
        native.apply_to_ffi(ffi);
    }
    let gr_info = &gr_info[..native_gr_info.len()];
    for (channel, info) in gr_info.iter().enumerate().take(channel_num) {
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
            let mut copy = bs.bs_copy();
            ffi::L3_huffman(
                grbuf[channel].as_mut_ptr(),
                &mut copy,
                info,
                scf.as_ptr(),
                layer3gr_limit,
            );
            bs.position = copy.pos as _;
        });
    }

    if header::test_1_stereo(&decoder.header) {
        ffi::L3_intensity_stereo(
            scratch.grbuf[0].as_mut_ptr(),
            scratch.ist_pos[1].as_mut_ptr(),
            gr_info.as_ptr(),
            decoder.header.as_ptr(),
        );
    } else if header::is_ms_stereo(&decoder.header) {
        ffi::L3_midside_stereo(scratch.grbuf[0].as_mut_ptr(), 576);
    }

    for channel in 0..channel_num {
        let gr_info = gr_info[channel];
        let mut aa_bands = 31;
        let n_long_bands = if 0 != gr_info.mixed_block_flag { 2 } else { 0 }
            << (header::get_my_sample_rate(&decoder.header) == 2) as i32;
        if 0 != gr_info.n_short_sfb {
            aa_bands = n_long_bands - 1;
            ffi::L3_reorder(
                scratch.grbuf[channel]
                    .as_mut_ptr()
                    .offset((n_long_bands * 18) as isize),
                scratch.syn[0].as_mut_ptr(),
                gr_info.sfbtab.offset(gr_info.n_long_sfb as isize),
            );
        }
        ffi::L3_antialias(scratch.grbuf[channel].as_mut_ptr(), aa_bands);
        ffi::L3_imdct_gr(
            scratch.grbuf[channel].as_mut_ptr(),
            decoder.mdct_overlap[channel].as_mut_ptr(),
            gr_info.block_type.into(),
            n_long_bands as u32,
        );
        ffi::L3_change_sign(scratch.grbuf[channel].as_mut_ptr());
    }
}
