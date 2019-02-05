use byteorder::{ByteOrder, LittleEndian};

use libpulse_binding::sample;
use libpulse_binding::stream::Direction;
use libpulse_simple_binding::Simple;

use minimp3port::{decode_frame, Decoder, FrameInfo};

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // read the mp3 file
    let mp3_name = env::args_os().nth(1).expect("missing mp3 file name");
    let mut file = File::open(mp3_name).expect("couldn't open mp3 file");
    let mut mp3_buffer = Vec::new();
    file.read_to_end(&mut mp3_buffer)
        .expect("failed to read file");

    let mut decoder = Decoder {
        mdct_overlap: [[0.; 288]; 2],
        qmf_state: [0.; 960],
        reserv: 0,
        free_format_bytes: 0,
        header: [0; 4],
        reserv_buf: [0; 511],
    };
    let mut pcm: [i16; 2304] = [0; 2304];
    let mut frame_info = FrameInfo {
        frame_bytes: 0,
        channels: 0,
        hz: 0,
        layer: 0,
        bitrate_kbps: 0,
    };

    let id3v2size = skip_id3v2(&mp3_buffer);
    if id3v2size > mp3_buffer.len() {
        panic!("too much id3v2");
    }

    // find the first frame
    let mut buf_slice = &mp3_buffer[(id3v2size as usize)..];
    let samples = loop {
        let samples = decode_frame(&mut decoder, buf_slice, &mut pcm, &mut frame_info);
        buf_slice = &buf_slice[(frame_info.frame_bytes as usize)..];
        if 0 != samples || frame_info.frame_bytes == 0 {
            break samples;
        }
    };
    if 0 == samples {
        panic!("not enough samples");
    }

    // save info to start the stream
    let init_samples = (samples * frame_info.channels) as usize;
    let spec = sample::Spec {
        format: sample::SAMPLE_S16NE,
        channels: frame_info.channels as u8,
        rate: frame_info.hz as u32,
    };

    assert!(spec.is_valid());
    let simple = Simple::new(
        None,                // Use the default server
        "Example",           // Our application’s name
        Direction::Playback, // We want a playback stream
        None,                // Use the default device
        "Music",             // Description of our stream
        &spec,               // Our sample format
        None,                // Use default channel map
        None,                // Use default buffering attributes
    )
    .unwrap();

    // the PCM is in i16 so we convert it to u8 so it can be written to pulseaudio
    let mut bytes = [0; 2304 * 2];
    LittleEndian::write_i16_into(&pcm[..init_samples], &mut bytes[..(init_samples * 2)]);
    simple
        .write(&bytes[..(init_samples * 2)])
        .expect("couldn't write to pulse audio");

    // write the rest of the buffer
    while frame_info.frame_bytes != 0 {
        let samples = decode_frame(&mut decoder, buf_slice, &mut pcm, &mut frame_info);
        let all_samples = samples as usize * frame_info.channels as usize;
        buf_slice = &buf_slice[(frame_info.frame_bytes as usize)..];

        LittleEndian::write_i16_into(&pcm[..all_samples], &mut bytes[..(all_samples * 2)]);
        simple
            .write(&bytes[..(init_samples * 2)])
            .expect("couldn't write to pulse audio");
    }

    // drain whatever data is left on pulseaudio
    simple.drain().expect("couldn't drain pulse audio");
    println!("playback complete");
}

fn skip_id3v2(buf: &[u8]) -> usize {
    if buf.len() > 10 && &buf[..3] == b"ID3" {
        ((buf[6] as usize & 0x7f) << 21
            | (buf[7] as usize & 0x7f) << 14
            | (buf[8] as usize & 0x7f) << 7
            | buf[9] as usize & 0x7f)
            + 10
    } else {
        0
    }
}
