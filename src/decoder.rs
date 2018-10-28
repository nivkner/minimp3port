#[cfg(test)]
use ffi;
use header;
use {HDR_SIZE, MAX_FRAME_SYNC_MATCHES, MAX_FREE_FORMAT_FRAME_SIZE};

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
    use std::vec::Vec;

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
