#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[allow(unused)]
#[allow(bad_style)]
mod ffi;
mod header;

use std::mem;
use std::ptr;
use std::slice;

#[no_mangle]
pub extern "C" fn mp3dec_init(dec: &mut ffi::mp3dec_t) {
    dec.header[0] = 0
}

#[no_mangle]
pub unsafe extern "C" fn mp3dec_decode_frame(
    dec: &mut ffi::mp3dec_t,
    mp3: *const u8,
    mp3_bytes: ::std::os::raw::c_int,
    pcm: *mut ffi::mp3d_sample_t,
    info: &mut ffi::mp3dec_frame_info_t,
) -> ::std::os::raw::c_int {
    let mp3 = slice::from_raw_parts(mp3, mp3_bytes as _);
    let pcm_slice = if pcm.is_null() {
        None
    } else {
        Some(slice::from_raw_parts_mut(
            pcm,
            MINIMP3_MAX_SAMPLES_PER_FRAME as _,
        ))
    };
    decode_frame(dec, mp3, pcm_slice, info)
}

const HDR_SIZE: i32 = 4;
const MINIMP3_MAX_SAMPLES_PER_FRAME: i32 = 1152 * 2;
const MAX_FREE_FORMAT_FRAME_SIZE: i32 = 2304; // more than ISO spec's
const MAX_FRAME_SYNC_MATCHES: i32 = 10;
const SHORT_BLOCK_TYPE: u8 = 2;

fn mp3d_find_frame(mp3: &[u8], free_format_bytes: &mut i32, ptr_frame_bytes: &mut i32) -> i32 {
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
            && mp3d_match_frame(mp3_view, frame_bytes))
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

