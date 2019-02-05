use honggfuzz::fuzz;
use minimp3port::{decode_frame, MINIMP3_MAX_SAMPLES_PER_FRAME};

fn main() {
    let mut pcm = [0; MINIMP3_MAX_SAMPLES_PER_FRAME as usize];
    let mut info = minimp3port::FrameInfo::default();
    let mut decoder = minimp3port::Decoder::default();

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
