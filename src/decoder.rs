use crate::bits::Bits;
use crate::header;
use crate::{HDR_SIZE, MAX_FRAME_SYNC_MATCHES, MAX_FREE_FORMAT_FRAME_SIZE};

#[derive(Copy, Clone)]
pub struct Scratch {
    pub bits: BitsProxy,
    pub grbuf: [[f32; 576]; 2],
    pub scf: [f32; 40],
    pub syn: [[f32; 64]; 33],
    pub ist_pos: [[u8; 39]; 2],
}

// used to avoid self referencial structs
#[derive(Copy, Clone)]
pub struct BitsProxy {
    pub position: usize,
    pub len: usize,
    pub maindata: [u8; 2815],
}

impl BitsProxy {
    pub fn with_bits<F, T>(&mut self, fun: F) -> T
    where
        F: FnOnce(&mut Bits) -> T,
    {
        let mut bits = Bits::new_with_pos(&self.maindata[..self.len], self.position);
        let res = fun(&mut bits);
        self.position = bits.position;
        res
    }
}

impl Default for BitsProxy {
    fn default() -> Self {
        BitsProxy {
            position: 0,
            len: 2815,
            maindata: [0; 2815],
        }
    }
}

impl Default for Scratch {
    fn default() -> Self {
        Scratch {
            bits: BitsProxy::default(),
            grbuf: [[0.0; 576]; 2],
            scf: [0.0; 40],
            syn: [[0.0; 64]; 33],
            ist_pos: [[0; 39]; 2],
        }
    }
}

pub fn find_frame(mp3: &[u8], free_format_bytes: &mut i32, ptr_frame_bytes: &mut i32) -> i32 {
    let valid_frames = mp3
        .windows(HDR_SIZE as _)
        .enumerate()
        .filter(|(_, hdr)| header::is_valid(hdr))
        .map(|(pos, _)| pos);
    for pos in valid_frames {
        let mp3_view = &mp3[pos..];
        let mut frame_bytes = header::frame_bytes(mp3_view, *free_format_bytes);
        let mut frame_and_padding = frame_bytes + header::padding(mp3_view);

        let mut k = HDR_SIZE;
        while frame_bytes == 0
            && k < MAX_FREE_FORMAT_FRAME_SIZE
            && pos as i32 + 2 * k < mp3.len() as i32 - HDR_SIZE
        {
            if header::compare(mp3_view, &mp3_view[(k as _)..]) {
                let fb = k - header::padding(mp3_view);
                let nextfb = fb + header::padding(&mp3_view[(k as _)..]);
                if pos as i32 + k + nextfb + HDR_SIZE < mp3.len() as i32
                    && header::compare(mp3_view, &mp3_view[((k + nextfb) as _)..])
                {
                    frame_and_padding = k;
                    frame_bytes = fb;
                    *free_format_bytes = fb;
                }
            }
            k += 1;
        }

        if (frame_bytes != 0
            && pos as i32 + frame_and_padding <= mp3.len() as i32
            && match_frame(mp3_view, frame_bytes))
            || (pos == 0 && frame_and_padding == mp3.len() as i32)
        {
            *ptr_frame_bytes = frame_and_padding;
            return pos as i32;
        }
        *free_format_bytes = 0;
    }
    *ptr_frame_bytes = 0;
    // match c version behavior, returns 0 when len < 4
    mp3.len().saturating_sub(HDR_SIZE as _) as i32
}

pub fn match_frame(hdr: &[u8], frame_bytes: i32) -> bool {
    let mut i = 0;
    for nmatch in 0..MAX_FRAME_SYNC_MATCHES {
        i += (header::frame_bytes(&hdr[i..], frame_bytes) + header::padding(&hdr[i..])) as usize;
        if i + HDR_SIZE as usize > hdr.len() {
            return nmatch > 0;
        } else if !header::compare(hdr, &hdr[i..]) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ffi;
    use core::fmt;
    use quickcheck::{quickcheck, Arbitrary, Gen};
    use std::vec::Vec;

    impl Scratch {
        // cannot be a From implementation because scratch is self referencial
        pub fn convert_to_ffi(mut self, scratch: &mut ffi::mp3dec_scratch_t) {
            let bs = self.bits.with_bits(|bits| unsafe { bits.bs_copy() });
            // when moved into the struct,
            // the pointer in bs is still pointing at maindata in self
            scratch.bs = bs;
            scratch.maindata = self.bits.maindata;
            scratch.grbuf = self.grbuf;
            scratch.scf = self.scf;
            scratch.syn = self.syn;
            scratch.ist_pos = self.ist_pos;
            // set bs to point to its maindata
            scratch.bs.buf = scratch.maindata.as_ptr();
        }
    }

    impl Arbitrary for BitsProxy {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let mut bits = BitsProxy::default();
            let maindata: Vec<_> = (0..2815).map(|_| u8::arbitrary(g)).collect();
            // the limit should be greater than the position
            let len = usize::arbitrary(g) % 2815;
            let position = (usize::arbitrary(g) % 2815) * 8;
            if position > len * 8 {
                bits.len = position / 8;
                bits.position = len * 8;
            } else {
                bits.position = position;
                bits.len = len;
            }
            bits.maindata.copy_from_slice(&maindata);
            bits
        }
    }

    impl fmt::Debug for BitsProxy {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.debug_struct("BitsProxy")
                .field("position", &self.position)
                .field("len", &self.len)
                .field("maindata", &format_args!("{:?}", &self.maindata[..10]))
                .finish()
        }
    }

    quickcheck! {
        fn test_find_frame(mp3: Vec<u8>, free_format_bytes: i32, ptr_frame_bytes: i32) -> bool {
            let mut native_ffb = free_format_bytes;
            let mut native_pfb = ptr_frame_bytes;
            let mut ffi_ffb = free_format_bytes;
            let mut ffi_pfb = ptr_frame_bytes;

            let native_res = find_frame(&mp3, &mut native_ffb, &mut native_pfb);
            let ffi_res = unsafe {
                ffi::mp3d_find_frame(
                    mp3.as_ptr(),
                    mp3.len() as _,
                    &mut ffi_ffb,
                    &mut ffi_pfb
                    )
                };
            native_res == ffi_res &&
                native_ffb == ffi_ffb &&
                native_pfb == ffi_pfb
        }
    }

    quickcheck! {
        fn test_match_frame(hdr: header::ValidHeader, data: Vec<u8>, frame_bytes: u32) -> bool {
            let mp3: Vec<u8> = hdr.0.iter().chain(data.iter()).map(|n| *n).collect();
            match_frame(&mp3, frame_bytes as _) == unsafe {
                ffi::mp3d_match_frame(mp3.as_ptr(), mp3.len() as _, frame_bytes as _) != 0
            }
        }
    }
}
