use bits::Bits;
use SHORT_BLOCK_TYPE;
use {ffi, header};

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

pub fn read_side_info(bs: &mut Bits, gr: &mut [ffi::L3_gr_info_t], hdr: &[u8]) -> i32 {
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
        let gr = &mut gr[i as usize];
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
        gr.sfbtab = G_SCF_LONG[sr_idx as usize].as_ptr();
        gr.n_long_sfb = 22;
        gr.n_short_sfb = 0;

        if bs.get_bits(1) != 0 {
            gr.block_type = bs.get_bits(2) as _;
            if gr.block_type == 0 {
                return -1;
            }

            gr.mixed_block_flag = bs.get_bits(1) as _;
            gr.region_count[0] = 7;
            gr.region_count[1] = 255;
            if gr.block_type == SHORT_BLOCK_TYPE {
                scfsi &= 0x0F0F;

                if gr.mixed_block_flag == 0 {
                    gr.region_count[0] = 8;
                    gr.sfbtab = G_SCF_SHORT[sr_idx as usize].as_ptr();
                    gr.n_long_sfb = 0;
                    gr.n_short_sfb = 39;
                } else {
                    gr.sfbtab = G_SCF_MIXED[sr_idx as usize].as_ptr();
                    gr.n_long_sfb = if header::test_mpeg1(hdr) { 8 } else { 6 };
                    gr.n_short_sfb = 30;
                }
            }

            tables = bs.get_bits(10) as _;
            tables <<= 5;

            for i in 0..3 {
                gr.subblock_gain[i] = bs.get_bits(3) as _
            }
        } else {
            gr.block_type = 0;
            gr.mixed_block_flag = 0;
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

#[cfg(test)]
mod tests {
    use super::*;
    use core::{mem, slice};
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
            let mut native_gr_info: [ffi::L3_gr_info_t; 4] = unsafe { mem::zeroed() };
            let mut ffi_gr_info = native_gr_info;
            let native_res = read_side_info(&mut native_bs, &mut native_gr_info, &hdr.0);
            let ffi_res = unsafe {
                ffi::L3_read_side_info(&mut ffi_bs, ffi_gr_info.as_mut_ptr(), hdr.0.as_ptr())
            };
            assert!(native_res == ffi_res);
            assert!(native_bs.position as i32 == ffi_bs.pos);
            native_gr_info.iter().enumerate().for_each(|(i, native)| compare_gr_info(native, &ffi_gr_info[i]));
            true
        }
    }
}
