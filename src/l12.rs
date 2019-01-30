use crate::bits::BitStream;
use crate::{ffi, header};

pub fn read_scale_info(hdr: &[u8], bs: &mut BitStream<'_>, mut sci: &mut ffi::L12_scale_info) {
    let g_bitalloc_code_tab: [u8; 92] = [
        0, 17, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 0, 17, 18, 3, 19, 4, 5, 6, 7, 8, 9,
        10, 11, 12, 13, 16, 0, 17, 18, 3, 19, 4, 5, 16, 0, 17, 18, 16, 0, 17, 18, 19, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15, 0, 17, 18, 3, 19, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 2,
        3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ];
    let mut subband_alloc: *const ffi::L12_subband_alloc_t =
        unsafe { ffi::L12_subband_alloc_table(hdr.as_ptr(), sci) };
    let mut k = 0;
    let mut ba_bits = 0;
    let mut ba_code_tab = &g_bitalloc_code_tab as &[u8];
    for i in 0..(sci.total_bands as usize) {
        if i == k {
            unsafe {
                k += (*subband_alloc).band_count as usize;
                ba_bits = u32::from((*subband_alloc).code_tab_width);
                ba_code_tab = &g_bitalloc_code_tab[((*subband_alloc).tab_offset as usize)..];
                subband_alloc = subband_alloc.offset(1)
            }
        }
        let mut ba = ba_code_tab[bs.get_bits(ba_bits) as usize];
        sci.bitalloc[2 * i] = ba;
        if i < sci.stereo_bands as usize {
            ba = ba_code_tab[bs.get_bits(ba_bits) as usize];
        }
        sci.bitalloc[2 * i + 1] = if 0 != sci.stereo_bands { ba } else { 0 };
    }
    for (scfcod, bitalloc) in sci
        .scfcod
        .iter_mut()
        .zip(sci.bitalloc.iter())
        .take(sci.total_bands as usize * 2)
    {
        *scfcod = if 0 != *bitalloc {
            if header::is_layer_1(hdr) {
                2
            } else {
                bs.get_bits(2) as u8
            }
        } else {
            6
        }
    }
    unsafe {
        let mut bs_copy = bs.bs_copy();
        ffi::L12_read_scalefactors(
            &mut bs_copy as *mut _,
            sci.bitalloc.as_mut_ptr(),
            sci.scfcod.as_mut_ptr(),
            i32::from(sci.total_bands) * 2,
            sci.scf.as_mut_ptr(),
        );
        bs.position = bs_copy.pos as _;
        bs.limit = bs_copy.limit as _;
    }

    for i in sci.stereo_bands..sci.total_bands {
        sci.bitalloc[2 * i as usize + 1] = 0;
    }
}
