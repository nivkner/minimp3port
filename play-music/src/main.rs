use byteorder::{ByteOrder, LittleEndian};

use libpulse_binding::sample;
use libpulse_binding::stream::Direction;
use libpulse_simple_binding::Simple;

use minimp3port::Decoder;

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

    let mut decoder = Decoder::default();

    let id3v2size = skip_id3v2(&mp3_buffer);
    if id3v2size > mp3_buffer.len() {
        panic!("too much id3v2");
    }

    // find the first frame
    let mut buf_slice = &mp3_buffer[(id3v2size as usize)..];
    let frame_info = loop {
        let frame_info = decoder.decode_frame(buf_slice);
        buf_slice = &buf_slice[(frame_info.frame_bytes as usize)..];
        if !decoder.get_pcm().is_empty() {
            break frame_info;
        } else if frame_info.frame_bytes == 0 {
            panic!("not enough samples");
        }
    };

    // save info to start the stream
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
    LittleEndian::write_i16_into(
        decoder.get_pcm(),
        &mut bytes[..(decoder.get_pcm().len() * 2)],
    );
    simple
        .write(&bytes[..(decoder.get_pcm().len() * 2)])
        .expect("couldn't write to pulse audio");

    // write the rest of the buffer
    loop {
        let frame_info = decoder.decode_frame(buf_slice);
        if frame_info.frame_bytes == 0 {
            break;
        }
        let all_samples = decoder.get_pcm().len();
        buf_slice = &buf_slice[(frame_info.frame_bytes as usize)..];

        LittleEndian::write_i16_into(decoder.get_pcm(), &mut bytes[..(all_samples * 2)]);
        if all_samples > 0 {
            simple
                .write(&bytes[..(all_samples * 2)])
                .expect("couldn't write to pulse audio");
        }
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
