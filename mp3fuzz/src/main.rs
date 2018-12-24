use honggfuzz::fuzz;
use minimp3port::{decode_frame, MINIMP3_MAX_SAMPLES_PER_FRAME};

fn main() {
    let mut pcm = [0; MINIMP3_MAX_SAMPLES_PER_FRAME as usize];
    let mut info = unsafe { std::mem::zeroed() };
    let mut decoder = unsafe { std::mem::zeroed() };

    loop {
        fuzz!(|data: &[u8]| {
            decode_frame(&mut decoder, &data, Some(&mut pcm), &mut info);
        });
    }
}
