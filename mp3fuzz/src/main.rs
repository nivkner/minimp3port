use honggfuzz::fuzz;

fn main() {
    let mut decoder = minimp3port::Decoder::default();
    let mut pcm = [0; minimp3port::MINIMP3_MAX_SAMPLES_PER_FRAME];

    loop {
        fuzz!(|data: &[u8]| {
            decoder.decode_frame(&data, &mut pcm);
        });
    }
}
