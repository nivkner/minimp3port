#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[allow(unused)]
#[allow(bad_style)]
mod ffi;

use std::mem;
use std::ptr;
use std::slice;

#[no_mangle]
pub extern "C" fn mp3dec_init(dec: *mut ffi::mp3dec_t) {
    unsafe { (*dec).header[0] = 0 }
}

#[no_mangle]
pub extern "C" fn mp3dec_decode_frame(
    dec: &mut ffi::mp3dec_t,
    mp3: *const u8,
    mp3_bytes: ::std::os::raw::c_int,
    pcm: *mut ffi::mp3d_sample_t,
    info: &mut ffi::mp3dec_frame_info_t,
) -> ::std::os::raw::c_int {
    let mp3 = unsafe { slice::from_raw_parts(mp3, mp3_bytes as _) };
    let pcm_slice = unsafe {
        if pcm.is_null() {
            None
        } else {
            Some(slice::from_raw_parts_mut(
                pcm,
                MINIMP3_MAX_SAMPLES_PER_FRAME as _,
            ))
        }
    };
    decode_frame(dec, mp3, pcm_slice, info)
}

const HDR_SIZE: i32 = 4;
const MINIMP3_MAX_SAMPLES_PER_FRAME: i32 = 1152 * 2;
const MAX_FREE_FORMAT_FRAME_SIZE: i32 = 2304; // more than ISO spec's
const MAX_FRAME_SYNC_MATCHES: i32 = 10;

#[inline]
fn hdr_is_mono(hdr: &[u8]) -> bool {
    hdr[3] & 0xC0 == 0xC0
}

#[inline]
fn hdr_is_crc(hdr: &[u8]) -> bool {
    hdr[1] & 1 == 0
}

#[inline]
fn hdr_is_free_format(hdr: &[u8]) -> bool {
    hdr[2] & 0xF0 == 0
}

#[inline]
fn hdr_is_layer_1(hdr: &[u8]) -> bool {
    hdr[1] & 6 == 6
}

#[inline]
fn hdr_is_frame_576(hdr: &[u8]) -> bool {
    hdr[1] & 14 == 2
}

#[inline]
fn hdr_get_layer(hdr: &[u8]) -> u8 {
    hdr[1] >> 1 & 3
}

#[inline]
fn hdr_get_bitrate(hdr: &[u8]) -> u8 {
    hdr[2] >> 4
}

#[inline]
fn hdr_get_sample_rate(hdr: &[u8]) -> u8 {
    hdr[2] >> 2 & 3
}

#[inline]
fn hdr_test_mpeg1(hdr: &[u8]) -> bool {
    hdr[1] & 0x8 != 0
}

#[inline]
fn hdr_test_padding(hdr: &[u8]) -> bool {
    hdr[2] & 0x2 != 0
}

#[inline]
fn hdr_test_not_mpeg25(hdr: &[u8]) -> bool {
    hdr[1] & 0x10 != 0
}

fn hdr_valid(hdr: &[u8]) -> bool {
    hdr[0] == 0xFF
        && ((hdr[1] & 0xF0) == 0xF0 || (hdr[1] & 0xFE) == 0xE2)
        && hdr_get_layer(hdr) != 0
        && hdr_get_bitrate(hdr) != 15
        && hdr_get_sample_rate(hdr) != 3
}

fn hdr_compare(this: &[u8], other: &[u8]) -> bool {
    hdr_valid(other)
        && (this[1] ^ other[1]) & 0xFE == 0
        && (this[2] ^ other[2]) & 0x0C == 0
        && hdr_is_free_format(this) as u8 ^ hdr_is_free_format(other) as u8 == 0
}

fn hdr_frame_bytes(hdr: &[u8], free_format_size: i32) -> i32 {
    let mut frame_bytes =
        hdr_frame_samples(hdr) * hdr_bitrate_kbps(hdr) * 125 / hdr_sample_rate_hz(hdr);
    if hdr_is_layer_1(hdr) {
        frame_bytes &= !3; // slot align
    }
    if frame_bytes != 0 {
        frame_bytes
    } else {
        free_format_size
    }
}

fn hdr_padding(hdr: &[u8]) -> i32 {
    if hdr_test_padding(hdr) {
        if hdr_is_layer_1(hdr) {
            4
        } else {
            1
        }
    } else {
        0
    }
}

fn hdr_frame_samples(hdr: &[u8]) -> i32 {
    if hdr_is_layer_1(hdr) {
        384
    } else {
        1152 >> hdr_is_frame_576(hdr) as u8
    }
}

fn hdr_bitrate_kbps(hdr: &[u8]) -> i32 {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    static HALFRATE: [u8 ; 2 * 3 * 15] = [
        0,4,8,12,16,20,24,28,32,40,48,56,64,72,80,
        0,4,8,12,16,20,24,28,32,40,48,56,64,72,80,
        0,16,24,28,32,40,48,56,64,72,80,88,96,112,128,

        0,16,20,24,28,32,40,48,56,64,80,96,112,128,160,
        0,16,24,28,32,40,48,56,64,80,96,112,128,160,192,
        0,16,32,48,64,80,96,112,128,144,160,176,192,208,224,
    ];
    2 * HALFRATE[(hdr_get_bitrate(hdr)
                     + (hdr_get_layer(hdr) - 1) * 15
                     + hdr_test_mpeg1(hdr) as u8 * 3 * 15) as usize] as i32
}

fn hdr_sample_rate_hz(hdr: &[u8]) -> i32 {
    let g_hz: [i32; 3] = [44100, 48000, 32000];
    g_hz[hdr_get_sample_rate(hdr) as usize]
        >> !hdr_test_mpeg1(hdr) as u8
        >> !hdr_test_not_mpeg25(hdr) as u8
}

