#![no_std]

#[cfg(test)]
extern crate std;

mod bits;
mod decoder;
mod ffi;
mod header;
mod l3;

use core::mem;
use core::ptr;
use core::slice;

use crate::bits::Bits;
pub use crate::ffi::{
    hdr_bitrate_kbps, hdr_sample_rate_hz, mp3d_find_frame, mp3d_sample_t, mp3dec_frame_info_t,
    mp3dec_init, mp3dec_t,
};

fn decoder_init(dec: &mut ffi::mp3dec_t) {
    dec.header[0] = 0
}

pub unsafe fn mp3dec_decode_frame(
    dec: *mut ffi::mp3dec_t,
    mp3: *const u8,
    mp3_bytes: libc::c_int,
    pcm: *mut ffi::mp3d_sample_t,
    info: *mut ffi::mp3dec_frame_info_t,
) -> libc::c_int {
    let mp3 = slice::from_raw_parts(mp3, mp3_bytes as _);
    let pcm_slice = if pcm.is_null() {
        None
    } else {
        Some(slice::from_raw_parts_mut(
            pcm,
            MINIMP3_MAX_SAMPLES_PER_FRAME as _,
        ))
    };
    decode_frame(
        dec.as_mut().unwrap(),
        mp3,
        pcm_slice,
        info.as_mut().unwrap(),
    )
}

const HDR_SIZE: i32 = 4;
const MINIMP3_MAX_SAMPLES_PER_FRAME: i32 = 1152 * 2;
const MAX_FREE_FORMAT_FRAME_SIZE: i32 = 2304; // more than ISO spec's
const MAX_FRAME_SYNC_MATCHES: i32 = 10;
const SHORT_BLOCK_TYPE: u8 = 2;

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
        i = decoder::find_frame(mp3, &mut decoder.free_format_bytes, &mut frame_size);
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

    let mut bs_frame = Bits::new(&hdr[(HDR_SIZE as _)..(frame_size as _)]);
    if header::is_crc(hdr) {
        bs_frame.position += 16;
    }

    let mut scratch: ffi::mp3dec_scratch_t = unsafe { mem::zeroed() };
    let mut native_scratch = decoder::Scratch::default();
    let mut success = true;
    if info.layer == 3 {
        let main_data_begin = l3::read_side_info(&mut bs_frame, &mut native_scratch.gr_info, hdr);
        if main_data_begin < 0 || bs_frame.position > bs_frame.limit() {
            decoder_init(decoder);
            return 0;
        }
        success = l3::restore_reservoir(
            decoder,
            &mut bs_frame,
            &mut native_scratch.bits,
            main_data_begin as _,
        );
        native_scratch.convert_to_ffi(&mut scratch);
        if success {
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
        let mut bs_copy = unsafe { bs_frame.bs_copy() };
        let mut sci = unsafe {
            let mut sci: ffi::L12_scale_info = mem::zeroed();
            ffi::L12_read_scale_info(hdr.as_ptr(), &mut bs_copy, &mut sci);
            ptr::write_bytes(&mut scratch.grbuf, 0, 1);
            sci
        };
        let mut i = 0;
        for igr in 0..3 {
            unsafe {
                i += ffi::L12_dequantize_granule(
                    scratch.grbuf[0][(i as _)..].as_mut_ptr(),
                    &mut bs_copy,
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
            if bs_copy.pos > bs_copy.limit {
                decoder_init(decoder);
                return 0;
            }
        }
        bs_frame.position = bs_copy.pos as _;
    }
    success as i32 * header::frame_samples(&decoder.header)
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt;
    use quickcheck::{quickcheck, Arbitrary, Gen};
    use std::vec::Vec;

    impl Arbitrary for ffi::mp3dec_t {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let mut mp3dec: ffi::mp3dec_t = unsafe { mem::zeroed() };
            let mdct_overlap: Vec<_> = (0..(288 * 2)).map(|_| f32::arbitrary(g)).collect();
            mp3dec.mdct_overlap[0].copy_from_slice(&mdct_overlap[..288]);
            mp3dec.mdct_overlap[1].copy_from_slice(&mdct_overlap[288..]);
            let qmf_state: Vec<_> = (0..960).map(|_| f32::arbitrary(g)).collect();
            mp3dec.qmf_state.copy_from_slice(&qmf_state);
            mp3dec.reserv = libc::c_int::arbitrary(g);
            mp3dec.free_format_bytes = libc::c_int::arbitrary(g);
            let header: Vec<_> = (0..4).map(|_| libc::c_uchar::arbitrary(g)).collect();
            mp3dec.header.copy_from_slice(&header);
            let reserv_buf: Vec<_> = (0..511).map(|_| libc::c_uchar::arbitrary(g)).collect();
            mp3dec.reserv_buf.copy_from_slice(&reserv_buf);
            mp3dec
        }
    }

    impl fmt::Debug for ffi::mp3dec_t {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.debug_struct("mp3dec_t")
                .field(
                    "mdct_overlap",
                    &format_args!("{:?}", &self.mdct_overlap[0][..10]),
                )
                .field("qmf_state", &format_args!("{:?}", &self.qmf_state[..10]))
                .field("reserv", &self.reserv)
                .field("free_format_bytes", &self.free_format_bytes)
                .field("header", &format_args!("{:?}", &self.header[..]))
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
            ffi::mp3dec_frame_info_t {
                frame_bytes: libc::c_int::arbitrary(g),
                channels: libc::c_int::arbitrary(g),
                hz: libc::c_int::arbitrary(g),
                layer: libc::c_int::arbitrary(g),
                bitrate_kbps: libc::c_int::arbitrary(g),
            }
        }
    }

    impl PartialEq for ffi::bs_t {
        fn eq(&self, other: &Self) -> bool {
            let self_slice = unsafe { slice::from_raw_parts(self.buf, (self.limit / 8) as _) };
            let other_slice = unsafe { slice::from_raw_parts(other.buf, (other.limit / 8) as _) };
            self_slice == other_slice && self.pos == other.pos && self.limit == self.limit
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
                ffi::mp3dec_decode_frame(
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
