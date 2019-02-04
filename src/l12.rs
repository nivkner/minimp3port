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
    read_scalefactors(
        bs,
        &sci.bitalloc,
        &sci.scfcod,
        sci.total_bands as usize * 2,
        &mut sci.scf,
    );

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

fn read_scalefactors(
    bs: &mut BitStream<'_>,
    pba: &[u8],
    scfcod: &[u8],
    bands: usize,
    scf: &mut [f32],
) {
    #[rustfmt::skip]
    let g_deq_l12: [f32; 18 * 3] = [
        9.536_743e-7 / 3.0, 7.569_318e-7 / 3.0, 6.007_772e-7 / 3.0,
        9.536_743e-7 / 7.0, 7.569_318e-7 / 7.0, 6.007_772e-7 / 7.0,
        9.536_743e-7 / 15.0, 7.569_318e-7 / 15.0, 6.007_772e-7 / 15.0,
        9.536_743e-7 / 31.0, 7.569_318e-7 / 31.0, 6.007_772e-7 / 31.0,
        9.536_743e-7 / 63.0, 7.569_318e-7 / 63.0, 6.007_772e-7 / 63.0,
        9.536_743e-7 / 127.0, 7.569_318e-7 / 127.0, 6.007_772e-7 / 127.0,
        9.536_743e-7 / 255.0, 7.569_318e-7 / 255.0, 6.007_772e-7 / 255.0,
        9.536_743e-7 / 511.0, 7.569_318e-7 / 511.0, 6.007_772e-7 / 511.0,
        9.536_743e-7 / 1023.0, 7.569_318e-7 / 1023.0, 6.007_772e-7 / 1023.0,
        9.536_743e-7 / 2047.0, 7.569_318e-7 / 2047.0, 6.007_772e-7 / 2047.0,
        9.536_743e-7 / 4095.0, 7.569_318e-7 / 4095.0, 6.007_772e-7 / 4095.0,
        9.536_743e-7 / 8191.0, 7.569_318e-7 / 8191.0, 6.007_772e-7 / 8191.0,
        9.536_743e-7 / 16383.0, 7.569_318e-7 / 16383.0, 6.007_772e-7 / 16383.0,
        9.536_743e-7 / 32767.0, 7.569_318e-7 / 32767.0, 6.007_772e-7 / 32767.0,
        9.536_743e-7 / 65535.0, 7.569_318e-7 / 65535.0, 6.007_772e-7 / 65535.0,
        9.536_743e-7 / 3.0, 7.569_318e-7 / 3.0, 6.007_772e-7 / 3.0,
        9.536_743e-7 / 5.0, 7.569_318e-7 / 5.0, 6.007_772e-7 / 5.0,
        9.536_743e-7 / 9.0, 7.569_318e-7 / 9.0, 6.007_772e-7 / 9.0,
    ];
    for ((&ba, scf), scfcod) in pba
        .iter()
        .zip(scf.chunks_exact_mut(3))
        .zip(scfcod.iter())
        .take(bands)
    {
        let mask = if 0 != ba { 4 + (19 >> scfcod & 3) } else { 0 };

        let mut s = 0.0;
        for (m, scf) in scf.iter_mut().enumerate().map(|(i, scf)| (4 >> i, scf)) {
            if 0 != mask & m {
                let b = bs.get_bits(6) as u8;
                s = g_deq_l12[(ba * 3 - 6 + b % 3) as usize] * (1 << 21 >> (b / 3)) as f32
            }
            *scf = s;
        }
    }
}

pub fn dequantize_granule(
    grbuf: &mut [f32],
    bs: &mut BitStream<'_>,
    sci: &mut ffi::L12_scale_info,
    group_size: usize,
) -> usize {
    let mut choff: i32 = 576;
    for j in 0..4 {
        let dst = &mut grbuf[(group_size * j)..];
        // since choff alternates between 576 and -558
        // use an always growing offset to slice dst
        let mut offset = 0;
        for &ba in sci.bitalloc.iter().take(2 * sci.total_bands as usize) {
            let dst = &mut dst[(offset as usize)..];
            if ba != 0 {
                if ba < 17 {
                    let half: i32 = (1 << (ba - 1)) - 1;
                    for dst in dst.iter_mut().take(group_size) {
                        *dst = (bs.get_bits(ba.into()) as i32 - half) as f32;
                    }
                } else {
                    /* 3, 5, 9 */
                    let mod_0: i32 = (2 << (ba - 17)) + 1;
                    /* 5, 7, 10 */
                    let mut code: i32 = bs.get_bits((mod_0 + 2 - (mod_0 >> 3)) as u32) as i32;
                    for dst in dst.iter_mut().take(group_size) {
                        *dst = ((code % mod_0) - (mod_0 / 2)) as f32;
                        code /= mod_0;
                    }
                }
            }
            offset += choff;
            choff = 18 - choff;
        }
    }
    group_size * 4
}

pub fn apply_scf_384(sci: &mut ffi::L12_scale_info, scf_index: usize, dst: &mut [f32]) {
    let (dst_left, dst_right) = dst.split_at_mut(576);
    let stereo18 = sci.stereo_bands as usize * 18;
    let total18 = (sci.total_bands as usize) * 18;
    dst_right[stereo18..total18].copy_from_slice(&dst_left[stereo18..total18]);

    for ((scf, dst_left), dst_right) in sci
        .scf
        .chunks_exact(6)
        .zip(dst_left.chunks_exact_mut(18))
        .zip(dst_right.chunks_exact_mut(18))
        .take(sci.total_bands as usize)
    {
        for (dst_left, dst_right) in dst_left.iter_mut().zip(dst_right.iter_mut()).take(12) {
            *dst_left *= scf[scf_index];
            *dst_right *= scf[scf_index + 3];
        }
    }
}
