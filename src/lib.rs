#![no_std]
#![deny(clippy::all)]

mod bits;
mod decoder;
mod header;
mod layer3;
mod layers12;

use crate::bits::BitStream;
use crate::decoder::Scratch;
use crate::layer3::GranuleInfo;
use crate::layers12::ScaleInfo;

/// the maximum number of samples that can be decoded from a single frame
pub const MINIMP3_MAX_SAMPLES_PER_FRAME: usize = 1152 * 2;
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

/// a struct used to decode mp3 buffers.
/// should be reused for all frames from the same file.
pub struct Decoder {
    pub(crate) mdct_overlap: [[f32; 288]; 2],
    pub(crate) qmf_state: [f32; 960],
    pub(crate) reserv: i32,
    pub(crate) free_format_bytes: i32,
    pub(crate) header: [u8; 4],
    pub(crate) reserv_buf: [u8; 511],
}

#[derive(Copy, Clone)]
/// information about the decoded frame
pub struct FrameInfo {
    pub(crate) frame_bytes: i32,
    pub(crate) channels: i32,
    pub(crate) hz: i32,
    pub(crate) layer: i32,
    pub(crate) bitrate_kbps: i32,
    pub(crate) samples: usize,
}

impl FrameInfo {
    /// the number of bytes that were processed by the decoder.
    /// if the decoder processes non-audio data, it would still be included
    pub fn frame_bytes(&self) -> usize {
        self.frame_bytes as usize
    }

    /// the number of samples that were decoded
    pub fn samples(&self) -> usize {
        self.samples
    }

    /// the sample rate in Hz
    pub fn sample_rate(&self) -> u32 {
        self.hz as u32
    }

    /// the bitrate, measured in kilobytes per second
    pub fn bitrate(&self) -> u32 {
        self.bitrate_kbps as u32
    }

    /// the number of audio channels
    pub fn channels(&self) -> u8 {
        self.channels as u8
    }

    /// the audio format, can be layers 1, 2, or 3
    pub fn layer(&self) -> u8 {
        self.layer as u8
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

impl Decoder {
    /// resets the state of the decoder to allow for its use for other mp3 files
    pub fn reset(&mut self) {
        self.header[0] = 0
    }

    /// decode a frame out of a mp3 buffer, and stores the PCM output in a given buffer.
    /// the PCM buffer should be at least `MINIMP3_MAX_SAMPLES_PER_FRAME` in length
    pub fn decode_frame(&mut self, mp3: &[u8], pcm: &mut [i16]) -> FrameInfo {
        let mut info = FrameInfo {
            frame_bytes: 0,
            channels: 0,
            hz: 0,
            layer: 0,
            bitrate_kbps: 0,
            samples: 0,
        };
        let mut frame_size = 0;
        if mp3.len() > 4 && self.header[0] == 0xff && header::compare(&self.header, mp3) {
            frame_size = header::frame_bytes(mp3, self.free_format_bytes) + header::padding(mp3);
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
            *self = Decoder::default();
            i = decoder::find_frame(mp3, &mut self.free_format_bytes, &mut frame_size);
            if frame_size == 0 || i + frame_size > mp3.len() as _ {
                info.frame_bytes = i;
                return info;
            }
        }

        let hdr = &mp3[(i as _)..];
        self.header.copy_from_slice(&hdr[..HDR_SIZE]);
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
            let mut gr_info = [GranuleInfo::default(); 4];
            let main_data_begin = layer3::read_side_info(&mut bs_frame, &mut gr_info, hdr);
            if main_data_begin < 0 || bs_frame.position > bs_frame.limit {
                self.reset();
                return info;
            }

            let mut main_data = [0; 2815];
            let (mut scratch_bs, success) = layer3::restore_reservoir(
                self,
                &mut bs_frame,
                &mut main_data,
                main_data_begin as _,
            );

            if success {
                let count = if header::test_mpeg1(hdr) { 2 } else { 1 };
                for igr in 0..count {
                    scratch.grbuf.copy_from_slice(&[0.0; 576 * 2]);
                    layer3::decode(
                        self,
                        &mut scratch,
                        &mut scratch_bs,
                        &gr_info[((igr * info.channels) as _)..],
                        info.channels as _,
                    );
                    decoder::synth_granule(
                        &mut self.qmf_state,
                        &mut scratch.grbuf,
                        18,
                        info.channels as usize,
                        &mut pcm[pcm_pos..],
                        &mut scratch.syn,
                    );
                    pcm_pos += 576 * info.channels as usize;
                }
                layer3::save_reservoir(self, &mut scratch_bs);
            } else {
                layer3::save_reservoir(self, &mut scratch_bs);
                return info;
            }
        } else {
            let mut sci = ScaleInfo {
                scf: [0.0; 192],
                total_bands: 0,
                stereo_bands: 0,
                bitalloc: [0; 64],
                scfcod: [0; 64],
            };
            layers12::read_scale_info(hdr, &mut bs_frame, &mut sci);
            let mut i = 0;
            for igr in 0..3 {
                i += layers12::dequantize_granule(
                    &mut scratch.grbuf[i..],
                    &mut bs_frame,
                    &mut sci,
                    info.layer as usize | 1,
                );
                if i == 12 {
                    i = 0;
                    layers12::apply_scf_384(&mut sci, igr, &mut scratch.grbuf);
                    decoder::synth_granule(
                        &mut self.qmf_state,
                        &mut scratch.grbuf,
                        12,
                        info.channels as usize,
                        &mut pcm[pcm_pos..],
                        &mut scratch.syn,
                    );
                    scratch.grbuf.copy_from_slice(&[0.0; 576 * 2]);
                    pcm_pos += 384 * info.channels as usize;
                }
                if bs_frame.position > bs_frame.limit {
                    self.reset();
                    return info;
                }
            }
        }
        info.samples = (header::frame_samples(&self.header) * info.channels) as usize;
        info
    }
}
