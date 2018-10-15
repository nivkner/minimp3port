mod ffi;

use std::mem;
use std::num::NonZeroUsize;
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
                ffi::MINIMP3_MAX_SAMPLES_PER_FRAME as _,
            ))
        }
    };
    decode_frame(dec, mp3, pcm_slice, info)
        .map(NonZeroUsize::get)
        .unwrap_or(0) as _
}

#[inline]
fn hdr_is_mono(hdr: &[u8]) -> bool {
    hdr[3] & 0xC0 == 0xC0
}
#[inline]
fn hdr_is_crc(hdr: &[u8]) -> bool {
    hdr[1] & 1 == 0
}

#[inline]
fn hdr_get_layer(hdr: &[u8]) -> u8 {
    hdr[1] >> 1 & 3
}

#[inline]
fn hdr_test_mpeg1(hdr: &[u8]) -> bool {
    hdr[1] & 0x8 != 0
}

fn decode_frame(
    decoder: &mut ffi::mp3dec_t,
    mp3: &[u8],
    pcm: Option<&mut [i16]>,
    info: &mut ffi::mp3dec_frame_info_t,
) -> Option<NonZeroUsize> {
    let mut frame_size = 0;
    if mp3.len() > 4
        && decoder.header[0] == 0xff
        && unsafe { ffi::hdr_compare((&decoder.header).as_ptr(), mp3.as_ptr()) != 0 }
    {
        frame_size = unsafe {
            ffi::hdr_frame_bytes(mp3.as_ptr(), decoder.free_format_bytes)
                + ffi::hdr_padding(mp3.as_ptr())
        };
        if frame_size != mp3.len() as _
            && (frame_size as usize + ffi::HDR_SIZE as usize > mp3.len()
                || unsafe { ffi::hdr_compare(mp3.as_ptr(), mp3[(frame_size as _)..].as_ptr()) }
                    == 0)
        {
            frame_size = 0;
        }
    }

    let mut i = 0;
    if frame_size == 0 {
        i = unsafe {
            ptr::write_bytes(decoder, 0, 1);
            ffi::mp3d_find_frame(
                mp3.as_ptr(),
                mp3.len() as _,
                &mut decoder.free_format_bytes,
                &mut frame_size,
            )
        };
        if frame_size == 0 || i + frame_size > mp3.len() as _ {
            info.frame_bytes = i;
            return None;
        }
    }

    let hdr = &mp3[(i as _)..];
    decoder.header.copy_from_slice(&hdr[..(ffi::HDR_SIZE as _)]);
    info.frame_bytes = i + frame_size;
    info.channels = if hdr_is_mono(hdr) { 1 } else { 2 };
    info.hz = unsafe { ffi::hdr_sample_rate_hz(hdr.as_ptr()) } as _;
    info.layer = (4 - hdr_get_layer(hdr)) as _;
    info.bitrate_kbps = unsafe { ffi::hdr_bitrate_kbps(hdr.as_ptr()) as _ };

    if pcm.is_none() {
        return unsafe { NonZeroUsize::new(ffi::hdr_frame_samples(hdr.as_ptr()) as _) };
    }

    let pcm_view = pcm.unwrap();
    let mut pcm_pos = 0;

    let mut bs_frame = ffi::bs_t {
        buf: hdr[(ffi::HDR_SIZE as _)..].as_ptr(),
        pos: 0,
        limit: ((frame_size as usize - ffi::HDR_SIZE as usize) * 8) as _,
    };
    if hdr_is_crc(hdr) {
        unsafe {
            ffi::get_bits(&mut bs_frame, 16);
        }
    }

    let mut scratch: ffi::mp3dec_scratch_t = unsafe { mem::uninitialized() };
    let mut success = 1;
    if info.layer == 3 {
        let main_data_begin = unsafe {
            ffi::L3_read_side_info(&mut bs_frame, scratch.gr_info.as_mut_ptr(), hdr.as_ptr())
        };
        if main_data_begin < 0 || bs_frame.pos > bs_frame.limit {
            mp3dec_init(decoder);
            return None;
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
                return None;
            }
        }
    }
    unsafe {
        NonZeroUsize::new(
            success as usize * ffi::hdr_frame_samples(decoder.header.as_mut_ptr()) as usize,
        )
    }
}