fn mp3d_find_frame(mp3: &[u8], free_format_bytes: &mut i32, ptr_frame_bytes: &mut i32) -> i32 {
    let data_size = mp3.len() as i32 - HDR_SIZE;
    let valid_frames = mp3
        .windows(HDR_SIZE as _)
        .enumerate()
        .filter(|(_, hdr)| hdr_valid(hdr))
        .map(|(pos, _)| pos);
    for pos in valid_frames {
        let mp3_view = &mp3[pos..];
        let mut frame_bytes = hdr_frame_bytes(mp3_view, *free_format_bytes);
        let mut frame_and_padding = frame_bytes + hdr_padding(mp3_view);

        let mut k = HDR_SIZE;
        while frame_bytes == 0
            && k < MAX_FREE_FORMAT_FRAME_SIZE
            && pos as i32 + 2 * k < mp3.len() as i32 - HDR_SIZE
        {
            if hdr_compare(mp3_view, &mp3_view[(k as _)..]) {
                let fb = k - hdr_padding(mp3_view);
                let nextfb = fb + hdr_padding(&mp3_view[(k as _)..]);
                if pos as i32 + k + nextfb + HDR_SIZE < mp3.len() as i32
                    && hdr_compare(mp3_view, &mp3_view[((k + nextfb) as _)..])
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
    data_size as i32
}

fn mp3d_match_frame(hdr: &[u8], frame_bytes: i32) -> bool {
    let mut i = 0;
    for nmatch in 0..MAX_FRAME_SYNC_MATCHES {
        i += (hdr_frame_bytes(&hdr[i..], frame_bytes) + hdr_padding(&hdr[i..])) as usize;
        if i + HDR_SIZE as usize > hdr.len() {
            return nmatch > 0;
        } else if !hdr_compare(hdr, &hdr[i..]) {
            return false;
        }
    }
    true
}

fn decode_frame(
    decoder: &mut ffi::mp3dec_t,
    mp3: &[u8],
    pcm: Option<&mut [i16]>,
    info: &mut ffi::mp3dec_frame_info_t,
) -> i32 {
    let mut frame_size = 0;
    if mp3.len() > 4 && decoder.header[0] == 0xff && hdr_compare(&decoder.header, mp3) {
        frame_size = hdr_frame_bytes(mp3, decoder.free_format_bytes) + hdr_padding(mp3);
        if frame_size != mp3.len() as _
            && (frame_size + HDR_SIZE > mp3.len() as i32
                || !hdr_compare(mp3, &mp3[(frame_size as _)..]))
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
    info.channels = if hdr_is_mono(hdr) { 1 } else { 2 };
    info.hz = hdr_sample_rate_hz(hdr);
    info.layer = (4 - hdr_get_layer(hdr)) as _;
    info.bitrate_kbps = hdr_bitrate_kbps(hdr);

    if pcm.is_none() {
        return hdr_frame_samples(hdr);
    }

    let pcm_view = pcm.unwrap();
    let mut pcm_pos = 0;

    let mut bs_frame = ffi::bs_t {
        buf: hdr[(HDR_SIZE as _)..].as_ptr(),
        pos: 0,
        limit: (frame_size - HDR_SIZE) * 8,
    };
    if hdr_is_crc(hdr) {
        bs_frame.pos += 16;
    }

    let mut scratch: ffi::mp3dec_scratch_t = unsafe { mem::uninitialized() };
    let mut success = 1;
    if info.layer == 3 {
        let main_data_begin = unsafe {
            ffi::L3_read_side_info(&mut bs_frame, scratch.gr_info.as_mut_ptr(), hdr.as_ptr())
        };
        if main_data_begin < 0 || bs_frame.pos > bs_frame.limit {
            mp3dec_init(decoder);
            return 0;
        }
        success = unsafe {
            ffi::L3_restore_reservoir(decoder, &mut bs_frame, &mut scratch, main_data_begin)
        };
        if success != 0 {
            let count = if hdr_test_mpeg1(hdr) { 2 } else { 1 };
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
    success * hdr_frame_samples(&decoder.header)
}

#[cfg(test)]
mod tests {
    use super::*;
    quickcheck! {
        fn test_hdr_valid(data: Vec<u8>) -> bool {
            if data.len() == 0 {
                return true // empty data is not interesting
            }
            let mut native_hdr = Vec::new();
            native_hdr.extend(data.into_iter().cycle().take(HDR_SIZE as _));
            let ffi_hdr = native_hdr.clone();
            hdr_valid(&native_hdr) == unsafe { (ffi::hdr_valid(ffi_hdr.as_ptr()) != 0) }
        }
    }

    quickcheck! {
        fn test_hdr_compare(data: Vec<u8>) -> bool {
            if data.len() == 0 {
                return true // empty data is not interesting
            }
            let mut native_hdrs = Vec::new();
            native_hdrs.extend(data.into_iter().cycle().take(2 * HDR_SIZE as usize));
            let ffi_hdrs = native_hdrs.clone();
            let (native_hdr1, native_hdr2) = native_hdrs.split_at(HDR_SIZE as _);
            let (ffi_hdr1, ffi_hdr2) = ffi_hdrs.split_at(HDR_SIZE as _);
            hdr_compare(native_hdr1, native_hdr2) == unsafe { (ffi::hdr_compare(ffi_hdr1.as_ptr(), ffi_hdr2.as_ptr()) != 0) }
        }
    }
}
