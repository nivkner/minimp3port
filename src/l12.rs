use crate::bits::BitStream;
use crate::{ffi, header};
use crate::{MODE_JOINT_STEREO, MODE_MONO};

use core::cmp;

pub fn read_scale_info(hdr: &[u8], bs: &mut BitStream<'_>, mut sci: &mut ffi::L12_scale_info) {
    let g_bitalloc_code_tab: [u8; 92] = [
        0, 17, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 0, 17, 18, 3, 19, 4, 5, 6, 7, 8, 9,
        10, 11, 12, 13, 16, 0, 17, 18, 3, 19, 4, 5, 16, 0, 17, 18, 16, 0, 17, 18, 19, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15, 0, 17, 18, 3, 19, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 2,
        3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ];
    let subband_alloc = subband_alloc_table(hdr, sci);
    let mut k = 0;
    let mut ba_bits = 0;
    let mut ba_code_tab = &g_bitalloc_code_tab as &[u8];
    let mut alloc_offset = 0;
    for i in 0..(sci.total_bands as usize) {
        if i == k {
            k += subband_alloc[alloc_offset].band_count as usize;
            ba_bits = u32::from(subband_alloc[alloc_offset].code_tab_width);
            ba_code_tab = &g_bitalloc_code_tab[(subband_alloc[alloc_offset].tab_offset as usize)..];
            alloc_offset += 1;
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

fn subband_alloc_table(
    hdr: &[u8],
    sci: &mut ffi::L12_scale_info,
) -> &'static [ffi::L12_subband_alloc_t] {
    static G_ALLOC_L1: &[ffi::L12_subband_alloc_t] = &[ffi::L12_subband_alloc_t {
        tab_offset: 76,
        code_tab_width: 4,
        band_count: 32,
    }];
    static G_ALLOC_L2M2: &[ffi::L12_subband_alloc_t] = &[
        ffi::L12_subband_alloc_t {
            tab_offset: 60,
            code_tab_width: 4,
            band_count: 4,
        },
        ffi::L12_subband_alloc_t {
            tab_offset: 44,
            code_tab_width: 3,
            band_count: 7,
        },
        ffi::L12_subband_alloc_t {
            tab_offset: 44,
            code_tab_width: 2,
            band_count: 19,
        },
    ];
    static G_ALLOC_L2M1_LOWRATE: &[ffi::L12_subband_alloc_t] = &[
        ffi::L12_subband_alloc_t {
            tab_offset: 44,
            code_tab_width: 4,
            band_count: 2,
        },
        ffi::L12_subband_alloc_t {
            tab_offset: 44,
            code_tab_width: 3,
            band_count: 10,
        },
    ];
    static G_ALLOC_L2M1: &[ffi::L12_subband_alloc_t] = &[
        ffi::L12_subband_alloc_t {
            tab_offset: 0,
            code_tab_width: 4,
            band_count: 3,
        },
        ffi::L12_subband_alloc_t {
            tab_offset: 16,
            code_tab_width: 4,
            band_count: 8,
        },
        ffi::L12_subband_alloc_t {
            tab_offset: 32,
            code_tab_width: 3,
            band_count: 12,
        },
        ffi::L12_subband_alloc_t {
            tab_offset: 40,
            code_tab_width: 2,
            band_count: 7,
        },
    ];
    let mode = header::get_stereo_mode(hdr);
    let stereo_bands = if mode == MODE_MONO {
        0
    } else if mode == MODE_JOINT_STEREO {
        (header::get_stereo_mode_ext(hdr) << 2) + 4
    } else {
        32
    };
    let (alloc, nbands) = if header::is_layer_1(hdr) {
        (&G_ALLOC_L1, 32)
    } else if !header::test_mpeg1(hdr) {
        (&G_ALLOC_L2M2, 30)
    } else {
        let sample_rate_idx = header::get_sample_rate(hdr);
        let mut kbps = header::bitrate_kbps(hdr) >> (mode != 3) as u8;
        /* free-format */
        if 0 == kbps {
            kbps = 192
        }
        if kbps < 56 {
            let nbands = if sample_rate_idx == 2 { 12 } else { 8 };
            (&G_ALLOC_L2M1_LOWRATE, nbands)
        } else if kbps >= 96 && sample_rate_idx != 1 {
            (&G_ALLOC_L2M1, 30)
        } else {
            (&G_ALLOC_L2M1, 27)
        }
    };
    sci.total_bands = nbands;
    sci.stereo_bands = cmp::min(stereo_bands, nbands);
    alloc
}
