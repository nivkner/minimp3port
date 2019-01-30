use honggfuzz::fuzz;
use minimp3port::{decode_frame, MINIMP3_MAX_SAMPLES_PER_FRAME};

fn main() {
    let mut pcm = [0; MINIMP3_MAX_SAMPLES_PER_FRAME as usize];
    let mut info = minimp3port::FrameInfo {
        frame_bytes: 0,
        channels: 0,
        hz: 0,
        layer: 0,
        bitrate_kbps: 0,
    };;
    let mut decoder = minimp3port::Decoder {
        mdct_overlap: [[0.; 288]; 2],
        qmf_state: [0.; 960],
        reserv: 0,
        free_format_bytes: 0,
        header: [0; 4],
        reserv_buf: [0; 511],
    };

    loop {
        fuzz!(|data: &[u8]| {
            let mut repeat = data.iter().rev().cycle().cloned();
            repeat
                .by_ref()
                .zip(decoder.header.iter_mut())
                .for_each(|(from, to)| *to = from);
            let mut bytes = [0u8; 4];
            repeat
                .by_ref()
                .zip(bytes.iter_mut())
                .for_each(|(from, to)| *to = from);
            decoder.free_format_bytes = i32::from_le_bytes(bytes);
            decode_frame(&mut decoder, &data, &mut pcm, &mut info);
        });
    }
}
