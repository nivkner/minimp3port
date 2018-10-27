#[cfg(test)]
use ffi;

#[inline]
pub fn is_mono(hdr: &[u8]) -> bool {
    hdr[3] & 0xC0 == 0xC0
}

#[inline]
pub fn is_crc(hdr: &[u8]) -> bool {
    hdr[1] & 1 == 0
}

#[inline]
pub fn is_free_format(hdr: &[u8]) -> bool {
    hdr[2] & 0xF0 == 0
}

#[inline]
pub fn is_layer_1(hdr: &[u8]) -> bool {
    hdr[1] & 6 == 6
}

#[inline]
pub fn is_frame_576(hdr: &[u8]) -> bool {
    hdr[1] & 14 == 2
}

#[inline]
pub fn get_layer(hdr: &[u8]) -> u8 {
    hdr[1] >> 1 & 3
}

#[inline]
pub fn get_bitrate(hdr: &[u8]) -> u8 {
    hdr[2] >> 4
}

#[inline]
pub fn get_sample_rate(hdr: &[u8]) -> u8 {
    hdr[2] >> 2 & 3
}

#[inline]
pub fn get_my_sample_rate(hdr: &[u8]) -> u8 {
    get_sample_rate(hdr) + (((hdr[1] >> 3) & 1) + ((hdr[1] >> 4) & 1)) * 3
}

#[inline]
pub fn test_mpeg1(hdr: &[u8]) -> bool {
    hdr[1] & 0x8 != 0
}

#[inline]
pub fn test_padding(hdr: &[u8]) -> bool {
    hdr[2] & 0x2 != 0
}

#[inline]
pub fn test_not_mpeg25(hdr: &[u8]) -> bool {
    hdr[1] & 0x10 != 0
}

pub fn is_valid(hdr: &[u8]) -> bool {
    hdr[0] == 0xFF
        && ((hdr[1] & 0xF0) == 0xF0 || (hdr[1] & 0xFE) == 0xE2)
        && get_layer(hdr) != 0
        && get_bitrate(hdr) != 15
        && get_sample_rate(hdr) != 3
}

pub fn compare(this: &[u8], other: &[u8]) -> bool {
    is_valid(other)
        && (this[1] ^ other[1]) & 0xFE == 0
        && (this[2] ^ other[2]) & 0x0C == 0
        && is_free_format(this) as u8 ^ is_free_format(other) as u8 == 0
}

pub fn frame_bytes(hdr: &[u8], free_format_size: i32) -> i32 {
    let mut frame_bytes = frame_samples(hdr) * bitrate_kbps(hdr) * 125 / sample_rate_hz(hdr);
    if is_layer_1(hdr) {
        frame_bytes &= !3; // slot align
    }
    if frame_bytes != 0 {
        frame_bytes
    } else {
        free_format_size
    }
}

pub fn padding(hdr: &[u8]) -> i32 {
    if test_padding(hdr) {
        if is_layer_1(hdr) {
            4
        } else {
            1
        }
    } else {
        0
    }
}

pub fn frame_samples(hdr: &[u8]) -> i32 {
    if is_layer_1(hdr) {
        384
    } else {
        1152 >> is_frame_576(hdr) as u8
    }
}

pub fn bitrate_kbps(hdr: &[u8]) -> i32 {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    static HALFRATE: [u8 ; 2 * 3 * 15] = [
        0,4,8,12,16,20,24,28,32,40,48,56,64,72,80,
        0,4,8,12,16,20,24,28,32,40,48,56,64,72,80,
        0,16,24,28,32,40,48,56,64,72,80,88,96,112,128,

        0,16,20,24,28,32,40,48,56,64,80,96,112,128,160,
        0,16,24,28,32,40,48,56,64,80,96,112,128,160,192,
        0,16,32,48,64,80,96,112,128,144,160,176,192,208,224,
    ];
    2 * HALFRATE
        [(get_bitrate(hdr) + (get_layer(hdr) - 1) * 15 + test_mpeg1(hdr) as u8 * 3 * 15) as usize]
        as i32
}

pub fn sample_rate_hz(hdr: &[u8]) -> i32 {
    let g_hz: [i32; 3] = [44100, 48000, 32000];
    g_hz[get_sample_rate(hdr) as usize] >> !test_mpeg1(hdr) as u8 >> !test_not_mpeg25(hdr) as u8
}

#[cfg(test)]
#[derive(Copy, Clone, Debug)]
pub struct Header(pub [u8; 4]);

#[cfg(test)]
#[derive(Copy, Clone, Debug)]
pub struct ValidHeader(pub [u8; 4]);

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use std::vec::Vec;

    impl Arbitrary for Header {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Header([
                u8::arbitrary(g),
                u8::arbitrary(g),
                u8::arbitrary(g),
                u8::arbitrary(g),
            ])
        }
    }

    impl Arbitrary for ValidHeader {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let mut header = ValidHeader([0xFF, 0, 0, 0]);
            let options: Vec<u8> = (0u8..=255u8)
                .filter(|num| {
                    num >> 1u8 & 3u8 != 0u8
                        && ((num & 0xF0u8) == 0xF0u8 || (num & 0xFEu8) == 0xE2u8)
                }).collect();
            let idx = u8::arbitrary(g) % options.len() as u8;
            header.0[1] = options[idx as usize];
            let options: Vec<u8> = (0u8..=255u8)
                .filter(|num| num >> 4u8 != 15u8 && num >> 2u8 & 3u8 != 3u8)
                .collect();
            let idx = u8::arbitrary(g) % options.len() as u8;
            header.0[2] = options[idx as usize];
            header.0[3] = u8::arbitrary(g);
            assert!(is_valid(&header.0));
            header
        }
    }

    quickcheck! {
        fn test_is_valid(hdr: Header) -> bool {
            is_valid(&hdr.0) == unsafe { (ffi::hdr_valid(hdr.0.as_ptr()) != 0) }
        }
    }

    quickcheck! {
        fn test_compare(hdr1: Header, hdr2: Header) -> bool {
            compare(&hdr1.0, &hdr2.0) == unsafe { (ffi::hdr_compare(hdr1.0.as_ptr(), hdr2.0.as_ptr()) != 0) }
        }
    }

    quickcheck! {
        fn test_frame_bytes(hdr: ValidHeader, free_format_bytes: i32) -> bool {
            frame_bytes(&hdr.0, free_format_bytes) == unsafe { ffi::hdr_frame_bytes(hdr.0.as_ptr(), free_format_bytes) }
        }
    }

    quickcheck! {
        fn test_padding(hdr: Header) -> bool {
            padding(&hdr.0) == unsafe { ffi::hdr_padding(hdr.0.as_ptr()) }
        }
    }

    quickcheck! {
        fn test_frame_samples(hdr: Header) -> bool {
            frame_samples(&hdr.0) as i64 == unsafe { ffi::hdr_frame_samples(hdr.0.as_ptr()) as i64 }
        }
    }

    quickcheck! {
        fn test_bitrate_kbps(hdr: ValidHeader) -> bool {
            bitrate_kbps(&hdr.0) as i64 == unsafe { ffi::hdr_bitrate_kbps(hdr.0.as_ptr()) as i64 }
        }
    }

    quickcheck! {
        fn test_sample_rate_hz(hdr: ValidHeader) -> bool {
            sample_rate_hz(&hdr.0) as i64 == unsafe { ffi::hdr_sample_rate_hz(hdr.0.as_ptr()) as i64 }
        }
    }
}
