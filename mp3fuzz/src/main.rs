use honggfuzz::fuzz;

fn main() {
    let mut decoder = minimp3port::Decoder::default();

    loop {
        fuzz!(|data: &[u8]| {
            decoder.decode_frame(&data);
        });
    }
}
