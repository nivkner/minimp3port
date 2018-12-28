#![allow(bad_style)]
#![allow(clippy::all)]

extern crate minimp3port;

use minimp3port::*;

extern crate libc;
/*
    https://github.com/lieff/minimp3
    To the extent possible under law, the author(s) have dedicated all copyright and related and neighboring rights to this software to the public domain worldwide.
    This software is distributed without any warranty.
    See <http://creativecommons.org/publicdomain/zero/1.0/>.
*/
#[derive(Clone)]
#[repr(C)]
pub struct mp3dec_file_info_t {
    pub buffer: Vec<i16>,
    pub samples: libc::size_t,
    pub channels: libc::c_int,
    pub hz: libc::c_int,
    pub layer: libc::c_int,
    pub avg_bitrate_kbps: libc::c_int,
}

/* decode whole buffer block */
fn load_buffer(dec: &mut mp3dec_t, buf: &[u8], info: &mut mp3dec_file_info_t) {
    let mut pcm: [mp3d_sample_t; 2304] = [0; 2304];
    let mut frame_info: mp3dec_frame_info_t = mp3dec_frame_info_t {
        frame_bytes: 0,
        channels: 0,
        hz: 0,
        layer: 0,
        bitrate_kbps: 0,
    };
    /* skip id3v2 */
    let id3v2size = mp3dec_skip_id3v2(buf);
    if id3v2size > buf.len() {
        return;
    }
    let mut buf_slice = &buf[(id3v2size as usize)..];
    unsafe { mp3dec_init(dec) };
    let mut samples: libc::c_int;
    loop {
        samples = decode_frame(dec, buf_slice, Some(&mut pcm), &mut frame_info);
        buf_slice = &buf_slice[(frame_info.frame_bytes as usize)..];
        if 0 != samples {
            break;
        }
        if !(0 != frame_info.frame_bytes) {
            break;
        }
    }
    if 0 == samples {
        return;
    } else {
        samples *= frame_info.channels;
        info.samples = samples as libc::size_t;
        info.buffer.extend_from_slice(&pcm[..(samples as usize)]);
        /* save info */
        info.channels = frame_info.channels;
        info.hz = frame_info.hz;
        info.layer = frame_info.layer;
        let mut avg_bitrate_kbps: libc::size_t = frame_info.bitrate_kbps as libc::size_t;
        let mut frames: libc::size_t = 1i32 as libc::size_t;
        /* decode rest frames */
        loop {
            samples = decode_frame(dec, buf_slice, Some(&mut pcm), &mut frame_info);
            info.buffer
                .extend_from_slice(&pcm[..(samples as usize * frame_info.channels as usize)]);
            buf_slice = &buf_slice[(frame_info.frame_bytes as usize)..];
            let frame_bytes = frame_info.frame_bytes;
            if 0 != samples {
                if info.hz != frame_info.hz || info.layer != frame_info.layer {
                    break;
                }
                if 0 != info.channels && info.channels != frame_info.channels {
                    /* mark file with mono-stereo transition */
                    info.channels = 0i32
                }
                info.samples = (info.samples as libc::c_ulong)
                    .wrapping_add((samples * frame_info.channels) as libc::c_ulong)
                    as libc::size_t;
                avg_bitrate_kbps = (avg_bitrate_kbps as libc::c_ulong)
                    .wrapping_add(frame_info.bitrate_kbps as libc::c_ulong)
                    as libc::size_t;
                frames = frames.wrapping_add(1);
            }
            if !(0 != frame_bytes) {
                break;
            }
        }
        info.avg_bitrate_kbps = avg_bitrate_kbps.wrapping_div(frames) as libc::c_int;
        return;
    };
}

fn mp3dec_skip_id3v2(buf: &[u8]) -> usize {
    if buf.len() > 10 && &buf[..3] == b"ID3" {
        return ((buf[6] as usize & 0x7f) << 21
            | (buf[7] as usize & 0x7f) << 14
            | (buf[8] as usize & 0x7f) << 7
            | buf[9] as usize & 0x7f)
            + 10;
    } else {
        return 0;
    };
}

fn decode(input_buffer: &[u8], buf: &[u8]) {
    let mut mp3d: mp3dec_t = mp3dec_t {
        mdct_overlap: [[0.; 288]; 2],
        qmf_state: [0.; 960],
        reserv: 0,
        free_format_bytes: 0,
        header: [0; 4],
        reserv_buf: [0; 511],
    };
    let mut total_samples: libc::c_int = 0i32;
    let mut maxdiff: libc::c_int = 0i32;
    let mut MSE: libc::c_double = 0.0f64;
    let mut info: mp3dec_file_info_t = mp3dec_file_info_t {
        buffer: Vec::new(),
        samples: 0,
        channels: 0,
        hz: 0,
        layer: 0,
        avg_bitrate_kbps: 0,
    };
    load_buffer(&mut mp3d, input_buffer, &mut info);
    if 0 != info.samples {
        total_samples += info.samples as i32;
        let max_samples = std::cmp::min(buf.len() / 2, info.samples as usize);
        for i in 0..max_samples {
            const SIZE: usize = std::mem::size_of::<i16>();
            let mut bytes: [u8; SIZE] = [0u8; SIZE];
            bytes.copy_from_slice(&buf[(i * SIZE)..(i * SIZE + SIZE)]);
            let ref_res: i16 = unsafe { i16::from_le(std::mem::transmute(bytes)) };
            let info_res = info.buffer[i as usize];
            let diff = (ref_res - info_res).abs();
            if diff as i32 > maxdiff {
                maxdiff = diff.into()
            }
            MSE += f64::from(diff.pow(2));
        }
    }
    MSE /= if 0 != total_samples {
        total_samples as f64
    } else {
        1.0
    };
    let psnr = if MSE == 0.0 {
        99.0
    } else {
        10.0 * (0x7fffu32.pow(2) as f64 / MSE).log10()
    };
    println!(
        "rate={} samples={} max_diff={} PSNR={}",
        info.hz, total_samples, maxdiff, psnr
    );
    assert!(psnr > 96.0, "PSNR compliance failed")
}

pub fn main() {
    let rinput = std::env::args().nth(1).unwrap();
    let rinput_buf = std::fs::read(rinput).unwrap();
    let rref = std::env::args().nth(2).unwrap();
    let rref_buf = std::fs::read(rref).unwrap();

    decode(&rinput_buf, &rref_buf);
}
