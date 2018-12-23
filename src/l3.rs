use core::cmp;
use core::mem;

use crate::bits::Bits;
use crate::{decoder, ffi, header};
use crate::{MAX_BITRESERVOIR_BYTES, SHORT_BLOCK_TYPE};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
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
    pub preflag: u8,
    pub scalefac_scale: u8,
    pub count1_table: u8,
    pub scfsi: u8,
}

// contains the table and describes the length
#[derive(Copy, Clone, PartialEq, Debug)]
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
        gr.preflag = self.preflag;
        gr.scalefac_scale = self.scalefac_scale;
        gr.count1_table = self.count1_table;
        gr.scfsi = self.scfsi;
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
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
#[cfg_attr(rustfmt, rustfmt_skip)]
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

#[cfg_attr(rustfmt, rustfmt_skip)]
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
        part_23_sum += gr.part_23_length as i32;
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
            bs.get_bits(1) as _
        } else {
            (gr.scalefac_compress >= 500) as _
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
    return decoder.reserv >= main_data_begin as i32;
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

pub unsafe fn decode(
    decoder: &mut ffi::mp3dec_t,
    scratch: &mut decoder::Scratch,
    native_gr_info: &mut [GrInfo],
    channel_num: usize,
) {
    let mut gr_info: [ffi::L3_gr_info_t; 4] = mem::zeroed();
    for (native, ffi) in native_gr_info.into_iter().zip(&mut gr_info) {
        native.apply_to_ffi(ffi);
    }
    let gr_info = &mut gr_info[..native_gr_info.len()];
    for channel in 0..channel_num {
        let ist_pos = &mut scratch.ist_pos;
        let scf = &mut scratch.scf;
        let grbuf = &mut scratch.grbuf;
        let layer3gr_limit =
            scratch.bits.position as libc::c_int + gr_info[channel].part_23_length as libc::c_int;
        scratch.bits.with_bits(|bs| {
            let mut copy = bs.bs_copy();
            ffi::L3_decode_scalefactors(
                decoder.header.as_mut_ptr(),
                ist_pos[channel].as_mut_ptr(),
                &mut copy,
                gr_info[channel..].as_mut_ptr(),
                scf.as_mut_ptr(),
                channel as _,
            );
            ffi::L3_huffman(
                grbuf[channel].as_mut_ptr(),
                &mut copy,
                gr_info[channel..].as_mut_ptr(),
                scf.as_mut_ptr(),
                layer3gr_limit,
            );
            bs.position = copy.pos as _;
        });
    }

    if header::test_1_stereo(&decoder.header) {
        ffi::L3_intensity_stereo(
            scratch.grbuf[0].as_mut_ptr(),
            scratch.ist_pos[1].as_mut_ptr(),
            gr_info.as_mut_ptr(),
            decoder.header.as_mut_ptr(),
        );
    } else if header::is_ms_stereo(&decoder.header) {
        ffi::L3_midside_stereo(scratch.grbuf[0].as_mut_ptr(), 576);
    }

    for channel in 0..channel_num {
        let gr_info = gr_info[channel];
        let mut aa_bands = 31;
        let n_long_bands = if 0 != gr_info.mixed_block_flag { 2 } else { 0 }
            << (header::get_my_sample_rate(&decoder.header) == 2) as libc::c_int;
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
            gr_info.block_type as libc::c_uint,
            n_long_bands as libc::c_uint,
        );
        ffi::L3_change_sign(scratch.grbuf[channel].as_mut_ptr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::{mem, slice};
    use quickcheck::quickcheck;
    use std::vec::Vec;

    // used instead of PartialEq to know what part was not equal
    fn compare_gr_info(this: &ffi::L3_gr_info_t, other: &ffi::L3_gr_info_t) {
        // can be between 23 and 40, figure out how that is figured out
        let this_sfbtab = unsafe { slice::from_raw_parts(this.sfbtab, 23) };
        let other_sfbtab = unsafe { slice::from_raw_parts(other.sfbtab, 23) };
        assert_eq!(this_sfbtab, other_sfbtab, "sfbtab");
        assert_eq!(this.part_23_length, other.part_23_length, "part_23_length");
        assert_eq!(this.big_values, other.big_values, "big_values");
        assert_eq!(
            this.scalefac_compress, other.scalefac_compress,
            "scalefac_compress"
        );
        assert_eq!(this.global_gain, other.global_gain, "global_gain");
        assert_eq!(this.block_type, other.block_type, "block_type");
        assert_eq!(
            this.mixed_block_flag, other.mixed_block_flag,
            "mixed_block_flag"
        );
        assert_eq!(this.n_long_sfb, other.n_long_sfb, "n_long_sfb");
        assert_eq!(this.n_short_sfb, other.n_short_sfb, "n_short_sfb");
        assert_eq!(this.table_select.as_ref(), other.table_select.as_ref());
        assert_eq!(this.region_count.as_ref(), other.region_count.as_ref());
        assert_eq!(this.subblock_gain.as_ref(), other.subblock_gain.as_ref());
        assert_eq!(this.preflag, other.preflag, "preflag");
        assert_eq!(this.scalefac_scale, other.scalefac_scale, "scalefac_scale");
        assert_eq!(this.count1_table, other.count1_table, "count1_table");
        assert_eq!(this.scfsi, other.scfsi, "scfsi");
    }

    quickcheck! {
        fn test_read_side_info(data: Vec<u8>, hdr: header::ValidHeader) -> bool {
            let mut native_bs = Bits::new(&data);
            let mut ffi_bs = unsafe { native_bs.bs_copy() };

            let mut native_gr_info = [GrInfo::default(); 4];
            let mut ffi_gr_info: [ffi::L3_gr_info_t; 4] = unsafe { mem::zeroed() };
            for (native, ffi) in native_gr_info.iter().zip(&mut ffi_gr_info) {
                native.apply_to_ffi(ffi);
            }

            let native_res = read_side_info(&mut native_bs, &mut native_gr_info, &hdr.0);
            let ffi_res = unsafe {
                ffi::L3_read_side_info(&mut ffi_bs, ffi_gr_info.as_mut_ptr(), hdr.0.as_ptr())
            };

            assert_eq!(native_res, ffi_res);
            assert_eq!(native_bs.position as i32, ffi_bs.pos);
            native_gr_info.iter().enumerate().for_each(|(i, native)| {
                let mut info = unsafe { mem::zeroed() };
                native.apply_to_ffi(&mut info);
                compare_gr_info(&info, &ffi_gr_info[i]);
            });
            true
        }
    }

    quickcheck! {
        fn test_restore_reservoir(decoder: ffi::mp3dec_t, data: Vec<u8>, bits: decoder::BitsProxy, main_data_begin: u32) -> bool {
            let mut native_bs = Bits::new(&data);
            let mut ffi_bs = unsafe { native_bs.bs_copy() };

            let mut native_decoder = decoder;
            // reserv musn't be negetive, causes stack overflow in the c version
            native_decoder.reserv = native_decoder.reserv.abs();
            let mut ffi_decoder = native_decoder;

            let mut ffi_scratch = unsafe { mem::zeroed() };

            let mut native_scratch = decoder::Scratch::default();
            native_scratch.bits = bits;
            native_scratch.convert_to_ffi(&mut ffi_scratch);

            let ffi_res = unsafe { ffi::L3_restore_reservoir(&mut ffi_decoder, &mut ffi_bs, &mut ffi_scratch, main_data_begin as _) };
            let native_res = restore_reservoir(&mut native_decoder, &mut native_bs, &mut native_scratch.bits, main_data_begin as _);

            assert_eq!(ffi_res != 0, native_res);
            assert_eq!(ffi_scratch.bs.limit, native_scratch.bits.with_bits(|b| b.limit()) as _);
            assert_eq!(ffi_scratch.bs.pos, native_scratch.bits.position as _);
            assert_eq!(&ffi_scratch.maindata as &[u8], &native_scratch.bits.maindata as &[u8]);
            assert_eq!(native_decoder, ffi_decoder);
            assert_eq!(native_bs.position as i32, ffi_bs.pos);
            true
        }
    }

    quickcheck! {
        fn test_save_reservoir(decoder: ffi::mp3dec_t, bits: decoder::BitsProxy) -> bool {
            let mut native_decoder = decoder;
            let mut ffi_decoder = native_decoder;

            let mut ffi_scratch = unsafe { mem::zeroed() };

            let mut native_scratch = decoder::Scratch::default();
            native_scratch.bits = bits;
            native_scratch.convert_to_ffi(&mut ffi_scratch);

            save_reservoir(&mut native_decoder, &mut native_scratch.bits);
            unsafe { ffi::L3_save_reservoir(&mut ffi_decoder, &mut ffi_scratch) };

            assert_eq!(ffi_scratch.bs.limit, native_scratch.bits.with_bits(|b| b.limit()) as _);
            assert_eq!(ffi_scratch.bs.pos, native_scratch.bits.position as _);
            assert_eq!(&ffi_scratch.maindata as &[u8], &native_scratch.bits.maindata as &[u8]);
            assert_eq!(ffi_decoder, native_decoder);
            true
        }
    }
}
