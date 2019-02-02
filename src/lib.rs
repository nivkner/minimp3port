#![no_std]
#![deny(clippy::all)]

mod bits;
mod decoder;
mod ffi;
mod header;
mod l12;
mod l3;

use core::ptr;

use crate::bits::BitStream;
pub use crate::decoder::{Decoder, FrameInfo, Scratch};

fn decoder_init(dec: &mut Decoder) {
    dec.header[0] = 0
}

pub const MINIMP3_MAX_SAMPLES_PER_FRAME: i32 = 1152 * 2;
const HDR_SIZE: usize = 4;
const MAX_FREE_FORMAT_FRAME_SIZE: i32 = 2304; // more than ISO spec's
const MAX_FRAME_SYNC_MATCHES: i32 = 10;
const SHORT_BLOCK_TYPE: u8 = 2;
const MAX_BITRESERVOIR_BYTES: usize = 511;
const BITS_DEQUANTIZER_OUT: i32 = -1;
const MAX_SCF: i32 = 255 + BITS_DEQUANTIZER_OUT * 4 - 210;
const MAX_SCFI: i32 = (MAX_SCF + 3) & !3;
const MODE_MONO: u8 = 3;
const MODE_JOINT_STEREO: u8 = 1;

pub fn decode_frame(
    decoder: &mut Decoder,
    mp3: &[u8],
    pcm: &mut [i16],
    info: &mut FrameInfo,
) -> i32 {
    let mut frame_size = 0;
    if mp3.len() > 4 && decoder.header[0] == 0xff && header::compare(&decoder.header, mp3) {
        frame_size = header::frame_bytes(mp3, decoder.free_format_bytes) + header::padding(mp3);
        // the condition is arranged such that if the frame size is too big
        // the expression would short-circuit before slicing the mp3 buffer
        if !(frame_size == mp3.len() as _
            || (frame_size as usize).saturating_add(HDR_SIZE) <= mp3.len()
                && header::compare(mp3, &mp3[(frame_size as _)..]))
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
    decoder.header.copy_from_slice(&hdr[..HDR_SIZE]);
    info.frame_bytes = i + frame_size;
    info.channels = if header::is_mono(hdr) { 1 } else { 2 };
    info.hz = header::sample_rate_hz(hdr);
    info.layer = (4 - header::get_layer(hdr)).into();
    info.bitrate_kbps = header::bitrate_kbps(hdr);

    let mut pcm_pos = 0;

    let mut bs_frame = BitStream::new(&hdr[HDR_SIZE..(frame_size as _)]);
    if header::is_crc(hdr) {
        bs_frame.position += 16;
    }

    let mut scratch = Scratch::default();
    if info.layer == 3 {
        let mut gr_info = [l3::GrInfo::default(); 4];
        let main_data_begin = l3::read_side_info(&mut bs_frame, &mut gr_info, hdr);
        if main_data_begin < 0 || bs_frame.position > bs_frame.limit {
            decoder_init(decoder);
            return 0;
        }

        let mut main_data = [0; 2815];
        let (mut scratch_bs, success) =
            l3::restore_reservoir(decoder, &mut bs_frame, &mut main_data, main_data_begin as _);

        if success {
            let count = if header::test_mpeg1(hdr) { 2 } else { 1 };
            for igr in 0..count {
                unsafe {
                    ptr::write_bytes(&mut scratch.grbuf, 0, 1);
                }
                l3::decode(
                    decoder,
                    &mut scratch,
                    &mut scratch_bs,
                    &gr_info[((igr * info.channels) as _)..],
                    info.channels as _,
                );
                decoder::synth_granule(
                    &mut decoder.qmf_state,
                    &mut scratch.grbuf,
                    18,
                    info.channels as usize,
                    &mut pcm[pcm_pos..],
                    &mut scratch.syn,
                );
                pcm_pos += 576 * info.channels as usize;
            }
            l3::save_reservoir(decoder, &mut scratch_bs);
        } else {
            l3::save_reservoir(decoder, &mut scratch_bs);
            return 0;
        }
    } else {
        let mut sci = ffi::L12_scale_info {
            scf: [0.0; 192],
            total_bands: 0,
            stereo_bands: 0,
            bitalloc: [0; 64],
            scfcod: [0; 64],
        };
        l12::read_scale_info(hdr, &mut bs_frame, &mut sci);
        unsafe {
            ptr::write_bytes(&mut scratch.grbuf, 0, 1);
        }
        let mut i = 0;
        let mut bs_copy = unsafe { bs_frame.bs_copy() };
        for igr in 0..3 {
            unsafe {
                i += ffi::L12_dequantize_granule(
                    scratch.grbuf[(i as _)..].as_mut_ptr(),
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
                        scratch.grbuf.as_mut_ptr(),
                    );
                    decoder::synth_granule(
                        &mut decoder.qmf_state,
                        &mut scratch.grbuf,
                        12,
                        info.channels as usize,
                        &mut pcm[pcm_pos..],
                        &mut scratch.syn,
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
    header::frame_samples(&decoder.header)
}