fn mp3d_match_frame(hdr: &[u8], frame_bytes: i32) -> bool {
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

fn l3_read_side_info(bs: &mut ffi::bs_t, gr: &mut [ffi::L3_gr_info_t], hdr: &[u8]) -> i32 {
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

    let mut sr_idx = header::get_my_sample_rate(hdr);
    if sr_idx != 0 {
        sr_idx -= 1
    }
    let mut gr_count = if header::is_mono(hdr) { 1 } else { 2 };
    let mut scfsi = 0;

    let main_data_begin = if header::test_mpeg1(hdr) {
        gr_count *= 2;
        let data = get_bits(bs, 9);
        scfsi = get_bits(bs, 7 + gr_count as u32);
        data
    } else {
        get_bits(bs, 8 + gr_count as u32) >> gr_count
    };

    let mut part_23_sum = 0;
    let mut tables: i32;
    let mut scfsi = scfsi as i32;

    for i in 0..gr_count {
        let gr = &mut gr[i as usize];
        if header::is_mono(hdr) {
            scfsi <<= 4;
        }
        gr.part_23_length = get_bits(bs, 12) as _;
        part_23_sum += gr.part_23_length as i32;
        gr.big_values = get_bits(bs, 9) as _;
        if gr.big_values > 288 {
            return -1;
        }

        gr.global_gain = get_bits(bs, 8) as _;
        gr.scalefac_compress = get_bits(bs, if header::test_mpeg1(hdr) { 4 } else { 9 }) as _;
        gr.sfbtab = G_SCF_LONG[sr_idx as usize].as_ptr();
        gr.n_long_sfb = 22;
        gr.n_short_sfb = 0;

        if get_bits(bs, 1) != 0 {
            gr.block_type = get_bits(bs, 2) as _;
            if gr.block_type == 0 {
                return -1;
            }

            gr.mixed_block_flag = get_bits(bs, 1) as _;
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

            tables = get_bits(bs, 10) as _;
            tables <<= 5;

            for i in 0..3 {
                gr.subblock_gain[i] = get_bits(bs, 3) as _
            }
        } else {
            gr.block_type = 0;
            gr.mixed_block_flag = 0;
            tables = get_bits(bs, 15) as _;

            gr.region_count[0] = get_bits(bs, 4) as _;
            gr.region_count[1] = get_bits(bs, 3) as _;
            gr.region_count[2] = 255;
        }

        gr.table_select[0] = (tables >> 10) as _;
        gr.table_select[1] = ((tables >> 5) & 31) as _;
        gr.table_select[2] = (tables & 31) as _;
        gr.preflag = if header::test_mpeg1(hdr) {
            get_bits(bs, 1) as _
        } else {
            (gr.scalefac_compress >= 500) as _
        };

        gr.scalefac_scale = get_bits(bs, 1) as _;
        gr.count1_table = get_bits(bs, 1) as _;
        gr.scfsi = ((scfsi >> 12) & 15) as _;
        scfsi <<= 4;
    }

    if part_23_sum + bs.pos > bs.limit + (main_data_begin as i32) * 8 {
        -1
    } else {
        main_data_begin as _
    }
}

fn get_bits(bs: &mut ffi::bs_t, amount: u32) -> u32 {
    let s = bs.pos & 7;
    let mut idx = bs.pos as usize >> 3;

    bs.pos += amount as i32;
    if bs.pos > bs.limit {
        return 0;
    }

    let bs_slice = unsafe { slice::from_raw_parts(bs.buf, bs.limit as usize / 8) };

    let mut next: u32 = bs_slice[idx] as u32 & (255 >> s);
    idx += 1;
    let mut shl: i32 = amount as i32 + s;

    let mut cache: u32 = 0;
    loop {
        shl -= 8;
        if shl <= 0 {
            break;
        }
        cache |= next << shl;
        next = bs_slice[idx] as u32;
        idx += 1;
    }

    return (cache | (next >> -shl)) as _;
}

fn decode_frame(
    decoder: &mut ffi::mp3dec_t,
    mp3: &[u8],
    pcm: Option<&mut [i16]>,
    info: &mut ffi::mp3dec_frame_info_t,
) -> i32 {
    let mut frame_size = 0;
    if mp3.len() > 4 && decoder.header[0] == 0xff && header::compare(&decoder.header, mp3) {
        frame_size = header::frame_bytes(mp3, decoder.free_format_bytes) + header::padding(mp3);
        if frame_size != mp3.len() as _
            && (frame_size + HDR_SIZE > mp3.len() as i32
                || !header::compare(mp3, &mp3[(frame_size as _)..]))
        {
            frame_size = 0;
        }
    }

    let mut i = 0;
    if frame_size == 0 {
        unsafe { ptr::write_bytes(decoder, 0, 1) }
        i = mp3d_find_frame(mp3, &mut decoder.free_format_bytes, &mut frame_size);
        if frame_size == 0 || i + frame_size > mp3.len() as _ {
            info.frame_bytes = i;
            return 0;
        }
    }

    let hdr = &mp3[(i as _)..];
    decoder.header.copy_from_slice(&hdr[..(HDR_SIZE as _)]);
    info.frame_bytes = i + frame_size;
    info.channels = if header::is_mono(hdr) { 1 } else { 2 };
    info.hz = header::sample_rate_hz(hdr);
    info.layer = (4 - header::get_layer(hdr)) as _;
    info.bitrate_kbps = header::bitrate_kbps(hdr);

    if pcm.is_none() {
        return header::frame_samples(hdr);
    }

    let pcm_view = pcm.unwrap();
    let mut pcm_pos = 0;

    let mut bs_frame = ffi::bs_t {
        buf: hdr[(HDR_SIZE as _)..].as_ptr(),
        pos: 0,
        limit: (frame_size - HDR_SIZE) * 8,
    };
    if header::is_crc(hdr) {
        bs_frame.pos += 16;
    }

    let mut scratch: ffi::mp3dec_scratch_t = unsafe { mem::uninitialized() };
    let mut success = 1;
    if info.layer == 3 {
        let main_data_begin = l3_read_side_info(&mut bs_frame, &mut scratch.gr_info, hdr);
        if main_data_begin < 0 || bs_frame.pos > bs_frame.limit {
            mp3dec_init(decoder);
            return 0;
        }
        success = unsafe {
            ffi::L3_restore_reservoir(decoder, &mut bs_frame, &mut scratch, main_data_begin)
        };
        if success != 0 {
            let count = if header::test_mpeg1(hdr) { 2 } else { 1 };
            for igr in 0..count {
                unsafe {
                    ptr::write_bytes(&mut scratch.grbuf, 0, 1);
                    ffi::L3_decode(
                        decoder,
                        &mut scratch,
                        scratch.gr_info[((igr * info.channels) as _)..].as_mut_ptr(),
                        info.channels,
                    );
                    ffi::mp3d_synth_granule(
                        decoder.qmf_state.as_mut_ptr(),
                        scratch.grbuf[0].as_mut_ptr(),
                        18,
                        info.channels,
                        pcm_view[pcm_pos..].as_mut_ptr(),
                        scratch.syn[0].as_mut_ptr(),
                    );
                };
                pcm_pos += 576 * info.channels as usize;
            }
        }
        unsafe { ffi::L3_save_reservoir(decoder, &mut scratch) };
    } else {
        let mut sci = unsafe {
            let mut sci: ffi::L12_scale_info = mem::uninitialized();
            ffi::L12_read_scale_info(hdr.as_ptr(), &mut bs_frame, &mut sci);
            ptr::write_bytes(&mut scratch.grbuf, 0, 1);
            sci
        };
        let mut i = 0;
        for igr in 0..3 {
            unsafe {
                i += ffi::L12_dequantize_granule(
                    scratch.grbuf[0][(i as _)..].as_mut_ptr(),
                    &mut bs_frame,
                    &mut sci,
                    info.layer | 1,
                );
            }
            if i == 12 {
                i = 0;
                unsafe {
                    ffi::L12_apply_scf_384(
                        &mut sci,
                        sci.scf[(igr as _)..].as_mut_ptr(),
                        scratch.grbuf[0].as_mut_ptr(),
                    );
                    ffi::mp3d_synth_granule(
                        decoder.qmf_state.as_mut_ptr(),
                        scratch.grbuf[0].as_mut_ptr(),
                        12,
                        info.channels,
                        pcm_view[pcm_pos..].as_mut_ptr(),
                        scratch.syn[0].as_mut_ptr(),
                    );
                    ptr::write_bytes(&mut scratch.grbuf, 0, 1);
                }
                pcm_pos += 384 * info.channels as usize;
            }
            if bs_frame.pos > bs_frame.limit {
                mp3dec_init(decoder);
                return 0;
            }
        }
    }
    success * header::frame_samples(&decoder.header)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use std::fmt;

    impl Arbitrary for ffi::mp3dec_t {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let mut mp3dec: ffi::mp3dec_t = unsafe { mem::uninitialized() };
            let mdct_overlap: Vec<_> = (0..(288 * 2)).map(|_| f32::arbitrary(g)).collect();
            mp3dec.mdct_overlap[0].copy_from_slice(&mdct_overlap[..288]);
            mp3dec.mdct_overlap[1].copy_from_slice(&mdct_overlap[288..]);
            let qmf_state: Vec<_> = (0..960).map(|_| f32::arbitrary(g)).collect();
            mp3dec.qmf_state.copy_from_slice(&qmf_state);
            mp3dec.reserv = ::std::os::raw::c_int::arbitrary(g);
            mp3dec.free_format_bytes = ::std::os::raw::c_int::arbitrary(g);
            let header: Vec<_> = (0..4)
                .map(|_| ::std::os::raw::c_uchar::arbitrary(g))
                .collect();
            mp3dec.header.copy_from_slice(&header);
            let reserv_buf: Vec<_> = (0..511)
                .map(|_| ::std::os::raw::c_uchar::arbitrary(g))
                .collect();
            mp3dec.reserv_buf.copy_from_slice(&reserv_buf);
            mp3dec
        }
    }

    impl fmt::Debug for ffi::mp3dec_t {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.debug_struct("mp3dec_t")
                .field(
                    "mdct_overlap",
                    &format!("{:?}", &self.mdct_overlap[0][..10]),
                ).field("qmf_state", &format!("{:?}", &self.qmf_state[..10]))
                .field("reserv", &self.reserv)
                .field("free_format_bytes", &self.free_format_bytes)
                .field("header", &format!("{:?}", &self.header[..]))
                .finish()
        }
    }

    impl PartialEq for ffi::mp3dec_t {
        fn eq(&self, other: &Self) -> bool {
            self.mdct_overlap[0].as_ref() == other.mdct_overlap[0].as_ref()
                && self.mdct_overlap[1].as_ref() == other.mdct_overlap[1].as_ref()
                && self.qmf_state.as_ref() == other.qmf_state.as_ref()
                && self.reserv == other.reserv
                && self.free_format_bytes == other.free_format_bytes
                && self.header.as_ref() == other.header.as_ref()
                && self.reserv_buf.as_ref() == other.reserv_buf.as_ref()
        }
    }

    impl Arbitrary for ffi::mp3dec_frame_info_t {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let mut mp3dec_frame_info: ffi::mp3dec_frame_info_t = unsafe { mem::uninitialized() };
            mp3dec_frame_info.frame_bytes = ::std::os::raw::c_int::arbitrary(g);
            mp3dec_frame_info.channels = ::std::os::raw::c_int::arbitrary(g);
            mp3dec_frame_info.hz = ::std::os::raw::c_int::arbitrary(g);
            mp3dec_frame_info.layer = ::std::os::raw::c_int::arbitrary(g);
            mp3dec_frame_info.bitrate_kbps = ::std::os::raw::c_int::arbitrary(g);
            mp3dec_frame_info
        }
    }

    quickcheck! {
        fn test_mp3d_find_frame(mp3: Vec<u8>, free_format_bytes: i32, ptr_frame_bytes: i32) -> bool {
            let mut native_ffb = free_format_bytes;
            let mut native_pfb = ptr_frame_bytes;
            let mut ffi_ffb = free_format_bytes;
            let mut ffi_pfb = ptr_frame_bytes;

            let native_res = mp3d_find_frame(&mp3, &mut native_ffb, &mut native_pfb);
            let ffi_res = unsafe {
                ffi::mp3d_find_frame(
                    mp3.as_ptr(),
                    mp3.len() as _,
                    &mut ffi_ffb,
                    &mut ffi_pfb
                    )
                };
            native_res == ffi_res &&
                native_ffb == ffi_ffb &&
                native_pfb == ffi_pfb
        }
    }

    impl PartialEq for ffi::bs_t {
        fn eq(&self, other: &Self) -> bool {
            let self_slice = unsafe { slice::from_raw_parts(self.buf, (self.limit / 8) as _) };
            let other_slice = unsafe { slice::from_raw_parts(other.buf, (other.limit / 8) as _) };
            self_slice == other_slice && self.pos == other.pos && self.limit == self.limit
        }
    }

    // used instead of PartialEq to know what part was not equal
    fn compare_l3_gr_info(this: &ffi::L3_gr_info_t, other: &ffi::L3_gr_info_t) {
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
        fn test_mp3d_match_frame(hdr: header::ValidHeader, data: Vec<u8>, frame_bytes: u32) -> bool {
            let mp3: Vec<u8> = hdr.0.iter().chain(data.iter()).map(|n| *n).collect();
            mp3d_match_frame(&mp3, frame_bytes as _) == unsafe {
                ffi::mp3d_match_frame(mp3.as_ptr(), mp3.len() as _, frame_bytes as _) != 0
            }
        }
    }

    quickcheck! {
        fn test_get_bits(data: Vec<u8>, position: u32, amount: u32) -> bool {
            let amount = amount % 32; // asking for more than 32
            // will cause undefined behavior in the c version
            let mut native_bs = ffi::bs_t {
                buf: data.as_ptr(),
                pos: position as i32,
                limit: data.len() as i32 * 8,
            };
            let mut ffi_bs = native_bs;
            get_bits(&mut native_bs, amount) == unsafe {
                ffi::get_bits(&mut ffi_bs, amount as _)
            }
        }
    }

    quickcheck! {
        fn test_l3_read_side_info(data: Vec<u8>, hdr: header::ValidHeader) -> bool {
            let mut native_bs = ffi::bs_t {
                buf: data.as_ptr(),
                pos: 0,
                limit: (data.len() * 8) as _,
            };
            let mut ffi_bs = native_bs;
            let mut native_gr_info: [ffi::L3_gr_info_t; 4] = unsafe { mem::zeroed() };
            let mut ffi_gr_info = native_gr_info;
            let native_res = l3_read_side_info(&mut native_bs, &mut native_gr_info, &hdr.0);
            let ffi_res = unsafe {
                ffi::L3_read_side_info(&mut ffi_bs, ffi_gr_info.as_mut_ptr(), hdr.0.as_ptr())
            };
            assert!(native_res == ffi_res);
            assert!(native_bs == ffi_bs);
            native_gr_info.iter().enumerate().for_each(|(i, native)| compare_l3_gr_info(native, &ffi_gr_info[i]));
            true
        }
    }

    quickcheck! {
        fn test_decode_frame(decoder: ffi::mp3dec_t, mp3: Vec<u8>, pcm: Option<()>, info: ffi::mp3dec_frame_info_t) -> bool {
            let mut native_decoder = decoder;
            let mut ffi_decoder = decoder;
            let mut native_pcm = [0; MINIMP3_MAX_SAMPLES_PER_FRAME as usize];
            let mut ffi_pcm = [0; MINIMP3_MAX_SAMPLES_PER_FRAME as usize];
            let mut native_info = info;
            let mut ffi_info = info;
            let native_res = decode_frame(
                &mut native_decoder,
                &mp3,
                pcm.map(|_| native_pcm.as_mut()),
                &mut native_info
                );
            let ffi_res = unsafe {
                ffi::__mp3dec_decode_frame(
                    &mut ffi_decoder,
                    mp3.as_ptr(),
                    mp3.len() as _,
                    pcm.map(|_| ffi_pcm.as_mut_ptr())
                    .unwrap_or_else(ptr::null_mut),
                    &mut ffi_info
                    )
            };
            native_res == ffi_res &&
                native_pcm.as_ref() == ffi_pcm.as_ref() &&
                native_info == ffi_info &&
                native_decoder == ffi_decoder
        }
    }
}
