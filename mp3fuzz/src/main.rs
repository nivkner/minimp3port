use honggfuzz::fuzz;
use minimp3port::decode_frame;

fn main() {
    let mut decoder = minimp3port::Decoder::default();

    loop {
        fuzz!(|data: &[u8]| {
            decode_frame(&mut decoder, &data);
        });
    }
}
