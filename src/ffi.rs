#![allow(unused)]
#![allow(bad_style)]

fn wrapping_offset_from<T>(this: *const T, origin: *const T) -> isize {
    let pointee_size = ::core::mem::size_of::<T>();
    assert!(0 < pointee_size && pointee_size <= isize::max_value() as usize);

    let d = isize::wrapping_sub(this as _, origin as _);
    d.wrapping_div(pointee_size as _)
}

use libc;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
/*
    https://github.com/lieff/minimp3
    To the extent possible under law, the author(s) have dedicated all copyright and related and neighboring rights to this software to the public domain worldwide.
    This software is distributed without any warranty.
    See <http://creativecommons.org/publicdomain/zero/1.0/>.
*/
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct mp3dec_frame_info_t {
    pub frame_bytes: libc::c_int,
    pub channels: libc::c_int,
    pub hz: libc::c_int,
    pub layer: libc::c_int,
    pub bitrate_kbps: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp3dec_t {
    pub mdct_overlap: [[libc::c_float; 288]; 2],
    pub qmf_state: [libc::c_float; 960],
    pub reserv: libc::c_int,
    pub free_format_bytes: libc::c_int,
    pub header: [libc::c_uchar; 4],
    pub reserv_buf: [libc::c_uchar; 511],
}
pub type mp3d_sample_t = int16_t;
/* __cplusplus */
/* MINIMP3_H */
/* more than ISO spec's */
/* MAX_FRAME_SYNC_MATCHES */
/* MUST be >= 320000/8/32000*1152 = 1440 */
/* !defined(MINIMP3_NO_SIMD) */
/* !defined(MINIMP3_NO_SIMD) */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bs_t {
    pub buf: *const uint8_t,
    pub pos: libc::c_int,
    pub limit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp3dec_scratch_t {
    pub bs: bs_t,
    pub maindata: [uint8_t; 2815],
    pub gr_info: [L3_gr_info_t; 4],
    pub grbuf: [[libc::c_float; 576]; 2],
    pub scf: [libc::c_float; 40],
    pub syn: [[libc::c_float; 64]; 33],
    pub ist_pos: [[uint8_t; 39]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct L3_gr_info_t {
    pub sfbtab: *const uint8_t,
    pub part_23_length: uint16_t,
    pub big_values: uint16_t,
    pub scalefac_compress: uint16_t,
    pub global_gain: uint8_t,
    pub block_type: uint8_t,
    pub mixed_block_flag: uint8_t,
    pub n_long_sfb: uint8_t,
    pub n_short_sfb: uint8_t,
    pub table_select: [uint8_t; 3],
    pub region_count: [uint8_t; 3],
    pub subblock_gain: [uint8_t; 3],
    pub preflag: uint8_t,
    pub scalefac_scale: uint8_t,
    pub count1_table: uint8_t,
    pub scfsi: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct L12_scale_info {
    pub scf: [libc::c_float; 192],
    pub total_bands: uint8_t,
    pub stereo_bands: uint8_t,
    pub bitalloc: [uint8_t; 64],
    pub scfcod: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct L12_subband_alloc_t {
    pub tab_offset: uint8_t,
    pub code_tab_width: uint8_t,
    pub band_count: uint8_t,
}
/* __cplusplus */
pub unsafe fn mp3dec_init(mut dec: *mut mp3dec_t) {
    (*dec).header[0usize] = 0i32 as libc::c_uchar;
}
/* MINIMP3_FLOAT_OUTPUT */
/* MINIMP3_FLOAT_OUTPUT */
pub unsafe fn mp3dec_decode_frame(
    mut dec: *mut mp3dec_t,
    mut mp3: *const uint8_t,
    mut mp3_bytes: libc::c_int,
    mut pcm: *mut mp3d_sample_t,
    mut info: *mut mp3dec_frame_info_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0i32;
    let mut igr: libc::c_int = 0;
    let mut frame_size: libc::c_int = 0i32;
    let mut success: libc::c_int = 1i32;
    let mut hdr: *const uint8_t = 0 as *const uint8_t;
    let mut bs_frame: [bs_t; 1] = [bs_t {
        buf: 0 as *const uint8_t,
        pos: 0,
        limit: 0,
    }; 1];
    let mut scratch: mp3dec_scratch_t = mp3dec_scratch_t {
        bs: bs_t {
            buf: 0 as *const uint8_t,
            pos: 0,
            limit: 0,
        },
        maindata: [0; 2815],
        gr_info: [L3_gr_info_t {
            sfbtab: 0 as *const uint8_t,
            part_23_length: 0,
            big_values: 0,
            scalefac_compress: 0,
            global_gain: 0,
            block_type: 0,
            mixed_block_flag: 0,
            n_long_sfb: 0,
            n_short_sfb: 0,
            table_select: [0; 3],
            region_count: [0; 3],
            subblock_gain: [0; 3],
            preflag: 0,
            scalefac_scale: 0,
            count1_table: 0,
            scfsi: 0,
        }; 4],
        grbuf: [[0.; 576]; 2],
        scf: [0.; 40],
        syn: [[0.; 64]; 33],
        ist_pos: [[0; 39]; 2],
    };
    if mp3_bytes > 4i32
        && (*dec).header[0usize] as libc::c_int == 0xffi32
        && 0 != hdr_compare((*dec).header.as_mut_ptr(), mp3)
    {
        frame_size = hdr_frame_bytes(mp3, (*dec).free_format_bytes) + hdr_padding(mp3);
        if frame_size != mp3_bytes
            && (frame_size + 4i32 > mp3_bytes
                || 0 == hdr_compare(mp3, mp3.offset(frame_size as isize)))
        {
            frame_size = 0i32
        }
    }
    if 0 == frame_size {
        memset(
            dec as *mut libc::c_void,
            0i32,
            ::core::mem::size_of::<mp3dec_t>() as libc::c_ulong,
        );
        i = mp3d_find_frame(
            mp3,
            mp3_bytes,
            &mut (*dec).free_format_bytes,
            &mut frame_size,
        );
        if 0 == frame_size || i + frame_size > mp3_bytes {
            (*info).frame_bytes = i;
            return 0i32;
        }
    }
    hdr = mp3.offset(i as isize);
    memcpy(
        (*dec).header.as_mut_ptr() as *mut libc::c_void,
        hdr as *const libc::c_void,
        4i32 as libc::c_ulong,
    );
    (*info).frame_bytes = i + frame_size;
    (*info).channels = if *hdr.offset(3isize) as libc::c_int & 0xc0i32 == 0xc0i32 {
        1i32
    } else {
        2i32
    };
    (*info).hz = hdr_sample_rate_hz(hdr) as libc::c_int;
    (*info).layer = 4i32 - (*hdr.offset(1isize) as libc::c_int >> 1i32 & 3i32);
    (*info).bitrate_kbps = hdr_bitrate_kbps(hdr) as libc::c_int;
    if pcm.is_null() {
        return hdr_frame_samples(hdr) as libc::c_int;
    } else {
        bs_init(bs_frame.as_mut_ptr(), hdr.offset(4isize), frame_size - 4i32);
        if 0 == *hdr.offset(1isize) as libc::c_int & 1i32 {
            get_bits(bs_frame.as_mut_ptr(), 16i32);
        }
        if (*info).layer == 3i32 {
            let mut main_data_begin: libc::c_int =
                L3_read_side_info(bs_frame.as_mut_ptr(), scratch.gr_info.as_mut_ptr(), hdr);
            if main_data_begin < 0i32
                || (*bs_frame.as_mut_ptr()).pos > (*bs_frame.as_mut_ptr()).limit
            {
                mp3dec_init(dec);
                return 0i32;
            } else {
                success =
                    L3_restore_reservoir(dec, bs_frame.as_mut_ptr(), &mut scratch, main_data_begin);
                if 0 != success {
                    igr = 0i32;
                    while igr < if 0 != *hdr.offset(1isize) as libc::c_int & 0x8i32 {
                        2i32
                    } else {
                        1i32
                    } {
                        memset(
                            scratch.grbuf[0usize].as_mut_ptr() as *mut libc::c_void,
                            0i32,
                            ((576i32 * 2i32) as libc::c_ulong).wrapping_mul(::core::mem::size_of::<
                                libc::c_float,
                            >(
                            )
                                as libc::c_ulong),
                        );
                        L3_decode(
                            dec,
                            &mut scratch,
                            scratch
                                .gr_info
                                .as_mut_ptr()
                                .offset((igr * (*info).channels) as isize),
                            (*info).channels,
                        );
                        mp3d_synth_granule(
                            (*dec).qmf_state.as_mut_ptr(),
                            scratch.grbuf[0usize].as_mut_ptr(),
                            18i32,
                            (*info).channels,
                            pcm,
                            scratch.syn[0usize].as_mut_ptr(),
                        );
                        igr += 1;
                        pcm = pcm.offset((576i32 * (*info).channels) as isize)
                    }
                }
                L3_save_reservoir(dec, &mut scratch);
            }
        } else {
            /* MINIMP3_ONLY_MP3 */
            let mut sci: [L12_scale_info; 1] = [L12_scale_info {
                scf: [0.; 192],
                total_bands: 0,
                stereo_bands: 0,
                bitalloc: [0; 64],
                scfcod: [0; 64],
            }; 1];
            L12_read_scale_info(hdr, bs_frame.as_mut_ptr(), sci.as_mut_ptr());
            memset(
                scratch.grbuf[0usize].as_mut_ptr() as *mut libc::c_void,
                0i32,
                ((576i32 * 2i32) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
            );
            i = 0i32;
            igr = 0i32;
            while igr < 3i32 {
                i += L12_dequantize_granule(
                    scratch.grbuf[0usize].as_mut_ptr().offset(i as isize),
                    bs_frame.as_mut_ptr(),
                    sci.as_mut_ptr(),
                    (*info).layer | 1i32,
                );
                if 12i32 == i {
                    i = 0i32;
                    L12_apply_scf_384(
                        sci.as_mut_ptr(),
                        (*sci.as_mut_ptr()).scf.as_mut_ptr().offset(igr as isize),
                        scratch.grbuf[0usize].as_mut_ptr(),
                    );
                    mp3d_synth_granule(
                        (*dec).qmf_state.as_mut_ptr(),
                        scratch.grbuf[0usize].as_mut_ptr(),
                        12i32,
                        (*info).channels,
                        pcm,
                        scratch.syn[0usize].as_mut_ptr(),
                    );
                    memset(
                        scratch.grbuf[0usize].as_mut_ptr() as *mut libc::c_void,
                        0i32,
                        ((576i32 * 2i32) as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
                    );
                    pcm = pcm.offset((384i32 * (*info).channels) as isize)
                }
                if (*bs_frame.as_mut_ptr()).pos > (*bs_frame.as_mut_ptr()).limit {
                    mp3dec_init(dec);
                    return 0i32;
                } else {
                    igr += 1
                }
            }
        }
        /* MINIMP3_ONLY_MP3 */
        return (success as libc::c_uint).wrapping_mul(hdr_frame_samples((*dec).header.as_mut_ptr()))
            as libc::c_int;
    };
}
pub unsafe fn hdr_frame_samples(mut h: *const uint8_t) -> libc::c_uint {
    return (if *h.offset(1isize) as libc::c_int & 6i32 == 6i32 {
        384i32
    } else {
        1152i32 >> (*h.offset(1isize) as libc::c_int & 14i32 == 2i32) as libc::c_int
    }) as libc::c_uint;
}
/* MINIMP3_ONLY_SIMD */
pub unsafe fn mp3d_synth_granule(
    mut qmf_state: *mut libc::c_float,
    mut grbuf: *mut libc::c_float,
    mut nbands: libc::c_int,
    mut nch: libc::c_int,
    mut pcm: *mut mp3d_sample_t,
    mut lins: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < nch {
        mp3d_DCT_II(grbuf.offset((576i32 * i) as isize), nbands);
        i += 1
    }
    memcpy(
        lins as *mut libc::c_void,
        qmf_state as *const libc::c_void,
        (::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(15i32 as libc::c_ulong)
            .wrapping_mul(64i32 as libc::c_ulong),
    );
    i = 0i32;
    while i < nbands {
        mp3d_synth(
            grbuf.offset(i as isize),
            pcm.offset((32i32 * nch * i) as isize),
            nch,
            lins.offset((i * 64i32) as isize),
        );
        i += 2i32
    }
    if nch == 1i32 {
        i = 0i32;
        while i < 15i32 * 64i32 {
            *qmf_state.offset(i as isize) = *lins.offset((nbands * 64i32 + i) as isize);
            i += 2i32
        }
    } else {
        /* MINIMP3_NONSTANDARD_BUT_LOGICAL */
        memcpy(
            qmf_state as *mut libc::c_void,
            lins.offset((nbands * 64i32) as isize) as *const libc::c_void,
            (::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(15i32 as libc::c_ulong)
                .wrapping_mul(64i32 as libc::c_ulong),
        );
    };
}
pub unsafe fn mp3d_synth(
    mut xl: *mut libc::c_float,
    mut dstl: *mut mp3d_sample_t,
    mut nch: libc::c_int,
    mut lins: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut xr: *mut libc::c_float = xl.offset((576i32 * (nch - 1i32)) as isize);
    let mut dstr: *mut mp3d_sample_t = dstl.offset((nch - 1i32) as isize);
    static mut g_win: [libc::c_float; 240] = [
        -1i32 as libc::c_float,
        26i32 as libc::c_float,
        -31i32 as libc::c_float,
        208i32 as libc::c_float,
        218i32 as libc::c_float,
        401i32 as libc::c_float,
        -519i32 as libc::c_float,
        2063i32 as libc::c_float,
        2000i32 as libc::c_float,
        4788i32 as libc::c_float,
        -5517i32 as libc::c_float,
        7134i32 as libc::c_float,
        5959i32 as libc::c_float,
        35640i32 as libc::c_float,
        -39336i32 as libc::c_float,
        74992i32 as libc::c_float,
        -1i32 as libc::c_float,
        24i32 as libc::c_float,
        -35i32 as libc::c_float,
        202i32 as libc::c_float,
        222i32 as libc::c_float,
        347i32 as libc::c_float,
        -581i32 as libc::c_float,
        2080i32 as libc::c_float,
        1952i32 as libc::c_float,
        4425i32 as libc::c_float,
        -5879i32 as libc::c_float,
        7640i32 as libc::c_float,
        5288i32 as libc::c_float,
        33791i32 as libc::c_float,
        -41176i32 as libc::c_float,
        74856i32 as libc::c_float,
        -1i32 as libc::c_float,
        21i32 as libc::c_float,
        -38i32 as libc::c_float,
        196i32 as libc::c_float,
        225i32 as libc::c_float,
        294i32 as libc::c_float,
        -645i32 as libc::c_float,
        2087i32 as libc::c_float,
        1893i32 as libc::c_float,
        4063i32 as libc::c_float,
        -6237i32 as libc::c_float,
        8092i32 as libc::c_float,
        4561i32 as libc::c_float,
        31947i32 as libc::c_float,
        -43006i32 as libc::c_float,
        74630i32 as libc::c_float,
        -1i32 as libc::c_float,
        19i32 as libc::c_float,
        -41i32 as libc::c_float,
        190i32 as libc::c_float,
        227i32 as libc::c_float,
        244i32 as libc::c_float,
        -711i32 as libc::c_float,
        2085i32 as libc::c_float,
        1822i32 as libc::c_float,
        3705i32 as libc::c_float,
        -6589i32 as libc::c_float,
        8492i32 as libc::c_float,
        3776i32 as libc::c_float,
        30112i32 as libc::c_float,
        -44821i32 as libc::c_float,
        74313i32 as libc::c_float,
        -1i32 as libc::c_float,
        17i32 as libc::c_float,
        -45i32 as libc::c_float,
        183i32 as libc::c_float,
        228i32 as libc::c_float,
        197i32 as libc::c_float,
        -779i32 as libc::c_float,
        2075i32 as libc::c_float,
        1739i32 as libc::c_float,
        3351i32 as libc::c_float,
        -6935i32 as libc::c_float,
        8840i32 as libc::c_float,
        2935i32 as libc::c_float,
        28289i32 as libc::c_float,
        -46617i32 as libc::c_float,
        73908i32 as libc::c_float,
        -1i32 as libc::c_float,
        16i32 as libc::c_float,
        -49i32 as libc::c_float,
        176i32 as libc::c_float,
        228i32 as libc::c_float,
        153i32 as libc::c_float,
        -848i32 as libc::c_float,
        2057i32 as libc::c_float,
        1644i32 as libc::c_float,
        3004i32 as libc::c_float,
        -7271i32 as libc::c_float,
        9139i32 as libc::c_float,
        2037i32 as libc::c_float,
        26482i32 as libc::c_float,
        -48390i32 as libc::c_float,
        73415i32 as libc::c_float,
        -2i32 as libc::c_float,
        14i32 as libc::c_float,
        -53i32 as libc::c_float,
        169i32 as libc::c_float,
        227i32 as libc::c_float,
        111i32 as libc::c_float,
        -919i32 as libc::c_float,
        2032i32 as libc::c_float,
        1535i32 as libc::c_float,
        2663i32 as libc::c_float,
        -7597i32 as libc::c_float,
        9389i32 as libc::c_float,
        1082i32 as libc::c_float,
        24694i32 as libc::c_float,
        -50137i32 as libc::c_float,
        72835i32 as libc::c_float,
        -2i32 as libc::c_float,
        13i32 as libc::c_float,
        -58i32 as libc::c_float,
        161i32 as libc::c_float,
        224i32 as libc::c_float,
        72i32 as libc::c_float,
        -991i32 as libc::c_float,
        2001i32 as libc::c_float,
        1414i32 as libc::c_float,
        2330i32 as libc::c_float,
        -7910i32 as libc::c_float,
        9592i32 as libc::c_float,
        70i32 as libc::c_float,
        22929i32 as libc::c_float,
        -51853i32 as libc::c_float,
        72169i32 as libc::c_float,
        -2i32 as libc::c_float,
        11i32 as libc::c_float,
        -63i32 as libc::c_float,
        154i32 as libc::c_float,
        221i32 as libc::c_float,
        36i32 as libc::c_float,
        -1064i32 as libc::c_float,
        1962i32 as libc::c_float,
        1280i32 as libc::c_float,
        2006i32 as libc::c_float,
        -8209i32 as libc::c_float,
        9750i32 as libc::c_float,
        -998i32 as libc::c_float,
        21189i32 as libc::c_float,
        -53534i32 as libc::c_float,
        71420i32 as libc::c_float,
        -2i32 as libc::c_float,
        10i32 as libc::c_float,
        -68i32 as libc::c_float,
        147i32 as libc::c_float,
        215i32 as libc::c_float,
        2i32 as libc::c_float,
        -1137i32 as libc::c_float,
        1919i32 as libc::c_float,
        1131i32 as libc::c_float,
        1692i32 as libc::c_float,
        -8491i32 as libc::c_float,
        9863i32 as libc::c_float,
        -2122i32 as libc::c_float,
        19478i32 as libc::c_float,
        -55178i32 as libc::c_float,
        70590i32 as libc::c_float,
        -3i32 as libc::c_float,
        9i32 as libc::c_float,
        -73i32 as libc::c_float,
        139i32 as libc::c_float,
        208i32 as libc::c_float,
        -29i32 as libc::c_float,
        -1210i32 as libc::c_float,
        1870i32 as libc::c_float,
        970i32 as libc::c_float,
        1388i32 as libc::c_float,
        -8755i32 as libc::c_float,
        9935i32 as libc::c_float,
        -3300i32 as libc::c_float,
        17799i32 as libc::c_float,
        -56778i32 as libc::c_float,
        69679i32 as libc::c_float,
        -3i32 as libc::c_float,
        8i32 as libc::c_float,
        -79i32 as libc::c_float,
        132i32 as libc::c_float,
        200i32 as libc::c_float,
        -57i32 as libc::c_float,
        -1283i32 as libc::c_float,
        1817i32 as libc::c_float,
        794i32 as libc::c_float,
        1095i32 as libc::c_float,
        -8998i32 as libc::c_float,
        9966i32 as libc::c_float,
        -4533i32 as libc::c_float,
        16155i32 as libc::c_float,
        -58333i32 as libc::c_float,
        68692i32 as libc::c_float,
        -4i32 as libc::c_float,
        7i32 as libc::c_float,
        -85i32 as libc::c_float,
        125i32 as libc::c_float,
        189i32 as libc::c_float,
        -83i32 as libc::c_float,
        -1356i32 as libc::c_float,
        1759i32 as libc::c_float,
        605i32 as libc::c_float,
        814i32 as libc::c_float,
        -9219i32 as libc::c_float,
        9959i32 as libc::c_float,
        -5818i32 as libc::c_float,
        14548i32 as libc::c_float,
        -59838i32 as libc::c_float,
        67629i32 as libc::c_float,
        -4i32 as libc::c_float,
        7i32 as libc::c_float,
        -91i32 as libc::c_float,
        117i32 as libc::c_float,
        177i32 as libc::c_float,
        -106i32 as libc::c_float,
        -1428i32 as libc::c_float,
        1698i32 as libc::c_float,
        402i32 as libc::c_float,
        545i32 as libc::c_float,
        -9416i32 as libc::c_float,
        9916i32 as libc::c_float,
        -7154i32 as libc::c_float,
        12980i32 as libc::c_float,
        -61289i32 as libc::c_float,
        66494i32 as libc::c_float,
        -5i32 as libc::c_float,
        6i32 as libc::c_float,
        -97i32 as libc::c_float,
        111i32 as libc::c_float,
        163i32 as libc::c_float,
        -127i32 as libc::c_float,
        -1498i32 as libc::c_float,
        1634i32 as libc::c_float,
        185i32 as libc::c_float,
        288i32 as libc::c_float,
        -9585i32 as libc::c_float,
        9838i32 as libc::c_float,
        -8540i32 as libc::c_float,
        11455i32 as libc::c_float,
        -62684i32 as libc::c_float,
        65290i32 as libc::c_float,
    ];
    let mut zlin: *mut libc::c_float = lins.offset((15i32 * 64i32) as isize);
    let mut w: *const libc::c_float = g_win.as_ptr();
    *zlin.offset((4i32 * 15i32) as isize) = *xl.offset((18i32 * 16i32) as isize);
    *zlin.offset((4i32 * 15i32 + 1i32) as isize) = *xr.offset((18i32 * 16i32) as isize);
    *zlin.offset((4i32 * 15i32 + 2i32) as isize) = *xl.offset(0isize);
    *zlin.offset((4i32 * 15i32 + 3i32) as isize) = *xr.offset(0isize);
    *zlin.offset((4i32 * 31i32) as isize) = *xl.offset((1i32 + 18i32 * 16i32) as isize);
    *zlin.offset((4i32 * 31i32 + 1i32) as isize) = *xr.offset((1i32 + 18i32 * 16i32) as isize);
    *zlin.offset((4i32 * 31i32 + 2i32) as isize) = *xl.offset(1isize);
    *zlin.offset((4i32 * 31i32 + 3i32) as isize) = *xr.offset(1isize);
    mp3d_synth_pair(
        dstr,
        nch,
        lins.offset((4i32 * 15i32) as isize).offset(1isize),
    );
    mp3d_synth_pair(
        dstr.offset((32i32 * nch) as isize),
        nch,
        lins.offset((4i32 * 15i32) as isize)
            .offset(64isize)
            .offset(1isize),
    );
    mp3d_synth_pair(dstl, nch, lins.offset((4i32 * 15i32) as isize));
    mp3d_synth_pair(
        dstl.offset((32i32 * nch) as isize),
        nch,
        lins.offset((4i32 * 15i32) as isize).offset(64isize),
    );
    /* HAVE_SIMD */
    /* MINIMP3_ONLY_SIMD */
    i = 14i32;
    while i >= 0i32 {
        let mut a: [libc::c_float; 4] = [0.; 4];
        let mut b: [libc::c_float; 4] = [0.; 4];
        *zlin.offset((4i32 * i) as isize) = *xl.offset((18i32 * (31i32 - i)) as isize);
        *zlin.offset((4i32 * i + 1i32) as isize) = *xr.offset((18i32 * (31i32 - i)) as isize);
        *zlin.offset((4i32 * i + 2i32) as isize) =
            *xl.offset((1i32 + 18i32 * (31i32 - i)) as isize);
        *zlin.offset((4i32 * i + 3i32) as isize) =
            *xr.offset((1i32 + 18i32 * (31i32 - i)) as isize);
        *zlin.offset((4i32 * (i + 16i32)) as isize) =
            *xl.offset((1i32 + 18i32 * (1i32 + i)) as isize);
        *zlin.offset((4i32 * (i + 16i32) + 1i32) as isize) =
            *xr.offset((1i32 + 18i32 * (1i32 + i)) as isize);
        *zlin.offset((4i32 * (i - 16i32) + 2i32) as isize) =
            *xl.offset((18i32 * (1i32 + i)) as isize);
        *zlin.offset((4i32 * (i - 16i32) + 3i32) as isize) =
            *xr.offset((18i32 * (1i32 + i)) as isize);
        let mut j: libc::c_int = 0;
        let fresh0 = w;
        w = w.offset(1);
        let mut w0: libc::c_float = *fresh0;
        let fresh1 = w;
        w = w.offset(1);
        let mut w1: libc::c_float = *fresh1;
        let mut vz: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - 0i32 * 64i32) as isize) as *mut libc::c_float;
        let mut vy: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - (15i32 - 0i32) * 64i32) as isize) as *mut libc::c_float;
        j = 0i32;
        while j < 4i32 {
            b[j as usize] = *vz.offset(j as isize) * w1 + *vy.offset(j as isize) * w0;
            a[j as usize] = *vz.offset(j as isize) * w0 - *vy.offset(j as isize) * w1;
            j += 1
        }
        let mut j_0: libc::c_int = 0;
        let fresh2 = w;
        w = w.offset(1);
        let mut w0_0: libc::c_float = *fresh2;
        let fresh3 = w;
        w = w.offset(1);
        let mut w1_0: libc::c_float = *fresh3;
        let mut vz_0: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - 1i32 * 64i32) as isize) as *mut libc::c_float;
        let mut vy_0: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - (15i32 - 1i32) * 64i32) as isize) as *mut libc::c_float;
        j_0 = 0i32;
        while j_0 < 4i32 {
            b[j_0 as usize] +=
                *vz_0.offset(j_0 as isize) * w1_0 + *vy_0.offset(j_0 as isize) * w0_0;
            a[j_0 as usize] +=
                *vy_0.offset(j_0 as isize) * w1_0 - *vz_0.offset(j_0 as isize) * w0_0;
            j_0 += 1
        }
        let mut j_1: libc::c_int = 0;
        let fresh4 = w;
        w = w.offset(1);
        let mut w0_1: libc::c_float = *fresh4;
        let fresh5 = w;
        w = w.offset(1);
        let mut w1_1: libc::c_float = *fresh5;
        let mut vz_1: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - 2i32 * 64i32) as isize) as *mut libc::c_float;
        let mut vy_1: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - (15i32 - 2i32) * 64i32) as isize) as *mut libc::c_float;
        j_1 = 0i32;
        while j_1 < 4i32 {
            b[j_1 as usize] +=
                *vz_1.offset(j_1 as isize) * w1_1 + *vy_1.offset(j_1 as isize) * w0_1;
            a[j_1 as usize] +=
                *vz_1.offset(j_1 as isize) * w0_1 - *vy_1.offset(j_1 as isize) * w1_1;
            j_1 += 1
        }
        let mut j_2: libc::c_int = 0;
        let fresh6 = w;
        w = w.offset(1);
        let mut w0_2: libc::c_float = *fresh6;
        let fresh7 = w;
        w = w.offset(1);
        let mut w1_2: libc::c_float = *fresh7;
        let mut vz_2: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - 3i32 * 64i32) as isize) as *mut libc::c_float;
        let mut vy_2: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - (15i32 - 3i32) * 64i32) as isize) as *mut libc::c_float;
        j_2 = 0i32;
        while j_2 < 4i32 {
            b[j_2 as usize] +=
                *vz_2.offset(j_2 as isize) * w1_2 + *vy_2.offset(j_2 as isize) * w0_2;
            a[j_2 as usize] +=
                *vy_2.offset(j_2 as isize) * w1_2 - *vz_2.offset(j_2 as isize) * w0_2;
            j_2 += 1
        }
        let mut j_3: libc::c_int = 0;
        let fresh8 = w;
        w = w.offset(1);
        let mut w0_3: libc::c_float = *fresh8;
        let fresh9 = w;
        w = w.offset(1);
        let mut w1_3: libc::c_float = *fresh9;
        let mut vz_3: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - 4i32 * 64i32) as isize) as *mut libc::c_float;
        let mut vy_3: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - (15i32 - 4i32) * 64i32) as isize) as *mut libc::c_float;
        j_3 = 0i32;
        while j_3 < 4i32 {
            b[j_3 as usize] +=
                *vz_3.offset(j_3 as isize) * w1_3 + *vy_3.offset(j_3 as isize) * w0_3;
            a[j_3 as usize] +=
                *vz_3.offset(j_3 as isize) * w0_3 - *vy_3.offset(j_3 as isize) * w1_3;
            j_3 += 1
        }
        let mut j_4: libc::c_int = 0;
        let fresh10 = w;
        w = w.offset(1);
        let mut w0_4: libc::c_float = *fresh10;
        let fresh11 = w;
        w = w.offset(1);
        let mut w1_4: libc::c_float = *fresh11;
        let mut vz_4: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - 5i32 * 64i32) as isize) as *mut libc::c_float;
        let mut vy_4: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - (15i32 - 5i32) * 64i32) as isize) as *mut libc::c_float;
        j_4 = 0i32;
        while j_4 < 4i32 {
            b[j_4 as usize] +=
                *vz_4.offset(j_4 as isize) * w1_4 + *vy_4.offset(j_4 as isize) * w0_4;
            a[j_4 as usize] +=
                *vy_4.offset(j_4 as isize) * w1_4 - *vz_4.offset(j_4 as isize) * w0_4;
            j_4 += 1
        }
        let mut j_5: libc::c_int = 0;
        let fresh12 = w;
        w = w.offset(1);
        let mut w0_5: libc::c_float = *fresh12;
        let fresh13 = w;
        w = w.offset(1);
        let mut w1_5: libc::c_float = *fresh13;
        let mut vz_5: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - 6i32 * 64i32) as isize) as *mut libc::c_float;
        let mut vy_5: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - (15i32 - 6i32) * 64i32) as isize) as *mut libc::c_float;
        j_5 = 0i32;
        while j_5 < 4i32 {
            b[j_5 as usize] +=
                *vz_5.offset(j_5 as isize) * w1_5 + *vy_5.offset(j_5 as isize) * w0_5;
            a[j_5 as usize] +=
                *vz_5.offset(j_5 as isize) * w0_5 - *vy_5.offset(j_5 as isize) * w1_5;
            j_5 += 1
        }
        let mut j_6: libc::c_int = 0;
        let fresh14 = w;
        w = w.offset(1);
        let mut w0_6: libc::c_float = *fresh14;
        let fresh15 = w;
        w = w.offset(1);
        let mut w1_6: libc::c_float = *fresh15;
        let mut vz_6: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - 7i32 * 64i32) as isize) as *mut libc::c_float;
        let mut vy_6: *mut libc::c_float =
            &mut *zlin.offset((4i32 * i - (15i32 - 7i32) * 64i32) as isize) as *mut libc::c_float;
        j_6 = 0i32;
        while j_6 < 4i32 {
            b[j_6 as usize] +=
                *vz_6.offset(j_6 as isize) * w1_6 + *vy_6.offset(j_6 as isize) * w0_6;
            a[j_6 as usize] +=
                *vy_6.offset(j_6 as isize) * w1_6 - *vz_6.offset(j_6 as isize) * w0_6;
            j_6 += 1
        }
        *dstr.offset(((15i32 - i) * nch) as isize) = mp3d_scale_pcm(a[1usize]);
        *dstr.offset(((17i32 + i) * nch) as isize) = mp3d_scale_pcm(b[1usize]);
        *dstl.offset(((15i32 - i) * nch) as isize) = mp3d_scale_pcm(a[0usize]);
        *dstl.offset(((17i32 + i) * nch) as isize) = mp3d_scale_pcm(b[0usize]);
        *dstr.offset(((47i32 - i) * nch) as isize) = mp3d_scale_pcm(a[3usize]);
        *dstr.offset(((49i32 + i) * nch) as isize) = mp3d_scale_pcm(b[3usize]);
        *dstl.offset(((47i32 - i) * nch) as isize) = mp3d_scale_pcm(a[2usize]);
        *dstl.offset(((49i32 + i) * nch) as isize) = mp3d_scale_pcm(b[2usize]);
        i -= 1
    }
}
/* MINIMP3_ONLY_SIMD */
pub unsafe fn mp3d_scale_pcm(mut sample: libc::c_float) -> int16_t {
    if sample as libc::c_double >= 32766.5f64 {
        return 32767i32 as int16_t;
    } else if sample as libc::c_double <= -32767.5f64 {
        return -32768i32 as int16_t;
    } else {
        let mut s: int16_t = (sample + 0.5f32) as int16_t;
        /* away from zero, to be compliant */
        s = (s as libc::c_int - ((s as libc::c_int) < 0i32) as libc::c_int) as int16_t;
        return s;
    };
}
/* MINIMP3_FLOAT_OUTPUT */
/* MINIMP3_FLOAT_OUTPUT */
pub unsafe fn mp3d_synth_pair(
    mut pcm: *mut mp3d_sample_t,
    mut nch: libc::c_int,
    mut z: *const libc::c_float,
) {
    let mut a: libc::c_float = 0.;
    a = (*z.offset((14i32 * 64i32) as isize) - *z.offset(0isize)) * 29i32 as libc::c_float;
    a += (*z.offset((1i32 * 64i32) as isize) + *z.offset((13i32 * 64i32) as isize))
        * 213i32 as libc::c_float;
    a += (*z.offset((12i32 * 64i32) as isize) - *z.offset((2i32 * 64i32) as isize))
        * 459i32 as libc::c_float;
    a += (*z.offset((3i32 * 64i32) as isize) + *z.offset((11i32 * 64i32) as isize))
        * 2037i32 as libc::c_float;
    a += (*z.offset((10i32 * 64i32) as isize) - *z.offset((4i32 * 64i32) as isize))
        * 5153i32 as libc::c_float;
    a += (*z.offset((5i32 * 64i32) as isize) + *z.offset((9i32 * 64i32) as isize))
        * 6574i32 as libc::c_float;
    a += (*z.offset((8i32 * 64i32) as isize) - *z.offset((6i32 * 64i32) as isize))
        * 37489i32 as libc::c_float;
    a += *z.offset((7i32 * 64i32) as isize) * 75038i32 as libc::c_float;
    *pcm.offset(0isize) = mp3d_scale_pcm(a);
    z = z.offset(2isize);
    a = *z.offset((14i32 * 64i32) as isize) * 104i32 as libc::c_float;
    a += *z.offset((12i32 * 64i32) as isize) * 1567i32 as libc::c_float;
    a += *z.offset((10i32 * 64i32) as isize) * 9727i32 as libc::c_float;
    a += *z.offset((8i32 * 64i32) as isize) * 64019i32 as libc::c_float;
    a += *z.offset((6i32 * 64i32) as isize) * -9975i32 as libc::c_float;
    a += *z.offset((4i32 * 64i32) as isize) * -45i32 as libc::c_float;
    a += *z.offset((2i32 * 64i32) as isize) * 146i32 as libc::c_float;
    a += *z.offset((0i32 * 64i32) as isize) * -5i32 as libc::c_float;
    *pcm.offset((16i32 * nch) as isize) = mp3d_scale_pcm(a);
}
pub unsafe fn mp3d_DCT_II(mut grbuf: *mut libc::c_float, mut n: libc::c_int) {
    static mut g_sec: [libc::c_float; 24] = [
        10.190008163452149f32,
        0.5006030201911926f32,
        0.5024192929267883f32,
        3.4076085090637209f32,
        0.5054709315299988f32,
        0.522498607635498f32,
        2.0577809810638429f32,
        0.5154473185539246f32,
        0.5669440627098084f32,
        1.4841645956039429f32,
        0.5310425758361816f32,
        0.6468217968940735f32,
        1.1694399118423463f32,
        0.5531039237976074f32,
        0.7881546020507813f32,
        0.9725682139396668f32,
        0.5829349756240845f32,
        1.0606776475906373f32,
        0.839349627494812f32,
        0.6225041151046753f32,
        1.722447156906128f32,
        0.744536280632019f32,
        0.6748083233833313f32,
        5.10114860534668f32,
    ];
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0i32;
    /* HAVE_SIMD */
    /* MINIMP3_ONLY_SIMD */
    while k < n {
        let mut t: [[libc::c_float; 8]; 4] = [[0.; 8]; 4];
        let mut x: *mut libc::c_float = 0 as *mut libc::c_float;
        let mut y: *mut libc::c_float = grbuf.offset(k as isize);
        x = t[0usize].as_mut_ptr();
        i = 0i32;
        while i < 8i32 {
            let mut x0: libc::c_float = *y.offset((i * 18i32) as isize);
            let mut x1: libc::c_float = *y.offset(((15i32 - i) * 18i32) as isize);
            let mut x2: libc::c_float = *y.offset(((16i32 + i) * 18i32) as isize);
            let mut x3: libc::c_float = *y.offset(((31i32 - i) * 18i32) as isize);
            let mut t0: libc::c_float = x0 + x3;
            let mut t1: libc::c_float = x1 + x2;
            let mut t2: libc::c_float = (x1 - x2) * g_sec[(3i32 * i + 0i32) as usize];
            let mut t3: libc::c_float = (x0 - x3) * g_sec[(3i32 * i + 1i32) as usize];
            *x.offset(0isize) = t0 + t1;
            *x.offset(8isize) = (t0 - t1) * g_sec[(3i32 * i + 2i32) as usize];
            *x.offset(16isize) = t3 + t2;
            *x.offset(24isize) = (t3 - t2) * g_sec[(3i32 * i + 2i32) as usize];
            i += 1;
            x = x.offset(1isize)
        }
        x = t[0usize].as_mut_ptr();
        i = 0i32;
        while i < 4i32 {
            let mut x0_0: libc::c_float = *x.offset(0isize);
            let mut x1_0: libc::c_float = *x.offset(1isize);
            let mut x2_0: libc::c_float = *x.offset(2isize);
            let mut x3_0: libc::c_float = *x.offset(3isize);
            let mut x4: libc::c_float = *x.offset(4isize);
            let mut x5: libc::c_float = *x.offset(5isize);
            let mut x6: libc::c_float = *x.offset(6isize);
            let mut x7: libc::c_float = *x.offset(7isize);
            let mut xt: libc::c_float = 0.;
            xt = x0_0 - x7;
            x0_0 += x7;
            x7 = x1_0 - x6;
            x1_0 += x6;
            x6 = x2_0 - x5;
            x2_0 += x5;
            x5 = x3_0 - x4;
            x3_0 += x4;
            x4 = x0_0 - x3_0;
            x0_0 += x3_0;
            x3_0 = x1_0 - x2_0;
            x1_0 += x2_0;
            *x.offset(0isize) = x0_0 + x1_0;
            *x.offset(4isize) = (x0_0 - x1_0) * 0.7071067690849304f32;
            x5 = x5 + x6;
            x6 = (x6 + x7) * 0.7071067690849304f32;
            x7 = x7 + xt;
            x3_0 = (x3_0 + x4) * 0.7071067690849304f32;
            /* rotate by PI/8 */
            x5 -= x7 * 0.1989123672246933f32;
            x7 += x5 * 0.3826834261417389f32;
            x5 -= x7 * 0.1989123672246933f32;
            x0_0 = xt - x6;
            xt += x6;
            *x.offset(1isize) = (xt + x7) * 0.509795606136322f32;
            *x.offset(2isize) = (x4 + x3_0) * 0.5411961078643799f32;
            *x.offset(3isize) = (x0_0 - x5) * 0.601344883441925f32;
            *x.offset(5isize) = (x0_0 + x5) * 0.8999761939048767f32;
            *x.offset(6isize) = (x4 - x3_0) * 1.3065630197525025f32;
            *x.offset(7isize) = (xt - x7) * 2.562915563583374f32;
            i += 1;
            x = x.offset(8isize)
        }
        i = 0i32;
        while i < 7i32 {
            *y.offset((0i32 * 18i32) as isize) = t[0usize][i as usize];
            *y.offset((1i32 * 18i32) as isize) =
                t[2usize][i as usize] + t[3usize][i as usize] + t[3usize][(i + 1i32) as usize];
            *y.offset((2i32 * 18i32) as isize) =
                t[1usize][i as usize] + t[1usize][(i + 1i32) as usize];
            *y.offset((3i32 * 18i32) as isize) = t[2usize][(i + 1i32) as usize]
                + t[3usize][i as usize]
                + t[3usize][(i + 1i32) as usize];
            i += 1;
            y = y.offset((4i32 * 18i32) as isize)
        }
        *y.offset((0i32 * 18i32) as isize) = t[0usize][7usize];
        *y.offset((1i32 * 18i32) as isize) = t[2usize][7usize] + t[3usize][7usize];
        *y.offset((2i32 * 18i32) as isize) = t[1usize][7usize];
        *y.offset((3i32 * 18i32) as isize) = t[3usize][7usize];
        k += 1
    }
}
pub unsafe fn L12_apply_scf_384(
    mut sci: *mut L12_scale_info,
    mut scf: *const libc::c_float,
    mut dst: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    memcpy(
        dst.offset(576isize)
            .offset(((*sci).stereo_bands as libc::c_int * 18i32) as isize)
            as *mut libc::c_void,
        dst.offset(((*sci).stereo_bands as libc::c_int * 18i32) as isize) as *const libc::c_void,
        ((((*sci).total_bands as libc::c_int - (*sci).stereo_bands as libc::c_int) * 18i32)
            as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    i = 0i32;
    while i < (*sci).total_bands as libc::c_int {
        k = 0i32;
        while k < 12i32 {
            *dst.offset((k + 0i32) as isize) *= *scf.offset(0isize);
            *dst.offset((k + 576i32) as isize) *= *scf.offset(3isize);
            k += 1
        }
        i += 1;
        dst = dst.offset(18isize);
        scf = scf.offset(6isize)
    }
}
pub unsafe fn L12_dequantize_granule(
    mut grbuf: *mut libc::c_float,
    mut bs: *mut bs_t,
    mut sci: *mut L12_scale_info,
    mut group_size: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut choff: libc::c_int = 576i32;
    j = 0i32;
    while j < 4i32 {
        let mut dst: *mut libc::c_float = grbuf.offset((group_size * j) as isize);
        i = 0i32;
        while i < 2i32 * (*sci).total_bands as libc::c_int {
            let mut ba: libc::c_int = (*sci).bitalloc[i as usize] as libc::c_int;
            if ba != 0i32 {
                if ba < 17i32 {
                    let mut half: libc::c_int = (1i32 << ba - 1i32) - 1i32;
                    k = 0i32;
                    while k < group_size {
                        *dst.offset(k as isize) =
                            (get_bits(bs, ba) as libc::c_int - half) as libc::c_float;
                        k += 1
                    }
                } else {
                    /* 3, 5, 9 */
                    let mut mod_0: libc::c_uint = ((2i32 << ba - 17i32) + 1i32) as libc::c_uint;
                    /* 5, 7, 10 */
                    let mut code: libc::c_uint = get_bits(
                        bs,
                        mod_0
                            .wrapping_add(2i32 as libc::c_uint)
                            .wrapping_sub(mod_0 >> 3i32) as libc::c_int,
                    );
                    k = 0i32;
                    while k < group_size {
                        *dst.offset(k as isize) =
                            code.wrapping_rem(mod_0)
                                .wrapping_sub(mod_0.wrapping_div(2i32 as libc::c_uint))
                                as libc::c_int as libc::c_float;
                        k += 1;
                        code = code.wrapping_div(mod_0)
                    }
                }
            }
            dst = dst.offset(choff as isize);
            choff = 18i32 - choff;
            i += 1
        }
        j += 1
    }
    return group_size * 4i32;
}
pub unsafe fn get_bits(mut bs: *mut bs_t, mut n: libc::c_int) -> uint32_t {
    let mut next: uint32_t = 0;
    let mut cache: uint32_t = 0i32 as uint32_t;
    let mut s: uint32_t = ((*bs).pos & 7i32) as uint32_t;
    let mut shl: libc::c_int = (n as libc::c_uint).wrapping_add(s) as libc::c_int;
    let mut p: *const uint8_t = (*bs).buf.offset(((*bs).pos >> 3i32) as isize);
    (*bs).pos += n;
    if (*bs).pos > (*bs).limit {
        return 0i32 as uint32_t;
    } else {
        let fresh16 = p;
        p = p.offset(1);
        next = (*fresh16 as libc::c_int & 255i32 >> s) as uint32_t;
        loop {
            shl -= 8i32;
            if !(shl > 0i32) {
                break;
            }
            cache |= next << shl;
            let fresh17 = p;
            p = p.offset(1);
            next = *fresh17 as uint32_t
        }
        return cache | next >> -shl;
    };
}
pub unsafe fn L12_read_scale_info(
    mut hdr: *const uint8_t,
    mut bs: *mut bs_t,
    mut sci: *mut L12_scale_info,
) {
    static mut g_bitalloc_code_tab: [uint8_t; 92] = [
        0i32 as uint8_t,
        17i32 as uint8_t,
        3i32 as uint8_t,
        4i32 as uint8_t,
        5i32 as uint8_t,
        6i32 as uint8_t,
        7i32 as uint8_t,
        8i32 as uint8_t,
        9i32 as uint8_t,
        10i32 as uint8_t,
        11i32 as uint8_t,
        12i32 as uint8_t,
        13i32 as uint8_t,
        14i32 as uint8_t,
        15i32 as uint8_t,
        16i32 as uint8_t,
        0i32 as uint8_t,
        17i32 as uint8_t,
        18i32 as uint8_t,
        3i32 as uint8_t,
        19i32 as uint8_t,
        4i32 as uint8_t,
        5i32 as uint8_t,
        6i32 as uint8_t,
        7i32 as uint8_t,
        8i32 as uint8_t,
        9i32 as uint8_t,
        10i32 as uint8_t,
        11i32 as uint8_t,
        12i32 as uint8_t,
        13i32 as uint8_t,
        16i32 as uint8_t,
        0i32 as uint8_t,
        17i32 as uint8_t,
        18i32 as uint8_t,
        3i32 as uint8_t,
        19i32 as uint8_t,
        4i32 as uint8_t,
        5i32 as uint8_t,
        16i32 as uint8_t,
        0i32 as uint8_t,
        17i32 as uint8_t,
        18i32 as uint8_t,
        16i32 as uint8_t,
        0i32 as uint8_t,
        17i32 as uint8_t,
        18i32 as uint8_t,
        19i32 as uint8_t,
        4i32 as uint8_t,
        5i32 as uint8_t,
        6i32 as uint8_t,
        7i32 as uint8_t,
        8i32 as uint8_t,
        9i32 as uint8_t,
        10i32 as uint8_t,
        11i32 as uint8_t,
        12i32 as uint8_t,
        13i32 as uint8_t,
        14i32 as uint8_t,
        15i32 as uint8_t,
        0i32 as uint8_t,
        17i32 as uint8_t,
        18i32 as uint8_t,
        3i32 as uint8_t,
        19i32 as uint8_t,
        4i32 as uint8_t,
        5i32 as uint8_t,
        6i32 as uint8_t,
        7i32 as uint8_t,
        8i32 as uint8_t,
        9i32 as uint8_t,
        10i32 as uint8_t,
        11i32 as uint8_t,
        12i32 as uint8_t,
        13i32 as uint8_t,
        14i32 as uint8_t,
        0i32 as uint8_t,
        2i32 as uint8_t,
        3i32 as uint8_t,
        4i32 as uint8_t,
        5i32 as uint8_t,
        6i32 as uint8_t,
        7i32 as uint8_t,
        8i32 as uint8_t,
        9i32 as uint8_t,
        10i32 as uint8_t,
        11i32 as uint8_t,
        12i32 as uint8_t,
        13i32 as uint8_t,
        14i32 as uint8_t,
        15i32 as uint8_t,
        16i32 as uint8_t,
    ];
    let mut subband_alloc: *const L12_subband_alloc_t = L12_subband_alloc_table(hdr, sci);
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0i32;
    let mut ba_bits: libc::c_int = 0i32;
    let mut ba_code_tab: *const uint8_t = g_bitalloc_code_tab.as_ptr();
    i = 0i32;
    while i < (*sci).total_bands as libc::c_int {
        let mut ba: uint8_t = 0;
        if i == k {
            k += (*subband_alloc).band_count as libc::c_int;
            ba_bits = (*subband_alloc).code_tab_width as libc::c_int;
            ba_code_tab = g_bitalloc_code_tab
                .as_ptr()
                .offset((*subband_alloc).tab_offset as libc::c_int as isize);
            subband_alloc = subband_alloc.offset(1isize)
        }
        ba = *ba_code_tab.offset(get_bits(bs, ba_bits) as isize);
        (*sci).bitalloc[(2i32 * i) as usize] = ba;
        if i < (*sci).stereo_bands as libc::c_int {
            ba = *ba_code_tab.offset(get_bits(bs, ba_bits) as isize)
        }
        (*sci).bitalloc[(2i32 * i + 1i32) as usize] = (if 0 != (*sci).stereo_bands as libc::c_int {
            ba as libc::c_int
        } else {
            0i32
        }) as uint8_t;
        i += 1
    }
    i = 0i32;
    while i < 2i32 * (*sci).total_bands as libc::c_int {
        (*sci).scfcod[i as usize] = (if 0 != (*sci).bitalloc[i as usize] as libc::c_int {
            if *hdr.offset(1isize) as libc::c_int & 6i32 == 6i32 {
                2i32 as libc::c_uint
            } else {
                get_bits(bs, 2i32)
            }
        } else {
            6i32 as libc::c_uint
        }) as uint8_t;
        i += 1
    }
    L12_read_scalefactors(
        bs,
        (*sci).bitalloc.as_mut_ptr(),
        (*sci).scfcod.as_mut_ptr(),
        (*sci).total_bands as libc::c_int * 2i32,
        (*sci).scf.as_mut_ptr(),
    );
    i = (*sci).stereo_bands as libc::c_int;
    while i < (*sci).total_bands as libc::c_int {
        (*sci).bitalloc[(2i32 * i + 1i32) as usize] = 0i32 as uint8_t;
        i += 1
    }
}
pub unsafe fn L12_read_scalefactors(
    mut bs: *mut bs_t,
    mut pba: *mut uint8_t,
    mut scfcod: *mut uint8_t,
    mut bands: libc::c_int,
    mut scf: *mut libc::c_float,
) {
    let mut b: libc::c_int = 0;
    static mut g_deq_L12: [libc::c_float; 54] = [
        9.5367431640625e-7f32 / 3i32 as libc::c_float,
        7.569317972411227e-7f32 / 3i32 as libc::c_float,
        6.007771844451781e-7f32 / 3i32 as libc::c_float,
        9.5367431640625e-7f32 / 7i32 as libc::c_float,
        7.569317972411227e-7f32 / 7i32 as libc::c_float,
        6.007771844451781e-7f32 / 7i32 as libc::c_float,
        9.5367431640625e-7f32 / 15i32 as libc::c_float,
        7.569317972411227e-7f32 / 15i32 as libc::c_float,
        6.007771844451781e-7f32 / 15i32 as libc::c_float,
        9.5367431640625e-7f32 / 31i32 as libc::c_float,
        7.569317972411227e-7f32 / 31i32 as libc::c_float,
        6.007771844451781e-7f32 / 31i32 as libc::c_float,
        9.5367431640625e-7f32 / 63i32 as libc::c_float,
        7.569317972411227e-7f32 / 63i32 as libc::c_float,
        6.007771844451781e-7f32 / 63i32 as libc::c_float,
        9.5367431640625e-7f32 / 127i32 as libc::c_float,
        7.569317972411227e-7f32 / 127i32 as libc::c_float,
        6.007771844451781e-7f32 / 127i32 as libc::c_float,
        9.5367431640625e-7f32 / 255i32 as libc::c_float,
        7.569317972411227e-7f32 / 255i32 as libc::c_float,
        6.007771844451781e-7f32 / 255i32 as libc::c_float,
        9.5367431640625e-7f32 / 511i32 as libc::c_float,
        7.569317972411227e-7f32 / 511i32 as libc::c_float,
        6.007771844451781e-7f32 / 511i32 as libc::c_float,
        9.5367431640625e-7f32 / 1023i32 as libc::c_float,
        7.569317972411227e-7f32 / 1023i32 as libc::c_float,
        6.007771844451781e-7f32 / 1023i32 as libc::c_float,
        9.5367431640625e-7f32 / 2047i32 as libc::c_float,
        7.569317972411227e-7f32 / 2047i32 as libc::c_float,
        6.007771844451781e-7f32 / 2047i32 as libc::c_float,
        9.5367431640625e-7f32 / 4095i32 as libc::c_float,
        7.569317972411227e-7f32 / 4095i32 as libc::c_float,
        6.007771844451781e-7f32 / 4095i32 as libc::c_float,
        9.5367431640625e-7f32 / 8191i32 as libc::c_float,
        7.569317972411227e-7f32 / 8191i32 as libc::c_float,
        6.007771844451781e-7f32 / 8191i32 as libc::c_float,
        9.5367431640625e-7f32 / 16383i32 as libc::c_float,
        7.569317972411227e-7f32 / 16383i32 as libc::c_float,
        6.007771844451781e-7f32 / 16383i32 as libc::c_float,
        9.5367431640625e-7f32 / 32767i32 as libc::c_float,
        7.569317972411227e-7f32 / 32767i32 as libc::c_float,
        6.007771844451781e-7f32 / 32767i32 as libc::c_float,
        9.5367431640625e-7f32 / 65535i32 as libc::c_float,
        7.569317972411227e-7f32 / 65535i32 as libc::c_float,
        6.007771844451781e-7f32 / 65535i32 as libc::c_float,
        9.5367431640625e-7f32 / 3i32 as libc::c_float,
        7.569317972411227e-7f32 / 3i32 as libc::c_float,
        6.007771844451781e-7f32 / 3i32 as libc::c_float,
        9.5367431640625e-7f32 / 5i32 as libc::c_float,
        7.569317972411227e-7f32 / 5i32 as libc::c_float,
        6.007771844451781e-7f32 / 5i32 as libc::c_float,
        9.5367431640625e-7f32 / 9i32 as libc::c_float,
        7.569317972411227e-7f32 / 9i32 as libc::c_float,
        6.007771844451781e-7f32 / 9i32 as libc::c_float,
    ];
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    i = 0i32;
    while i < bands {
        let mut s: libc::c_float = 0i32 as libc::c_float;
        let fresh18 = pba;
        pba = pba.offset(1);
        let mut ba: libc::c_int = *fresh18 as libc::c_int;
        let mut mask: libc::c_int = if 0 != ba {
            4i32 + (19i32 >> *scfcod.offset(i as isize) as libc::c_int & 3i32)
        } else {
            0i32
        };
        m = 4i32;
        while 0 != m {
            if 0 != mask & m {
                b = get_bits(bs, 6i32) as libc::c_int;
                s = g_deq_L12[(ba * 3i32 - 6i32 + b % 3i32) as usize]
                    * (1i32 << 21i32 >> b / 3i32) as libc::c_float
            }
            let fresh19 = scf;
            scf = scf.offset(1);
            *fresh19 = s;
            m >>= 1i32
        }
        i += 1
    }
}
pub unsafe fn L12_subband_alloc_table(
    mut hdr: *const uint8_t,
    mut sci: *mut L12_scale_info,
) -> *const L12_subband_alloc_t {
    static mut g_alloc_L1: [L12_subband_alloc_t; 1] = [L12_subband_alloc_t {
        tab_offset: 76i32 as uint8_t,
        code_tab_width: 4i32 as uint8_t,
        band_count: 32i32 as uint8_t,
    }];
    static mut g_alloc_L2M2: [L12_subband_alloc_t; 3] = [
        L12_subband_alloc_t {
            tab_offset: 60i32 as uint8_t,
            code_tab_width: 4i32 as uint8_t,
            band_count: 4i32 as uint8_t,
        },
        L12_subband_alloc_t {
            tab_offset: 44i32 as uint8_t,
            code_tab_width: 3i32 as uint8_t,
            band_count: 7i32 as uint8_t,
        },
        L12_subband_alloc_t {
            tab_offset: 44i32 as uint8_t,
            code_tab_width: 2i32 as uint8_t,
            band_count: 19i32 as uint8_t,
        },
    ];
    static mut g_alloc_L2M1_lowrate: [L12_subband_alloc_t; 2] = [
        L12_subband_alloc_t {
            tab_offset: 44i32 as uint8_t,
            code_tab_width: 4i32 as uint8_t,
            band_count: 2i32 as uint8_t,
        },
        L12_subband_alloc_t {
            tab_offset: 44i32 as uint8_t,
            code_tab_width: 3i32 as uint8_t,
            band_count: 10i32 as uint8_t,
        },
    ];
    let mut alloc: *const L12_subband_alloc_t = 0 as *const L12_subband_alloc_t;
    let mut mode: libc::c_int = *hdr.offset(3isize) as libc::c_int >> 6i32 & 3i32;
    let mut nbands: libc::c_int = 0;
    let mut stereo_bands: libc::c_int = if mode == 3i32 {
        0i32
    } else if mode == 1i32 {
        ((*hdr.offset(3isize) as libc::c_int >> 4i32 & 3i32) << 2i32) + 4i32
    } else {
        32i32
    };
    if *hdr.offset(1isize) as libc::c_int & 6i32 == 6i32 {
        alloc = g_alloc_L1.as_ptr();
        nbands = 32i32
    } else if 0 == *hdr.offset(1isize) as libc::c_int & 0x8i32 {
        alloc = g_alloc_L2M2.as_ptr();
        nbands = 30i32
    } else {
        static mut g_alloc_L2M1: [L12_subband_alloc_t; 4] = [
            L12_subband_alloc_t {
                tab_offset: 0i32 as uint8_t,
                code_tab_width: 4i32 as uint8_t,
                band_count: 3i32 as uint8_t,
            },
            L12_subband_alloc_t {
                tab_offset: 16i32 as uint8_t,
                code_tab_width: 4i32 as uint8_t,
                band_count: 8i32 as uint8_t,
            },
            L12_subband_alloc_t {
                tab_offset: 32i32 as uint8_t,
                code_tab_width: 3i32 as uint8_t,
                band_count: 12i32 as uint8_t,
            },
            L12_subband_alloc_t {
                tab_offset: 40i32 as uint8_t,
                code_tab_width: 2i32 as uint8_t,
                band_count: 7i32 as uint8_t,
            },
        ];
        let mut sample_rate_idx: libc::c_int = *hdr.offset(2isize) as libc::c_int >> 2i32 & 3i32;
        let mut kbps: libc::c_uint = hdr_bitrate_kbps(hdr) >> (mode != 3i32) as libc::c_int;
        /* free-format */
        if 0 == kbps {
            kbps = 192i32 as libc::c_uint
        }
        alloc = g_alloc_L2M1.as_ptr();
        nbands = 27i32;
        if kbps < 56i32 as libc::c_uint {
            alloc = g_alloc_L2M1_lowrate.as_ptr();
            nbands = if sample_rate_idx == 2i32 { 12i32 } else { 8i32 }
        } else if kbps >= 96i32 as libc::c_uint && sample_rate_idx != 1i32 {
            nbands = 30i32
        }
    }
    (*sci).total_bands = nbands as uint8_t;
    (*sci).stereo_bands = (if stereo_bands > nbands {
        nbands
    } else {
        stereo_bands
    }) as uint8_t;
    return alloc;
}
pub unsafe fn hdr_bitrate_kbps(mut h: *const uint8_t) -> libc::c_uint {
    static mut halfrate: [[[uint8_t; 15]; 3]; 2] = [
        [
            [
                0i32 as uint8_t,
                4i32 as uint8_t,
                8i32 as uint8_t,
                12i32 as uint8_t,
                16i32 as uint8_t,
                20i32 as uint8_t,
                24i32 as uint8_t,
                28i32 as uint8_t,
                32i32 as uint8_t,
                40i32 as uint8_t,
                48i32 as uint8_t,
                56i32 as uint8_t,
                64i32 as uint8_t,
                72i32 as uint8_t,
                80i32 as uint8_t,
            ],
            [
                0i32 as uint8_t,
                4i32 as uint8_t,
                8i32 as uint8_t,
                12i32 as uint8_t,
                16i32 as uint8_t,
                20i32 as uint8_t,
                24i32 as uint8_t,
                28i32 as uint8_t,
                32i32 as uint8_t,
                40i32 as uint8_t,
                48i32 as uint8_t,
                56i32 as uint8_t,
                64i32 as uint8_t,
                72i32 as uint8_t,
                80i32 as uint8_t,
            ],
            [
                0i32 as uint8_t,
                16i32 as uint8_t,
                24i32 as uint8_t,
                28i32 as uint8_t,
                32i32 as uint8_t,
                40i32 as uint8_t,
                48i32 as uint8_t,
                56i32 as uint8_t,
                64i32 as uint8_t,
                72i32 as uint8_t,
                80i32 as uint8_t,
                88i32 as uint8_t,
                96i32 as uint8_t,
                112i32 as uint8_t,
                128i32 as uint8_t,
            ],
        ],
        [
            [
                0i32 as uint8_t,
                16i32 as uint8_t,
                20i32 as uint8_t,
                24i32 as uint8_t,
                28i32 as uint8_t,
                32i32 as uint8_t,
                40i32 as uint8_t,
                48i32 as uint8_t,
                56i32 as uint8_t,
                64i32 as uint8_t,
                80i32 as uint8_t,
                96i32 as uint8_t,
                112i32 as uint8_t,
                128i32 as uint8_t,
                160i32 as uint8_t,
            ],
            [
                0i32 as uint8_t,
                16i32 as uint8_t,
                24i32 as uint8_t,
                28i32 as uint8_t,
                32i32 as uint8_t,
                40i32 as uint8_t,
                48i32 as uint8_t,
                56i32 as uint8_t,
                64i32 as uint8_t,
                80i32 as uint8_t,
                96i32 as uint8_t,
                112i32 as uint8_t,
                128i32 as uint8_t,
                160i32 as uint8_t,
                192i32 as uint8_t,
            ],
            [
                0i32 as uint8_t,
                16i32 as uint8_t,
                32i32 as uint8_t,
                48i32 as uint8_t,
                64i32 as uint8_t,
                80i32 as uint8_t,
                96i32 as uint8_t,
                112i32 as uint8_t,
                128i32 as uint8_t,
                144i32 as uint8_t,
                160i32 as uint8_t,
                176i32 as uint8_t,
                192i32 as uint8_t,
                208i32 as uint8_t,
                224i32 as uint8_t,
            ],
        ],
    ];
    return (2i32 * halfrate
        [(0 != *h.offset(1isize) as libc::c_int & 0x8i32) as libc::c_int as usize]
        [((*h.offset(1isize) as libc::c_int >> 1i32 & 3i32) - 1i32) as usize]
        [(*h.offset(2isize) as libc::c_int >> 4i32) as usize] as libc::c_int)
        as libc::c_uint;
}
pub unsafe fn L3_save_reservoir(mut h: *mut mp3dec_t, mut s: *mut mp3dec_scratch_t) {
    let mut pos: libc::c_int =
        (((*s).bs.pos + 7i32) as libc::c_uint).wrapping_div(8u32) as libc::c_int;
    let mut remains: libc::c_int = ((*s).bs.limit as libc::c_uint)
        .wrapping_div(8u32)
        .wrapping_sub(pos as libc::c_uint) as libc::c_int;
    if remains > 511i32 {
        pos += remains - 511i32;
        remains = 511i32
    }
    if remains > 0i32 {
        memmove(
            (*h).reserv_buf.as_mut_ptr() as *mut libc::c_void,
            (*s).maindata.as_mut_ptr().offset(pos as isize) as *const libc::c_void,
            remains as libc::c_ulong,
        );
    }
    (*h).reserv = remains;
}
pub unsafe fn L3_decode(
    mut h: *mut mp3dec_t,
    mut s: *mut mp3dec_scratch_t,
    mut gr_info: *mut L3_gr_info_t,
    mut nch: libc::c_int,
) {
    let mut ch: libc::c_int = 0;
    ch = 0i32;
    while ch < nch {
        let mut layer3gr_limit: libc::c_int =
            (*s).bs.pos + (*gr_info.offset(ch as isize)).part_23_length as libc::c_int;
        L3_decode_scalefactors(
            (*h).header.as_mut_ptr(),
            (*s).ist_pos[ch as usize].as_mut_ptr(),
            &mut (*s).bs,
            gr_info.offset(ch as isize),
            (*s).scf.as_mut_ptr(),
            ch,
        );
        L3_huffman(
            (*s).grbuf[ch as usize].as_mut_ptr(),
            &mut (*s).bs,
            gr_info.offset(ch as isize),
            (*s).scf.as_mut_ptr(),
            layer3gr_limit,
        );
        ch += 1
    }
    if 0 != (*h).header[3usize] as libc::c_int & 0x10i32 {
        L3_intensity_stereo(
            (*s).grbuf[0usize].as_mut_ptr(),
            (*s).ist_pos[1usize].as_mut_ptr(),
            gr_info,
            (*h).header.as_mut_ptr(),
        );
    } else if (*h).header[3usize] as libc::c_int & 0xe0i32 == 0x60i32 {
        L3_midside_stereo((*s).grbuf[0usize].as_mut_ptr(), 576i32);
    }
    ch = 0i32;
    while ch < nch {
        let mut aa_bands: libc::c_int = 31i32;
        let mut n_long_bands: libc::c_int = if 0 != (*gr_info).mixed_block_flag as libc::c_int {
            2i32
        } else {
            0i32
        } << (((*h).header[2usize] as libc::c_int >> 2i32
            & 3i32)
            + (((*h).header[1usize] as libc::c_int >> 3i32 & 1i32)
                + ((*h).header[1usize] as libc::c_int >> 4i32 & 1i32))
                * 3i32
            == 2i32) as libc::c_int;
        if 0 != (*gr_info).n_short_sfb {
            aa_bands = n_long_bands - 1i32;
            L3_reorder(
                (*s).grbuf[ch as usize]
                    .as_mut_ptr()
                    .offset((n_long_bands * 18i32) as isize),
                (*s).syn[0usize].as_mut_ptr(),
                (*gr_info)
                    .sfbtab
                    .offset((*gr_info).n_long_sfb as libc::c_int as isize),
            );
        }
        L3_antialias((*s).grbuf[ch as usize].as_mut_ptr(), aa_bands);
        L3_imdct_gr(
            (*s).grbuf[ch as usize].as_mut_ptr(),
            (*h).mdct_overlap[ch as usize].as_mut_ptr(),
            (*gr_info).block_type as libc::c_uint,
            n_long_bands as libc::c_uint,
        );
        L3_change_sign((*s).grbuf[ch as usize].as_mut_ptr());
        ch += 1;
        gr_info = gr_info.offset(1isize)
    }
}
pub unsafe fn L3_change_sign(mut grbuf: *mut libc::c_float) {
    let mut b: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    b = 0i32;
    grbuf = grbuf.offset(18isize);
    while b < 32i32 {
        i = 1i32;
        while i < 18i32 {
            *grbuf.offset(i as isize) = -*grbuf.offset(i as isize);
            i += 2i32
        }
        b += 2i32;
        grbuf = grbuf.offset(36isize)
    }
}
pub unsafe fn L3_imdct_gr(
    mut grbuf: *mut libc::c_float,
    mut overlap: *mut libc::c_float,
    mut block_type: libc::c_uint,
    mut n_long_bands: libc::c_uint,
) {
    static mut g_mdct_window: [[libc::c_float; 18]; 2] = [
        [
            0.9990482330322266f32,
            0.9914448857307434f32,
            0.9762960076332092f32,
            0.9537169337272644f32,
            0.9238795042037964f32,
            0.8870108127593994f32,
            0.843391478061676f32,
            0.7933533191680908f32,
            0.7372773289680481f32,
            0.04361937940120697f32,
            0.13052618503570558f32,
            0.2164396047592163f32,
            0.30070579051971438f32,
            0.3826834261417389f32,
            0.4617486000061035f32,
            0.537299633026123f32,
            0.6087614297866821f32,
            0.6755902171134949f32,
        ],
        [
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            0.9914448857307434f32,
            0.9238795042037964f32,
            0.7933533191680908f32,
            0i32 as libc::c_float,
            0i32 as libc::c_float,
            0i32 as libc::c_float,
            0i32 as libc::c_float,
            0i32 as libc::c_float,
            0i32 as libc::c_float,
            0.13052618503570558f32,
            0.3826834261417389f32,
            0.6087614297866821f32,
        ],
    ];
    if 0 != n_long_bands {
        L3_imdct36(
            grbuf,
            overlap,
            g_mdct_window[0usize].as_ptr(),
            n_long_bands as libc::c_int,
        );
        grbuf = grbuf.offset((18i32 as libc::c_uint).wrapping_mul(n_long_bands) as isize);
        overlap = overlap.offset((9i32 as libc::c_uint).wrapping_mul(n_long_bands) as isize)
    }
    if block_type == 2i32 as libc::c_uint {
        L3_imdct_short(
            grbuf,
            overlap,
            (32i32 as libc::c_uint).wrapping_sub(n_long_bands) as libc::c_int,
        );
    } else {
        L3_imdct36(
            grbuf,
            overlap,
            g_mdct_window[(block_type == 3i32 as libc::c_uint) as libc::c_int as usize].as_ptr(),
            (32i32 as libc::c_uint).wrapping_sub(n_long_bands) as libc::c_int,
        );
    };
}
pub unsafe fn L3_imdct36(
    mut grbuf: *mut libc::c_float,
    mut overlap: *mut libc::c_float,
    mut window: *const libc::c_float,
    mut nbands: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    static mut g_twid9: [libc::c_float; 18] = [
        0.7372773289680481f32,
        0.7933533191680908f32,
        0.843391478061676f32,
        0.8870108127593994f32,
        0.9238795042037964f32,
        0.9537169337272644f32,
        0.9762960076332092f32,
        0.9914448857307434f32,
        0.9990482330322266f32,
        0.6755902171134949f32,
        0.6087614297866821f32,
        0.537299633026123f32,
        0.4617486000061035f32,
        0.3826834261417389f32,
        0.30070579051971438f32,
        0.2164396047592163f32,
        0.13052618503570558f32,
        0.04361937940120697f32,
    ];
    j = 0i32;
    while j < nbands {
        let mut co: [libc::c_float; 9] = [0.; 9];
        let mut si: [libc::c_float; 9] = [0.; 9];
        co[0usize] = -*grbuf.offset(0isize);
        si[0usize] = *grbuf.offset(17isize);
        i = 0i32;
        while i < 4i32 {
            si[(8i32 - 2i32 * i) as usize] = *grbuf.offset((4i32 * i + 1i32) as isize)
                - *grbuf.offset((4i32 * i + 2i32) as isize);
            co[(1i32 + 2i32 * i) as usize] = *grbuf.offset((4i32 * i + 1i32) as isize)
                + *grbuf.offset((4i32 * i + 2i32) as isize);
            si[(7i32 - 2i32 * i) as usize] = *grbuf.offset((4i32 * i + 4i32) as isize)
                - *grbuf.offset((4i32 * i + 3i32) as isize);
            co[(2i32 + 2i32 * i) as usize] = -(*grbuf.offset((4i32 * i + 3i32) as isize)
                + *grbuf.offset((4i32 * i + 4i32) as isize));
            i += 1
        }
        L3_dct3_9(co.as_mut_ptr());
        L3_dct3_9(si.as_mut_ptr());
        si[1usize] = -si[1usize];
        si[3usize] = -si[3usize];
        si[5usize] = -si[5usize];
        si[7usize] = -si[7usize];
        i = 0i32;
        /* HAVE_SIMD */
        while i < 9i32 {
            let mut ovl: libc::c_float = *overlap.offset(i as isize);
            let mut sum: libc::c_float = co[i as usize] * g_twid9[(9i32 + i) as usize]
                + si[i as usize] * g_twid9[(0i32 + i) as usize];
            *overlap.offset(i as isize) = co[i as usize] * g_twid9[(0i32 + i) as usize]
                - si[i as usize] * g_twid9[(9i32 + i) as usize];
            *grbuf.offset(i as isize) = ovl * *window.offset((0i32 + i) as isize)
                - sum * *window.offset((9i32 + i) as isize);
            *grbuf.offset((17i32 - i) as isize) = ovl * *window.offset((9i32 + i) as isize)
                + sum * *window.offset((0i32 + i) as isize);
            i += 1
        }
        j += 1;
        grbuf = grbuf.offset(18isize);
        overlap = overlap.offset(9isize)
    }
}
/* MINIMP3_ONLY_SIMD */
pub unsafe fn L3_dct3_9(mut y: *mut libc::c_float) {
    let mut s0: libc::c_float = 0.;
    let mut s1: libc::c_float = 0.;
    let mut s2: libc::c_float = 0.;
    let mut s3: libc::c_float = 0.;
    let mut s4: libc::c_float = 0.;
    let mut s5: libc::c_float = 0.;
    let mut s6: libc::c_float = 0.;
    let mut s7: libc::c_float = 0.;
    let mut s8: libc::c_float = 0.;
    let mut t0: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    let mut t4: libc::c_float = 0.;
    s0 = *y.offset(0isize);
    s2 = *y.offset(2isize);
    s4 = *y.offset(4isize);
    s6 = *y.offset(6isize);
    s8 = *y.offset(8isize);
    t0 = s0 + s6 * 0.5f32;
    s0 -= s6;
    t4 = (s4 + s2) * 0.9396926164627075f32;
    t2 = (s8 + s2) * 0.7660444378852844f32;
    s6 = (s4 - s8) * 0.1736481785774231f32;
    s4 += s8 - s2;
    s2 = s0 - s4 * 0.5f32;
    *y.offset(4isize) = s4 + s0;
    s8 = t0 - t2 + s6;
    s0 = t0 - t4 + t2;
    s4 = t0 + t4 - s6;
    s1 = *y.offset(1isize);
    s3 = *y.offset(3isize);
    s5 = *y.offset(5isize);
    s7 = *y.offset(7isize);
    s3 *= 0.8660253882408142f32;
    t0 = (s5 + s1) * 0.9848077297210693f32;
    t4 = (s5 - s7) * 0.3420201539993286f32;
    t2 = (s1 + s7) * 0.6427876353263855f32;
    s1 = (s1 - s5 - s7) * 0.8660253882408142f32;
    s5 = t0 - s3 - t2;
    s7 = t4 - s3 - t0;
    s3 = t4 + s3 - t2;
    *y.offset(0isize) = s4 - s7;
    *y.offset(1isize) = s2 + s1;
    *y.offset(2isize) = s0 - s3;
    *y.offset(3isize) = s8 + s5;
    *y.offset(5isize) = s8 - s5;
    *y.offset(6isize) = s0 + s3;
    *y.offset(7isize) = s2 - s1;
    *y.offset(8isize) = s4 + s7;
}
pub unsafe fn L3_imdct_short(
    mut grbuf: *mut libc::c_float,
    mut overlap: *mut libc::c_float,
    mut nbands: libc::c_int,
) {
    while nbands > 0i32 {
        let mut tmp: [libc::c_float; 18] = [0.; 18];
        memcpy(
            tmp.as_mut_ptr() as *mut libc::c_void,
            grbuf as *const libc::c_void,
            ::core::mem::size_of::<[libc::c_float; 18]>() as libc::c_ulong,
        );
        memcpy(
            grbuf as *mut libc::c_void,
            overlap as *const libc::c_void,
            (6i32 as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
        L3_imdct12(
            tmp.as_mut_ptr(),
            grbuf.offset(6isize),
            overlap.offset(6isize),
        );
        L3_imdct12(
            tmp.as_mut_ptr().offset(1isize),
            grbuf.offset(12isize),
            overlap.offset(6isize),
        );
        L3_imdct12(
            tmp.as_mut_ptr().offset(2isize),
            overlap,
            overlap.offset(6isize),
        );
        nbands -= 1;
        overlap = overlap.offset(9isize);
        grbuf = grbuf.offset(18isize)
    }
}
pub unsafe fn L3_imdct12(
    mut x: *mut libc::c_float,
    mut dst: *mut libc::c_float,
    mut overlap: *mut libc::c_float,
) {
    static mut g_twid3: [libc::c_float; 6] = [
        0.7933533191680908f32,
        0.9238795042037964f32,
        0.9914448857307434f32,
        0.6087614297866821f32,
        0.3826834261417389f32,
        0.13052618503570558f32,
    ];
    let mut co: [libc::c_float; 3] = [0.; 3];
    let mut si: [libc::c_float; 3] = [0.; 3];
    let mut i: libc::c_int = 0;
    L3_idct3(
        -*x.offset(0isize),
        *x.offset(6isize) + *x.offset(3isize),
        *x.offset(12isize) + *x.offset(9isize),
        co.as_mut_ptr(),
    );
    L3_idct3(
        *x.offset(15isize),
        *x.offset(12isize) - *x.offset(9isize),
        *x.offset(6isize) - *x.offset(3isize),
        si.as_mut_ptr(),
    );
    si[1usize] = -si[1usize];
    i = 0i32;
    while i < 3i32 {
        let mut ovl: libc::c_float = *overlap.offset(i as isize);
        let mut sum: libc::c_float = co[i as usize] * g_twid3[(3i32 + i) as usize]
            + si[i as usize] * g_twid3[(0i32 + i) as usize];
        *overlap.offset(i as isize) = co[i as usize] * g_twid3[(0i32 + i) as usize]
            - si[i as usize] * g_twid3[(3i32 + i) as usize];
        *dst.offset(i as isize) =
            ovl * g_twid3[(2i32 - i) as usize] - sum * g_twid3[(5i32 - i) as usize];
        *dst.offset((5i32 - i) as isize) =
            ovl * g_twid3[(5i32 - i) as usize] + sum * g_twid3[(2i32 - i) as usize];
        i += 1
    }
}
pub unsafe fn L3_idct3(
    mut x0: libc::c_float,
    mut x1: libc::c_float,
    mut x2: libc::c_float,
    mut dst: *mut libc::c_float,
) {
    let mut m1: libc::c_float = x1 * 0.8660253882408142f32;
    let mut a1: libc::c_float = x0 - x2 * 0.5f32;
    *dst.offset(1isize) = x0 + x2;
    *dst.offset(0isize) = a1 + m1;
    *dst.offset(2isize) = a1 - m1;
}
pub unsafe fn L3_antialias(mut grbuf: *mut libc::c_float, mut nbands: libc::c_int) {
    static mut g_aa: [[libc::c_float; 8]; 2] = [
        [
            0.8574929237365723f32,
            0.881742000579834f32,
            0.9496286511421204f32,
            0.983314573764801f32,
            0.9955177903175354f32,
            0.9991605877876282f32,
            0.9998992085456848f32,
            0.9999931454658508f32,
        ],
        [
            0.5144957304000855f32,
            0.471731960773468f32,
            0.3133774399757385f32,
            0.18191319704055787f32,
            0.09457419067621231f32,
            0.04096557945013046f32,
            0.014198560267686844f32,
            0.0036999699659645559f32,
        ],
    ];
    while nbands > 0i32 {
        let mut i: libc::c_int = 0i32;
        /* HAVE_SIMD */
        while i < 8i32 {
            let mut u: libc::c_float = *grbuf.offset((18i32 + i) as isize);
            let mut d: libc::c_float = *grbuf.offset((17i32 - i) as isize);
            *grbuf.offset((18i32 + i) as isize) =
                u * g_aa[0usize][i as usize] - d * g_aa[1usize][i as usize];
            *grbuf.offset((17i32 - i) as isize) =
                u * g_aa[1usize][i as usize] + d * g_aa[0usize][i as usize];
            i += 1
        }
        nbands -= 1;
        grbuf = grbuf.offset(18isize)
    }
}
pub unsafe fn L3_reorder(
    mut grbuf: *mut libc::c_float,
    mut scratch: *mut libc::c_float,
    mut sfb: *const uint8_t,
) {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut src: *mut libc::c_float = grbuf;
    let mut dst: *mut libc::c_float = scratch;
    loop {
        len = *sfb as libc::c_int;
        if !(0i32 != len) {
            break;
        }
        i = 0i32;
        while i < len {
            let fresh20 = dst;
            dst = dst.offset(1);
            *fresh20 = *src.offset((0i32 * len) as isize);
            let fresh21 = dst;
            dst = dst.offset(1);
            *fresh21 = *src.offset((1i32 * len) as isize);
            let fresh22 = dst;
            dst = dst.offset(1);
            *fresh22 = *src.offset((2i32 * len) as isize);
            i += 1;
            src = src.offset(1isize)
        }
        sfb = sfb.offset(3isize);
        src = src.offset((2i32 * len) as isize)
    }
    memcpy(
        grbuf as *mut libc::c_void,
        scratch as *const libc::c_void,
        (wrapping_offset_from(dst, scratch) as libc::c_long as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
}
pub unsafe fn L3_midside_stereo(mut left: *mut libc::c_float, mut n: libc::c_int) {
    let mut i: libc::c_int = 0i32;
    let mut right: *mut libc::c_float = left.offset(576isize);
    /* HAVE_SIMD */
    while i < n {
        let mut a: libc::c_float = *left.offset(i as isize);
        let mut b: libc::c_float = *right.offset(i as isize);
        *left.offset(i as isize) = a + b;
        *right.offset(i as isize) = a - b;
        i += 1
    }
}
pub unsafe fn L3_intensity_stereo(
    mut left: *mut libc::c_float,
    mut ist_pos: *mut uint8_t,
    mut gr: *const L3_gr_info_t,
    mut hdr: *const uint8_t,
) {
    let mut max_band: [libc::c_int; 3] = [0; 3];
    let mut n_sfb: libc::c_int = (*gr).n_long_sfb as libc::c_int + (*gr).n_short_sfb as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut max_blocks: libc::c_int = if 0 != (*gr).n_short_sfb as libc::c_int {
        3i32
    } else {
        1i32
    };
    L3_stereo_top_band(
        left.offset(576isize),
        (*gr).sfbtab,
        n_sfb,
        max_band.as_mut_ptr(),
    );
    if 0 != (*gr).n_long_sfb {
        max_band[2usize] = if if max_band[0usize] < max_band[1usize] {
            max_band[1usize]
        } else {
            max_band[0usize]
        } < max_band[2usize]
        {
            max_band[2usize]
        } else if max_band[0usize] < max_band[1usize] {
            max_band[1usize]
        } else {
            max_band[0usize]
        };
        max_band[1usize] = max_band[2usize];
        max_band[0usize] = max_band[1usize]
    }
    i = 0i32;
    while i < max_blocks {
        let mut default_pos: libc::c_int = if 0 != *hdr.offset(1isize) as libc::c_int & 0x8i32 {
            3i32
        } else {
            0i32
        };
        let mut itop: libc::c_int = n_sfb - max_blocks + i;
        let mut prev: libc::c_int = itop - max_blocks;
        *ist_pos.offset(itop as isize) = (if max_band[i as usize] >= prev {
            default_pos
        } else {
            *ist_pos.offset(prev as isize) as libc::c_int
        }) as uint8_t;
        i += 1
    }
    L3_stereo_process(
        left,
        ist_pos,
        (*gr).sfbtab,
        hdr,
        max_band.as_mut_ptr(),
        (*gr.offset(1isize)).scalefac_compress as libc::c_int & 1i32,
    );
}
pub unsafe fn L3_stereo_process(
    mut left: *mut libc::c_float,
    mut ist_pos: *const uint8_t,
    mut sfb: *const uint8_t,
    mut hdr: *const uint8_t,
    mut max_band: *mut libc::c_int,
    mut mpeg2_sh: libc::c_int,
) {
    static mut g_pan: [libc::c_float; 14] = [
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        0.21132487058639527f32,
        0.7886751294136047f32,
        0.3660253882408142f32,
        0.6339746117591858f32,
        0.5f32,
        0.5f32,
        0.6339746117591858f32,
        0.3660253882408142f32,
        0.7886751294136047f32,
        0.21132487058639527f32,
        1i32 as libc::c_float,
        0i32 as libc::c_float,
    ];
    let mut i: libc::c_uint = 0;
    let mut max_pos: libc::c_uint = (if 0 != *hdr.offset(1isize) as libc::c_int & 0x8i32 {
        7i32
    } else {
        64i32
    }) as libc::c_uint;
    i = 0i32 as libc::c_uint;
    while 0 != *sfb.offset(i as isize) {
        let mut ipos: libc::c_uint = *ist_pos.offset(i as isize) as libc::c_uint;
        if i as libc::c_int > *max_band.offset(i.wrapping_rem(3i32 as libc::c_uint) as isize)
            && ipos < max_pos
        {
            let mut kl: libc::c_float = 0.;
            let mut kr: libc::c_float = 0.;
            let mut s: libc::c_float = if 0 != *hdr.offset(3isize) as libc::c_int & 0x20i32 {
                1.4142135381698609f32
            } else {
                1i32 as libc::c_float
            };
            if 0 != *hdr.offset(1isize) as libc::c_int & 0x8i32 {
                kl = g_pan[(2i32 as libc::c_uint).wrapping_mul(ipos) as usize];
                kr = g_pan[(2i32 as libc::c_uint)
                               .wrapping_mul(ipos)
                               .wrapping_add(1i32 as libc::c_uint)
                               as usize]
            } else {
                kl = 1i32 as libc::c_float;
                kr = L3_ldexp_q2(
                    1i32 as libc::c_float,
                    (ipos.wrapping_add(1i32 as libc::c_uint) >> 1i32 << mpeg2_sh) as libc::c_int,
                );
                if 0 != ipos & 1i32 as libc::c_uint {
                    kl = kr;
                    kr = 1i32 as libc::c_float
                }
            }
            L3_intensity_stereo_band(left, *sfb.offset(i as isize) as libc::c_int, kl * s, kr * s);
        } else if 0 != *hdr.offset(3isize) as libc::c_int & 0x20i32 {
            L3_midside_stereo(left, *sfb.offset(i as isize) as libc::c_int);
        }
        left = left.offset(*sfb.offset(i as isize) as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
}
pub unsafe fn L3_intensity_stereo_band(
    mut left: *mut libc::c_float,
    mut n: libc::c_int,
    mut kl: libc::c_float,
    mut kr: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < n {
        *left.offset((i + 576i32) as isize) = *left.offset(i as isize) * kr;
        *left.offset(i as isize) = *left.offset(i as isize) * kl;
        i += 1
    }
}
pub unsafe fn L3_ldexp_q2(
    mut y: libc::c_float,
    mut exp_q2: libc::c_int,
) -> libc::c_float {
    static mut g_expfrac: [libc::c_float; 4] = [
        9.313225746154786e-10f32,
        7.831458259666135e-10f32,
        6.5854449671221e-10f32,
        5.53767698363572e-10f32,
    ];
    let mut e: libc::c_int = 0;
    loop {
        e = if 30i32 * 4i32 > exp_q2 {
            exp_q2
        } else {
            30i32 * 4i32
        };
        y *= g_expfrac[(e & 3i32) as usize] * (1i32 << 30i32 >> (e >> 2i32)) as libc::c_float;
        exp_q2 -= e;
        if !(exp_q2 > 0i32) {
            break;
        }
    }
    return y;
}
pub unsafe fn L3_stereo_top_band(
    mut right: *const libc::c_float,
    mut sfb: *const uint8_t,
    mut nbands: libc::c_int,
    mut max_band: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let ref mut fresh24 = *max_band.offset(1isize);
    let ref mut fresh23 = *max_band.offset(2isize);
    *fresh23 = -1i32;
    *fresh24 = *fresh23;
    *max_band.offset(0isize) = *fresh24;
    i = 0i32;
    while i < nbands {
        k = 0i32;
        while k < *sfb.offset(i as isize) as libc::c_int {
            if *right.offset(k as isize) != 0i32 as libc::c_float
                || *right.offset((k + 1i32) as isize) != 0i32 as libc::c_float
            {
                *max_band.offset((i % 3i32) as isize) = i;
                break;
            } else {
                k += 2i32
            }
        }
        right = right.offset(*sfb.offset(i as isize) as libc::c_int as isize);
        i += 1
    }
}
pub unsafe fn L3_huffman(
    mut dst: *mut libc::c_float,
    mut bs: *mut bs_t,
    mut gr_info: *const L3_gr_info_t,
    mut scf: *const libc::c_float,
    mut layer3gr_limit: libc::c_int,
) {
    static mut tabs: [int16_t; 2164] = [
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        0i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        513i32 as int16_t,
        513i32 as int16_t,
        513i32 as int16_t,
        513i32 as int16_t,
        513i32 as int16_t,
        513i32 as int16_t,
        513i32 as int16_t,
        513i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        -255i32 as int16_t,
        1313i32 as int16_t,
        1298i32 as int16_t,
        1282i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        290i32 as int16_t,
        288i32 as int16_t,
        -255i32 as int16_t,
        1313i32 as int16_t,
        1298i32 as int16_t,
        1282i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        528i32 as int16_t,
        528i32 as int16_t,
        528i32 as int16_t,
        528i32 as int16_t,
        528i32 as int16_t,
        528i32 as int16_t,
        528i32 as int16_t,
        528i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        290i32 as int16_t,
        288i32 as int16_t,
        -253i32 as int16_t,
        -318i32 as int16_t,
        -351i32 as int16_t,
        -367i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        819i32 as int16_t,
        818i32 as int16_t,
        547i32 as int16_t,
        547i32 as int16_t,
        275i32 as int16_t,
        275i32 as int16_t,
        275i32 as int16_t,
        275i32 as int16_t,
        561i32 as int16_t,
        560i32 as int16_t,
        515i32 as int16_t,
        546i32 as int16_t,
        289i32 as int16_t,
        274i32 as int16_t,
        288i32 as int16_t,
        258i32 as int16_t,
        -254i32 as int16_t,
        -287i32 as int16_t,
        1329i32 as int16_t,
        1299i32 as int16_t,
        1314i32 as int16_t,
        1312i32 as int16_t,
        1057i32 as int16_t,
        1057i32 as int16_t,
        1042i32 as int16_t,
        1042i32 as int16_t,
        1026i32 as int16_t,
        1026i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        768i32 as int16_t,
        768i32 as int16_t,
        768i32 as int16_t,
        768i32 as int16_t,
        563i32 as int16_t,
        560i32 as int16_t,
        306i32 as int16_t,
        306i32 as int16_t,
        291i32 as int16_t,
        259i32 as int16_t,
        -252i32 as int16_t,
        -413i32 as int16_t,
        -477i32 as int16_t,
        -542i32 as int16_t,
        1298i32 as int16_t,
        -575i32 as int16_t,
        1041i32 as int16_t,
        1041i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        -383i32 as int16_t,
        -399i32 as int16_t,
        1107i32 as int16_t,
        1092i32 as int16_t,
        1106i32 as int16_t,
        1061i32 as int16_t,
        849i32 as int16_t,
        849i32 as int16_t,
        789i32 as int16_t,
        789i32 as int16_t,
        1104i32 as int16_t,
        1091i32 as int16_t,
        773i32 as int16_t,
        773i32 as int16_t,
        1076i32 as int16_t,
        1075i32 as int16_t,
        341i32 as int16_t,
        340i32 as int16_t,
        325i32 as int16_t,
        309i32 as int16_t,
        834i32 as int16_t,
        804i32 as int16_t,
        577i32 as int16_t,
        577i32 as int16_t,
        532i32 as int16_t,
        532i32 as int16_t,
        516i32 as int16_t,
        516i32 as int16_t,
        832i32 as int16_t,
        818i32 as int16_t,
        803i32 as int16_t,
        816i32 as int16_t,
        561i32 as int16_t,
        561i32 as int16_t,
        531i32 as int16_t,
        531i32 as int16_t,
        515i32 as int16_t,
        546i32 as int16_t,
        289i32 as int16_t,
        289i32 as int16_t,
        288i32 as int16_t,
        258i32 as int16_t,
        -252i32 as int16_t,
        -429i32 as int16_t,
        -493i32 as int16_t,
        -559i32 as int16_t,
        1057i32 as int16_t,
        1057i32 as int16_t,
        1042i32 as int16_t,
        1042i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        529i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        -382i32 as int16_t,
        1077i32 as int16_t,
        -415i32 as int16_t,
        1106i32 as int16_t,
        1061i32 as int16_t,
        1104i32 as int16_t,
        849i32 as int16_t,
        849i32 as int16_t,
        789i32 as int16_t,
        789i32 as int16_t,
        1091i32 as int16_t,
        1076i32 as int16_t,
        1029i32 as int16_t,
        1075i32 as int16_t,
        834i32 as int16_t,
        834i32 as int16_t,
        597i32 as int16_t,
        581i32 as int16_t,
        340i32 as int16_t,
        340i32 as int16_t,
        339i32 as int16_t,
        324i32 as int16_t,
        804i32 as int16_t,
        833i32 as int16_t,
        532i32 as int16_t,
        532i32 as int16_t,
        832i32 as int16_t,
        772i32 as int16_t,
        818i32 as int16_t,
        803i32 as int16_t,
        817i32 as int16_t,
        787i32 as int16_t,
        816i32 as int16_t,
        771i32 as int16_t,
        290i32 as int16_t,
        290i32 as int16_t,
        290i32 as int16_t,
        290i32 as int16_t,
        288i32 as int16_t,
        258i32 as int16_t,
        -253i32 as int16_t,
        -349i32 as int16_t,
        -414i32 as int16_t,
        -447i32 as int16_t,
        -463i32 as int16_t,
        1329i32 as int16_t,
        1299i32 as int16_t,
        -479i32 as int16_t,
        1314i32 as int16_t,
        1312i32 as int16_t,
        1057i32 as int16_t,
        1057i32 as int16_t,
        1042i32 as int16_t,
        1042i32 as int16_t,
        1026i32 as int16_t,
        1026i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        768i32 as int16_t,
        768i32 as int16_t,
        768i32 as int16_t,
        768i32 as int16_t,
        -319i32 as int16_t,
        851i32 as int16_t,
        821i32 as int16_t,
        -335i32 as int16_t,
        836i32 as int16_t,
        850i32 as int16_t,
        805i32 as int16_t,
        849i32 as int16_t,
        341i32 as int16_t,
        340i32 as int16_t,
        325i32 as int16_t,
        336i32 as int16_t,
        533i32 as int16_t,
        533i32 as int16_t,
        579i32 as int16_t,
        579i32 as int16_t,
        564i32 as int16_t,
        564i32 as int16_t,
        773i32 as int16_t,
        832i32 as int16_t,
        578i32 as int16_t,
        548i32 as int16_t,
        563i32 as int16_t,
        516i32 as int16_t,
        321i32 as int16_t,
        276i32 as int16_t,
        306i32 as int16_t,
        291i32 as int16_t,
        304i32 as int16_t,
        259i32 as int16_t,
        -251i32 as int16_t,
        -572i32 as int16_t,
        -733i32 as int16_t,
        -830i32 as int16_t,
        -863i32 as int16_t,
        -879i32 as int16_t,
        1041i32 as int16_t,
        1041i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        -511i32 as int16_t,
        -527i32 as int16_t,
        -543i32 as int16_t,
        1396i32 as int16_t,
        1351i32 as int16_t,
        1381i32 as int16_t,
        1366i32 as int16_t,
        1395i32 as int16_t,
        1335i32 as int16_t,
        1380i32 as int16_t,
        -559i32 as int16_t,
        1334i32 as int16_t,
        1138i32 as int16_t,
        1138i32 as int16_t,
        1063i32 as int16_t,
        1063i32 as int16_t,
        1350i32 as int16_t,
        1392i32 as int16_t,
        1031i32 as int16_t,
        1031i32 as int16_t,
        1062i32 as int16_t,
        1062i32 as int16_t,
        1364i32 as int16_t,
        1363i32 as int16_t,
        1120i32 as int16_t,
        1120i32 as int16_t,
        1333i32 as int16_t,
        1348i32 as int16_t,
        881i32 as int16_t,
        881i32 as int16_t,
        881i32 as int16_t,
        881i32 as int16_t,
        375i32 as int16_t,
        374i32 as int16_t,
        359i32 as int16_t,
        373i32 as int16_t,
        343i32 as int16_t,
        358i32 as int16_t,
        341i32 as int16_t,
        325i32 as int16_t,
        791i32 as int16_t,
        791i32 as int16_t,
        1123i32 as int16_t,
        1122i32 as int16_t,
        -703i32 as int16_t,
        1105i32 as int16_t,
        1045i32 as int16_t,
        -719i32 as int16_t,
        865i32 as int16_t,
        865i32 as int16_t,
        790i32 as int16_t,
        790i32 as int16_t,
        774i32 as int16_t,
        774i32 as int16_t,
        1104i32 as int16_t,
        1029i32 as int16_t,
        338i32 as int16_t,
        293i32 as int16_t,
        323i32 as int16_t,
        308i32 as int16_t,
        -799i32 as int16_t,
        -815i32 as int16_t,
        833i32 as int16_t,
        788i32 as int16_t,
        772i32 as int16_t,
        818i32 as int16_t,
        803i32 as int16_t,
        816i32 as int16_t,
        322i32 as int16_t,
        292i32 as int16_t,
        307i32 as int16_t,
        320i32 as int16_t,
        561i32 as int16_t,
        531i32 as int16_t,
        515i32 as int16_t,
        546i32 as int16_t,
        289i32 as int16_t,
        274i32 as int16_t,
        288i32 as int16_t,
        258i32 as int16_t,
        -251i32 as int16_t,
        -525i32 as int16_t,
        -605i32 as int16_t,
        -685i32 as int16_t,
        -765i32 as int16_t,
        -831i32 as int16_t,
        -846i32 as int16_t,
        1298i32 as int16_t,
        1057i32 as int16_t,
        1057i32 as int16_t,
        1312i32 as int16_t,
        1282i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        512i32 as int16_t,
        1399i32 as int16_t,
        1398i32 as int16_t,
        1383i32 as int16_t,
        1367i32 as int16_t,
        1382i32 as int16_t,
        1396i32 as int16_t,
        1351i32 as int16_t,
        -511i32 as int16_t,
        1381i32 as int16_t,
        1366i32 as int16_t,
        1139i32 as int16_t,
        1139i32 as int16_t,
        1079i32 as int16_t,
        1079i32 as int16_t,
        1124i32 as int16_t,
        1124i32 as int16_t,
        1364i32 as int16_t,
        1349i32 as int16_t,
        1363i32 as int16_t,
        1333i32 as int16_t,
        882i32 as int16_t,
        882i32 as int16_t,
        882i32 as int16_t,
        882i32 as int16_t,
        807i32 as int16_t,
        807i32 as int16_t,
        807i32 as int16_t,
        807i32 as int16_t,
        1094i32 as int16_t,
        1094i32 as int16_t,
        1136i32 as int16_t,
        1136i32 as int16_t,
        373i32 as int16_t,
        341i32 as int16_t,
        535i32 as int16_t,
        535i32 as int16_t,
        881i32 as int16_t,
        775i32 as int16_t,
        867i32 as int16_t,
        822i32 as int16_t,
        774i32 as int16_t,
        -591i32 as int16_t,
        324i32 as int16_t,
        338i32 as int16_t,
        -671i32 as int16_t,
        849i32 as int16_t,
        550i32 as int16_t,
        550i32 as int16_t,
        866i32 as int16_t,
        864i32 as int16_t,
        609i32 as int16_t,
        609i32 as int16_t,
        293i32 as int16_t,
        336i32 as int16_t,
        534i32 as int16_t,
        534i32 as int16_t,
        789i32 as int16_t,
        835i32 as int16_t,
        773i32 as int16_t,
        -751i32 as int16_t,
        834i32 as int16_t,
        804i32 as int16_t,
        308i32 as int16_t,
        307i32 as int16_t,
        833i32 as int16_t,
        788i32 as int16_t,
        832i32 as int16_t,
        772i32 as int16_t,
        562i32 as int16_t,
        562i32 as int16_t,
        547i32 as int16_t,
        547i32 as int16_t,
        305i32 as int16_t,
        275i32 as int16_t,
        560i32 as int16_t,
        515i32 as int16_t,
        290i32 as int16_t,
        290i32 as int16_t,
        -252i32 as int16_t,
        -397i32 as int16_t,
        -477i32 as int16_t,
        -557i32 as int16_t,
        -622i32 as int16_t,
        -653i32 as int16_t,
        -719i32 as int16_t,
        -735i32 as int16_t,
        -750i32 as int16_t,
        1329i32 as int16_t,
        1299i32 as int16_t,
        1314i32 as int16_t,
        1057i32 as int16_t,
        1057i32 as int16_t,
        1042i32 as int16_t,
        1042i32 as int16_t,
        1312i32 as int16_t,
        1282i32 as int16_t,
        1024i32 as int16_t,
        1024i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        784i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        -383i32 as int16_t,
        1127i32 as int16_t,
        1141i32 as int16_t,
        1111i32 as int16_t,
        1126i32 as int16_t,
        1140i32 as int16_t,
        1095i32 as int16_t,
        1110i32 as int16_t,
        869i32 as int16_t,
        869i32 as int16_t,
        883i32 as int16_t,
        883i32 as int16_t,
        1079i32 as int16_t,
        1109i32 as int16_t,
        882i32 as int16_t,
        882i32 as int16_t,
        375i32 as int16_t,
        374i32 as int16_t,
        807i32 as int16_t,
        868i32 as int16_t,
        838i32 as int16_t,
        881i32 as int16_t,
        791i32 as int16_t,
        -463i32 as int16_t,
        867i32 as int16_t,
        822i32 as int16_t,
        368i32 as int16_t,
        263i32 as int16_t,
        852i32 as int16_t,
        837i32 as int16_t,
        836i32 as int16_t,
        -543i32 as int16_t,
        610i32 as int16_t,
        610i32 as int16_t,
        550i32 as int16_t,
        550i32 as int16_t,
        352i32 as int16_t,
        336i32 as int16_t,
        534i32 as int16_t,
        534i32 as int16_t,
        865i32 as int16_t,
        774i32 as int16_t,
        851i32 as int16_t,
        821i32 as int16_t,
        850i32 as int16_t,
        805i32 as int16_t,
        593i32 as int16_t,
        533i32 as int16_t,
        579i32 as int16_t,
        564i32 as int16_t,
        773i32 as int16_t,
        832i32 as int16_t,
        578i32 as int16_t,
        578i32 as int16_t,
        548i32 as int16_t,
        548i32 as int16_t,
        577i32 as int16_t,
        577i32 as int16_t,
        307i32 as int16_t,
        276i32 as int16_t,
        306i32 as int16_t,
        291i32 as int16_t,
        516i32 as int16_t,
        560i32 as int16_t,
        259i32 as int16_t,
        259i32 as int16_t,
        -250i32 as int16_t,
        -2107i32 as int16_t,
        -2507i32 as int16_t,
        -2764i32 as int16_t,
        -2909i32 as int16_t,
        -2974i32 as int16_t,
        -3007i32 as int16_t,
        -3023i32 as int16_t,
        1041i32 as int16_t,
        1041i32 as int16_t,
        1040i32 as int16_t,
        1040i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        -767i32 as int16_t,
        -1052i32 as int16_t,
        -1213i32 as int16_t,
        -1277i32 as int16_t,
        -1358i32 as int16_t,
        -1405i32 as int16_t,
        -1469i32 as int16_t,
        -1535i32 as int16_t,
        -1550i32 as int16_t,
        -1582i32 as int16_t,
        -1614i32 as int16_t,
        -1647i32 as int16_t,
        -1662i32 as int16_t,
        -1694i32 as int16_t,
        -1726i32 as int16_t,
        -1759i32 as int16_t,
        -1774i32 as int16_t,
        -1807i32 as int16_t,
        -1822i32 as int16_t,
        -1854i32 as int16_t,
        -1886i32 as int16_t,
        1565i32 as int16_t,
        -1919i32 as int16_t,
        -1935i32 as int16_t,
        -1951i32 as int16_t,
        -1967i32 as int16_t,
        1731i32 as int16_t,
        1730i32 as int16_t,
        1580i32 as int16_t,
        1717i32 as int16_t,
        -1983i32 as int16_t,
        1729i32 as int16_t,
        1564i32 as int16_t,
        -1999i32 as int16_t,
        1548i32 as int16_t,
        -2015i32 as int16_t,
        -2031i32 as int16_t,
        1715i32 as int16_t,
        1595i32 as int16_t,
        -2047i32 as int16_t,
        1714i32 as int16_t,
        -2063i32 as int16_t,
        1610i32 as int16_t,
        -2079i32 as int16_t,
        1609i32 as int16_t,
        -2095i32 as int16_t,
        1323i32 as int16_t,
        1323i32 as int16_t,
        1457i32 as int16_t,
        1457i32 as int16_t,
        1307i32 as int16_t,
        1307i32 as int16_t,
        1712i32 as int16_t,
        1547i32 as int16_t,
        1641i32 as int16_t,
        1700i32 as int16_t,
        1699i32 as int16_t,
        1594i32 as int16_t,
        1685i32 as int16_t,
        1625i32 as int16_t,
        1442i32 as int16_t,
        1442i32 as int16_t,
        1322i32 as int16_t,
        1322i32 as int16_t,
        -780i32 as int16_t,
        -973i32 as int16_t,
        -910i32 as int16_t,
        1279i32 as int16_t,
        1278i32 as int16_t,
        1277i32 as int16_t,
        1262i32 as int16_t,
        1276i32 as int16_t,
        1261i32 as int16_t,
        1275i32 as int16_t,
        1215i32 as int16_t,
        1260i32 as int16_t,
        1229i32 as int16_t,
        -959i32 as int16_t,
        974i32 as int16_t,
        974i32 as int16_t,
        989i32 as int16_t,
        989i32 as int16_t,
        -943i32 as int16_t,
        735i32 as int16_t,
        478i32 as int16_t,
        478i32 as int16_t,
        495i32 as int16_t,
        463i32 as int16_t,
        506i32 as int16_t,
        414i32 as int16_t,
        -1039i32 as int16_t,
        1003i32 as int16_t,
        958i32 as int16_t,
        1017i32 as int16_t,
        927i32 as int16_t,
        942i32 as int16_t,
        987i32 as int16_t,
        957i32 as int16_t,
        431i32 as int16_t,
        476i32 as int16_t,
        1272i32 as int16_t,
        1167i32 as int16_t,
        1228i32 as int16_t,
        -1183i32 as int16_t,
        1256i32 as int16_t,
        -1199i32 as int16_t,
        895i32 as int16_t,
        895i32 as int16_t,
        941i32 as int16_t,
        941i32 as int16_t,
        1242i32 as int16_t,
        1227i32 as int16_t,
        1212i32 as int16_t,
        1135i32 as int16_t,
        1014i32 as int16_t,
        1014i32 as int16_t,
        490i32 as int16_t,
        489i32 as int16_t,
        503i32 as int16_t,
        487i32 as int16_t,
        910i32 as int16_t,
        1013i32 as int16_t,
        985i32 as int16_t,
        925i32 as int16_t,
        863i32 as int16_t,
        894i32 as int16_t,
        970i32 as int16_t,
        955i32 as int16_t,
        1012i32 as int16_t,
        847i32 as int16_t,
        -1343i32 as int16_t,
        831i32 as int16_t,
        755i32 as int16_t,
        755i32 as int16_t,
        984i32 as int16_t,
        909i32 as int16_t,
        428i32 as int16_t,
        366i32 as int16_t,
        754i32 as int16_t,
        559i32 as int16_t,
        -1391i32 as int16_t,
        752i32 as int16_t,
        486i32 as int16_t,
        457i32 as int16_t,
        924i32 as int16_t,
        997i32 as int16_t,
        698i32 as int16_t,
        698i32 as int16_t,
        983i32 as int16_t,
        893i32 as int16_t,
        740i32 as int16_t,
        740i32 as int16_t,
        908i32 as int16_t,
        877i32 as int16_t,
        739i32 as int16_t,
        739i32 as int16_t,
        667i32 as int16_t,
        667i32 as int16_t,
        953i32 as int16_t,
        938i32 as int16_t,
        497i32 as int16_t,
        287i32 as int16_t,
        271i32 as int16_t,
        271i32 as int16_t,
        683i32 as int16_t,
        606i32 as int16_t,
        590i32 as int16_t,
        712i32 as int16_t,
        726i32 as int16_t,
        574i32 as int16_t,
        302i32 as int16_t,
        302i32 as int16_t,
        738i32 as int16_t,
        736i32 as int16_t,
        481i32 as int16_t,
        286i32 as int16_t,
        526i32 as int16_t,
        725i32 as int16_t,
        605i32 as int16_t,
        711i32 as int16_t,
        636i32 as int16_t,
        724i32 as int16_t,
        696i32 as int16_t,
        651i32 as int16_t,
        589i32 as int16_t,
        681i32 as int16_t,
        666i32 as int16_t,
        710i32 as int16_t,
        364i32 as int16_t,
        467i32 as int16_t,
        573i32 as int16_t,
        695i32 as int16_t,
        466i32 as int16_t,
        466i32 as int16_t,
        301i32 as int16_t,
        465i32 as int16_t,
        379i32 as int16_t,
        379i32 as int16_t,
        709i32 as int16_t,
        604i32 as int16_t,
        665i32 as int16_t,
        679i32 as int16_t,
        316i32 as int16_t,
        316i32 as int16_t,
        634i32 as int16_t,
        633i32 as int16_t,
        436i32 as int16_t,
        436i32 as int16_t,
        464i32 as int16_t,
        269i32 as int16_t,
        424i32 as int16_t,
        394i32 as int16_t,
        452i32 as int16_t,
        332i32 as int16_t,
        438i32 as int16_t,
        363i32 as int16_t,
        347i32 as int16_t,
        408i32 as int16_t,
        393i32 as int16_t,
        448i32 as int16_t,
        331i32 as int16_t,
        422i32 as int16_t,
        362i32 as int16_t,
        407i32 as int16_t,
        392i32 as int16_t,
        421i32 as int16_t,
        346i32 as int16_t,
        406i32 as int16_t,
        391i32 as int16_t,
        376i32 as int16_t,
        375i32 as int16_t,
        359i32 as int16_t,
        1441i32 as int16_t,
        1306i32 as int16_t,
        -2367i32 as int16_t,
        1290i32 as int16_t,
        -2383i32 as int16_t,
        1337i32 as int16_t,
        -2399i32 as int16_t,
        -2415i32 as int16_t,
        1426i32 as int16_t,
        1321i32 as int16_t,
        -2431i32 as int16_t,
        1411i32 as int16_t,
        1336i32 as int16_t,
        -2447i32 as int16_t,
        -2463i32 as int16_t,
        -2479i32 as int16_t,
        1169i32 as int16_t,
        1169i32 as int16_t,
        1049i32 as int16_t,
        1049i32 as int16_t,
        1424i32 as int16_t,
        1289i32 as int16_t,
        1412i32 as int16_t,
        1352i32 as int16_t,
        1319i32 as int16_t,
        -2495i32 as int16_t,
        1154i32 as int16_t,
        1154i32 as int16_t,
        1064i32 as int16_t,
        1064i32 as int16_t,
        1153i32 as int16_t,
        1153i32 as int16_t,
        416i32 as int16_t,
        390i32 as int16_t,
        360i32 as int16_t,
        404i32 as int16_t,
        403i32 as int16_t,
        389i32 as int16_t,
        344i32 as int16_t,
        374i32 as int16_t,
        373i32 as int16_t,
        343i32 as int16_t,
        358i32 as int16_t,
        372i32 as int16_t,
        327i32 as int16_t,
        357i32 as int16_t,
        342i32 as int16_t,
        311i32 as int16_t,
        356i32 as int16_t,
        326i32 as int16_t,
        1395i32 as int16_t,
        1394i32 as int16_t,
        1137i32 as int16_t,
        1137i32 as int16_t,
        1047i32 as int16_t,
        1047i32 as int16_t,
        1365i32 as int16_t,
        1392i32 as int16_t,
        1287i32 as int16_t,
        1379i32 as int16_t,
        1334i32 as int16_t,
        1364i32 as int16_t,
        1349i32 as int16_t,
        1378i32 as int16_t,
        1318i32 as int16_t,
        1363i32 as int16_t,
        792i32 as int16_t,
        792i32 as int16_t,
        792i32 as int16_t,
        792i32 as int16_t,
        1152i32 as int16_t,
        1152i32 as int16_t,
        1032i32 as int16_t,
        1032i32 as int16_t,
        1121i32 as int16_t,
        1121i32 as int16_t,
        1046i32 as int16_t,
        1046i32 as int16_t,
        1120i32 as int16_t,
        1120i32 as int16_t,
        1030i32 as int16_t,
        1030i32 as int16_t,
        -2895i32 as int16_t,
        1106i32 as int16_t,
        1061i32 as int16_t,
        1104i32 as int16_t,
        849i32 as int16_t,
        849i32 as int16_t,
        789i32 as int16_t,
        789i32 as int16_t,
        1091i32 as int16_t,
        1076i32 as int16_t,
        1029i32 as int16_t,
        1090i32 as int16_t,
        1060i32 as int16_t,
        1075i32 as int16_t,
        833i32 as int16_t,
        833i32 as int16_t,
        309i32 as int16_t,
        324i32 as int16_t,
        532i32 as int16_t,
        532i32 as int16_t,
        832i32 as int16_t,
        772i32 as int16_t,
        818i32 as int16_t,
        803i32 as int16_t,
        561i32 as int16_t,
        561i32 as int16_t,
        531i32 as int16_t,
        560i32 as int16_t,
        515i32 as int16_t,
        546i32 as int16_t,
        289i32 as int16_t,
        274i32 as int16_t,
        288i32 as int16_t,
        258i32 as int16_t,
        -250i32 as int16_t,
        -1179i32 as int16_t,
        -1579i32 as int16_t,
        -1836i32 as int16_t,
        -1996i32 as int16_t,
        -2124i32 as int16_t,
        -2253i32 as int16_t,
        -2333i32 as int16_t,
        -2413i32 as int16_t,
        -2477i32 as int16_t,
        -2542i32 as int16_t,
        -2574i32 as int16_t,
        -2607i32 as int16_t,
        -2622i32 as int16_t,
        -2655i32 as int16_t,
        1314i32 as int16_t,
        1313i32 as int16_t,
        1298i32 as int16_t,
        1312i32 as int16_t,
        1282i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        785i32 as int16_t,
        1040i32 as int16_t,
        1040i32 as int16_t,
        1025i32 as int16_t,
        1025i32 as int16_t,
        768i32 as int16_t,
        768i32 as int16_t,
        768i32 as int16_t,
        768i32 as int16_t,
        -766i32 as int16_t,
        -798i32 as int16_t,
        -830i32 as int16_t,
        -862i32 as int16_t,
        -895i32 as int16_t,
        -911i32 as int16_t,
        -927i32 as int16_t,
        -943i32 as int16_t,
        -959i32 as int16_t,
        -975i32 as int16_t,
        -991i32 as int16_t,
        -1007i32 as int16_t,
        -1023i32 as int16_t,
        -1039i32 as int16_t,
        -1055i32 as int16_t,
        -1070i32 as int16_t,
        1724i32 as int16_t,
        1647i32 as int16_t,
        -1103i32 as int16_t,
        -1119i32 as int16_t,
        1631i32 as int16_t,
        1767i32 as int16_t,
        1662i32 as int16_t,
        1738i32 as int16_t,
        1708i32 as int16_t,
        1723i32 as int16_t,
        -1135i32 as int16_t,
        1780i32 as int16_t,
        1615i32 as int16_t,
        1779i32 as int16_t,
        1599i32 as int16_t,
        1677i32 as int16_t,
        1646i32 as int16_t,
        1778i32 as int16_t,
        1583i32 as int16_t,
        -1151i32 as int16_t,
        1777i32 as int16_t,
        1567i32 as int16_t,
        1737i32 as int16_t,
        1692i32 as int16_t,
        1765i32 as int16_t,
        1722i32 as int16_t,
        1707i32 as int16_t,
        1630i32 as int16_t,
        1751i32 as int16_t,
        1661i32 as int16_t,
        1764i32 as int16_t,
        1614i32 as int16_t,
        1736i32 as int16_t,
        1676i32 as int16_t,
        1763i32 as int16_t,
        1750i32 as int16_t,
        1645i32 as int16_t,
        1598i32 as int16_t,
        1721i32 as int16_t,
        1691i32 as int16_t,
        1762i32 as int16_t,
        1706i32 as int16_t,
        1582i32 as int16_t,
        1761i32 as int16_t,
        1566i32 as int16_t,
        -1167i32 as int16_t,
        1749i32 as int16_t,
        1629i32 as int16_t,
        767i32 as int16_t,
        766i32 as int16_t,
        751i32 as int16_t,
        765i32 as int16_t,
        494i32 as int16_t,
        494i32 as int16_t,
        735i32 as int16_t,
        764i32 as int16_t,
        719i32 as int16_t,
        749i32 as int16_t,
        734i32 as int16_t,
        763i32 as int16_t,
        447i32 as int16_t,
        447i32 as int16_t,
        748i32 as int16_t,
        718i32 as int16_t,
        477i32 as int16_t,
        506i32 as int16_t,
        431i32 as int16_t,
        491i32 as int16_t,
        446i32 as int16_t,
        476i32 as int16_t,
        461i32 as int16_t,
        505i32 as int16_t,
        415i32 as int16_t,
        430i32 as int16_t,
        475i32 as int16_t,
        445i32 as int16_t,
        504i32 as int16_t,
        399i32 as int16_t,
        460i32 as int16_t,
        489i32 as int16_t,
        414i32 as int16_t,
        503i32 as int16_t,
        383i32 as int16_t,
        474i32 as int16_t,
        429i32 as int16_t,
        459i32 as int16_t,
        502i32 as int16_t,
        502i32 as int16_t,
        746i32 as int16_t,
        752i32 as int16_t,
        488i32 as int16_t,
        398i32 as int16_t,
        501i32 as int16_t,
        473i32 as int16_t,
        413i32 as int16_t,
        472i32 as int16_t,
        486i32 as int16_t,
        271i32 as int16_t,
        480i32 as int16_t,
        270i32 as int16_t,
        -1439i32 as int16_t,
        -1455i32 as int16_t,
        1357i32 as int16_t,
        -1471i32 as int16_t,
        -1487i32 as int16_t,
        -1503i32 as int16_t,
        1341i32 as int16_t,
        1325i32 as int16_t,
        -1519i32 as int16_t,
        1489i32 as int16_t,
        1463i32 as int16_t,
        1403i32 as int16_t,
        1309i32 as int16_t,
        -1535i32 as int16_t,
        1372i32 as int16_t,
        1448i32 as int16_t,
        1418i32 as int16_t,
        1476i32 as int16_t,
        1356i32 as int16_t,
        1462i32 as int16_t,
        1387i32 as int16_t,
        -1551i32 as int16_t,
        1475i32 as int16_t,
        1340i32 as int16_t,
        1447i32 as int16_t,
        1402i32 as int16_t,
        1386i32 as int16_t,
        -1567i32 as int16_t,
        1068i32 as int16_t,
        1068i32 as int16_t,
        1474i32 as int16_t,
        1461i32 as int16_t,
        455i32 as int16_t,
        380i32 as int16_t,
        468i32 as int16_t,
        440i32 as int16_t,
        395i32 as int16_t,
        425i32 as int16_t,
        410i32 as int16_t,
        454i32 as int16_t,
        364i32 as int16_t,
        467i32 as int16_t,
        466i32 as int16_t,
        464i32 as int16_t,
        453i32 as int16_t,
        269i32 as int16_t,
        409i32 as int16_t,
        448i32 as int16_t,
        268i32 as int16_t,
        432i32 as int16_t,
        1371i32 as int16_t,
        1473i32 as int16_t,
        1432i32 as int16_t,
        1417i32 as int16_t,
        1308i32 as int16_t,
        1460i32 as int16_t,
        1355i32 as int16_t,
        1446i32 as int16_t,
        1459i32 as int16_t,
        1431i32 as int16_t,
        1083i32 as int16_t,
        1083i32 as int16_t,
        1401i32 as int16_t,
        1416i32 as int16_t,
        1458i32 as int16_t,
        1445i32 as int16_t,
        1067i32 as int16_t,
        1067i32 as int16_t,
        1370i32 as int16_t,
        1457i32 as int16_t,
        1051i32 as int16_t,
        1051i32 as int16_t,
        1291i32 as int16_t,
        1430i32 as int16_t,
        1385i32 as int16_t,
        1444i32 as int16_t,
        1354i32 as int16_t,
        1415i32 as int16_t,
        1400i32 as int16_t,
        1443i32 as int16_t,
        1082i32 as int16_t,
        1082i32 as int16_t,
        1173i32 as int16_t,
        1113i32 as int16_t,
        1186i32 as int16_t,
        1066i32 as int16_t,
        1185i32 as int16_t,
        1050i32 as int16_t,
        -1967i32 as int16_t,
        1158i32 as int16_t,
        1128i32 as int16_t,
        1172i32 as int16_t,
        1097i32 as int16_t,
        1171i32 as int16_t,
        1081i32 as int16_t,
        -1983i32 as int16_t,
        1157i32 as int16_t,
        1112i32 as int16_t,
        416i32 as int16_t,
        266i32 as int16_t,
        375i32 as int16_t,
        400i32 as int16_t,
        1170i32 as int16_t,
        1142i32 as int16_t,
        1127i32 as int16_t,
        1065i32 as int16_t,
        793i32 as int16_t,
        793i32 as int16_t,
        1169i32 as int16_t,
        1033i32 as int16_t,
        1156i32 as int16_t,
        1096i32 as int16_t,
        1141i32 as int16_t,
        1111i32 as int16_t,
        1155i32 as int16_t,
        1080i32 as int16_t,
        1126i32 as int16_t,
        1140i32 as int16_t,
        898i32 as int16_t,
        898i32 as int16_t,
        808i32 as int16_t,
        808i32 as int16_t,
        897i32 as int16_t,
        897i32 as int16_t,
        792i32 as int16_t,
        792i32 as int16_t,
        1095i32 as int16_t,
        1152i32 as int16_t,
        1032i32 as int16_t,
        1125i32 as int16_t,
        1110i32 as int16_t,
        1139i32 as int16_t,
        1079i32 as int16_t,
        1124i32 as int16_t,
        882i32 as int16_t,
        807i32 as int16_t,
        838i32 as int16_t,
        881i32 as int16_t,
        853i32 as int16_t,
        791i32 as int16_t,
        -2319i32 as int16_t,
        867i32 as int16_t,
        368i32 as int16_t,
        263i32 as int16_t,
        822i32 as int16_t,
        852i32 as int16_t,
        837i32 as int16_t,
        866i32 as int16_t,
        806i32 as int16_t,
        865i32 as int16_t,
        -2399i32 as int16_t,
        851i32 as int16_t,
        352i32 as int16_t,
        262i32 as int16_t,
        534i32 as int16_t,
        534i32 as int16_t,
        821i32 as int16_t,
        836i32 as int16_t,
        594i32 as int16_t,
        594i32 as int16_t,
        549i32 as int16_t,
        549i32 as int16_t,
        593i32 as int16_t,
        593i32 as int16_t,
        533i32 as int16_t,
        533i32 as int16_t,
        848i32 as int16_t,
        773i32 as int16_t,
        579i32 as int16_t,
        579i32 as int16_t,
        564i32 as int16_t,
        578i32 as int16_t,
        548i32 as int16_t,
        563i32 as int16_t,
        276i32 as int16_t,
        276i32 as int16_t,
        577i32 as int16_t,
        576i32 as int16_t,
        306i32 as int16_t,
        291i32 as int16_t,
        516i32 as int16_t,
        560i32 as int16_t,
        305i32 as int16_t,
        305i32 as int16_t,
        275i32 as int16_t,
        259i32 as int16_t,
        -251i32 as int16_t,
        -892i32 as int16_t,
        -2058i32 as int16_t,
        -2620i32 as int16_t,
        -2828i32 as int16_t,
        -2957i32 as int16_t,
        -3023i32 as int16_t,
        -3039i32 as int16_t,
        1041i32 as int16_t,
        1041i32 as int16_t,
        1040i32 as int16_t,
        1040i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        769i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        256i32 as int16_t,
        -511i32 as int16_t,
        -527i32 as int16_t,
        -543i32 as int16_t,
        -559i32 as int16_t,
        1530i32 as int16_t,
        -575i32 as int16_t,
        -591i32 as int16_t,
        1528i32 as int16_t,
        1527i32 as int16_t,
        1407i32 as int16_t,
        1526i32 as int16_t,
        1391i32 as int16_t,
        1023i32 as int16_t,
        1023i32 as int16_t,
        1023i32 as int16_t,
        1023i32 as int16_t,
        1525i32 as int16_t,
        1375i32 as int16_t,
        1268i32 as int16_t,
        1268i32 as int16_t,
        1103i32 as int16_t,
        1103i32 as int16_t,
        1087i32 as int16_t,
        1087i32 as int16_t,
        1039i32 as int16_t,
        1039i32 as int16_t,
        1523i32 as int16_t,
        -604i32 as int16_t,
        815i32 as int16_t,
        815i32 as int16_t,
        815i32 as int16_t,
        815i32 as int16_t,
        510i32 as int16_t,
        495i32 as int16_t,
        509i32 as int16_t,
        479i32 as int16_t,
        508i32 as int16_t,
        463i32 as int16_t,
        507i32 as int16_t,
        447i32 as int16_t,
        431i32 as int16_t,
        505i32 as int16_t,
        415i32 as int16_t,
        399i32 as int16_t,
        -734i32 as int16_t,
        -782i32 as int16_t,
        1262i32 as int16_t,
        -815i32 as int16_t,
        1259i32 as int16_t,
        1244i32 as int16_t,
        -831i32 as int16_t,
        1258i32 as int16_t,
        1228i32 as int16_t,
        -847i32 as int16_t,
        -863i32 as int16_t,
        1196i32 as int16_t,
        -879i32 as int16_t,
        1253i32 as int16_t,
        987i32 as int16_t,
        987i32 as int16_t,
        748i32 as int16_t,
        -767i32 as int16_t,
        493i32 as int16_t,
        493i32 as int16_t,
        462i32 as int16_t,
        477i32 as int16_t,
        414i32 as int16_t,
        414i32 as int16_t,
        686i32 as int16_t,
        669i32 as int16_t,
        478i32 as int16_t,
        446i32 as int16_t,
        461i32 as int16_t,
        445i32 as int16_t,
        474i32 as int16_t,
        429i32 as int16_t,
        487i32 as int16_t,
        458i32 as int16_t,
        412i32 as int16_t,
        471i32 as int16_t,
        1266i32 as int16_t,
        1264i32 as int16_t,
        1009i32 as int16_t,
        1009i32 as int16_t,
        799i32 as int16_t,
        799i32 as int16_t,
        -1019i32 as int16_t,
        -1276i32 as int16_t,
        -1452i32 as int16_t,
        -1581i32 as int16_t,
        -1677i32 as int16_t,
        -1757i32 as int16_t,
        -1821i32 as int16_t,
        -1886i32 as int16_t,
        -1933i32 as int16_t,
        -1997i32 as int16_t,
        1257i32 as int16_t,
        1257i32 as int16_t,
        1483i32 as int16_t,
        1468i32 as int16_t,
        1512i32 as int16_t,
        1422i32 as int16_t,
        1497i32 as int16_t,
        1406i32 as int16_t,
        1467i32 as int16_t,
        1496i32 as int16_t,
        1421i32 as int16_t,
        1510i32 as int16_t,
        1134i32 as int16_t,
        1134i32 as int16_t,
        1225i32 as int16_t,
        1225i32 as int16_t,
        1466i32 as int16_t,
        1451i32 as int16_t,
        1374i32 as int16_t,
        1405i32 as int16_t,
        1252i32 as int16_t,
        1252i32 as int16_t,
        1358i32 as int16_t,
        1480i32 as int16_t,
        1164i32 as int16_t,
        1164i32 as int16_t,
        1251i32 as int16_t,
        1251i32 as int16_t,
        1238i32 as int16_t,
        1238i32 as int16_t,
        1389i32 as int16_t,
        1465i32 as int16_t,
        -1407i32 as int16_t,
        1054i32 as int16_t,
        1101i32 as int16_t,
        -1423i32 as int16_t,
        1207i32 as int16_t,
        -1439i32 as int16_t,
        830i32 as int16_t,
        830i32 as int16_t,
        1248i32 as int16_t,
        1038i32 as int16_t,
        1237i32 as int16_t,
        1117i32 as int16_t,
        1223i32 as int16_t,
        1148i32 as int16_t,
        1236i32 as int16_t,
        1208i32 as int16_t,
        411i32 as int16_t,
        426i32 as int16_t,
        395i32 as int16_t,
        410i32 as int16_t,
        379i32 as int16_t,
        269i32 as int16_t,
        1193i32 as int16_t,
        1222i32 as int16_t,
        1132i32 as int16_t,
        1235i32 as int16_t,
        1221i32 as int16_t,
        1116i32 as int16_t,
        976i32 as int16_t,
        976i32 as int16_t,
        1192i32 as int16_t,
        1162i32 as int16_t,
        1177i32 as int16_t,
        1220i32 as int16_t,
        1131i32 as int16_t,
        1191i32 as int16_t,
        963i32 as int16_t,
        963i32 as int16_t,
        -1647i32 as int16_t,
        961i32 as int16_t,
        780i32 as int16_t,
        -1663i32 as int16_t,
        558i32 as int16_t,
        558i32 as int16_t,
        994i32 as int16_t,
        993i32 as int16_t,
        437i32 as int16_t,
        408i32 as int16_t,
        393i32 as int16_t,
        407i32 as int16_t,
        829i32 as int16_t,
        978i32 as int16_t,
        813i32 as int16_t,
        797i32 as int16_t,
        947i32 as int16_t,
        -1743i32 as int16_t,
        721i32 as int16_t,
        721i32 as int16_t,
        377i32 as int16_t,
        392i32 as int16_t,
        844i32 as int16_t,
        950i32 as int16_t,
        828i32 as int16_t,
        890i32 as int16_t,
        706i32 as int16_t,
        706i32 as int16_t,
        812i32 as int16_t,
        859i32 as int16_t,
        796i32 as int16_t,
        960i32 as int16_t,
        948i32 as int16_t,
        843i32 as int16_t,
        934i32 as int16_t,
        874i32 as int16_t,
        571i32 as int16_t,
        571i32 as int16_t,
        -1919i32 as int16_t,
        690i32 as int16_t,
        555i32 as int16_t,
        689i32 as int16_t,
        421i32 as int16_t,
        346i32 as int16_t,
        539i32 as int16_t,
        539i32 as int16_t,
        944i32 as int16_t,
        779i32 as int16_t,
        918i32 as int16_t,
        873i32 as int16_t,
        932i32 as int16_t,
        842i32 as int16_t,
        903i32 as int16_t,
        888i32 as int16_t,
        570i32 as int16_t,
        570i32 as int16_t,
        931i32 as int16_t,
        917i32 as int16_t,
        674i32 as int16_t,
        674i32 as int16_t,
        -2575i32 as int16_t,
        1562i32 as int16_t,
        -2591i32 as int16_t,
        1609i32 as int16_t,
        -2607i32 as int16_t,
        1654i32 as int16_t,
        1322i32 as int16_t,
        1322i32 as int16_t,
        1441i32 as int16_t,
        1441i32 as int16_t,
        1696i32 as int16_t,
        1546i32 as int16_t,
        1683i32 as int16_t,
        1593i32 as int16_t,
        1669i32 as int16_t,
        1624i32 as int16_t,
        1426i32 as int16_t,
        1426i32 as int16_t,
        1321i32 as int16_t,
        1321i32 as int16_t,
        1639i32 as int16_t,
        1680i32 as int16_t,
        1425i32 as int16_t,
        1425i32 as int16_t,
        1305i32 as int16_t,
        1305i32 as int16_t,
        1545i32 as int16_t,
        1668i32 as int16_t,
        1608i32 as int16_t,
        1623i32 as int16_t,
        1667i32 as int16_t,
        1592i32 as int16_t,
        1638i32 as int16_t,
        1666i32 as int16_t,
        1320i32 as int16_t,
        1320i32 as int16_t,
        1652i32 as int16_t,
        1607i32 as int16_t,
        1409i32 as int16_t,
        1409i32 as int16_t,
        1304i32 as int16_t,
        1304i32 as int16_t,
        1288i32 as int16_t,
        1288i32 as int16_t,
        1664i32 as int16_t,
        1637i32 as int16_t,
        1395i32 as int16_t,
        1395i32 as int16_t,
        1335i32 as int16_t,
        1335i32 as int16_t,
        1622i32 as int16_t,
        1636i32 as int16_t,
        1394i32 as int16_t,
        1394i32 as int16_t,
        1319i32 as int16_t,
        1319i32 as int16_t,
        1606i32 as int16_t,
        1621i32 as int16_t,
        1392i32 as int16_t,
        1392i32 as int16_t,
        1137i32 as int16_t,
        1137i32 as int16_t,
        1137i32 as int16_t,
        1137i32 as int16_t,
        345i32 as int16_t,
        390i32 as int16_t,
        360i32 as int16_t,
        375i32 as int16_t,
        404i32 as int16_t,
        373i32 as int16_t,
        1047i32 as int16_t,
        -2751i32 as int16_t,
        -2767i32 as int16_t,
        -2783i32 as int16_t,
        1062i32 as int16_t,
        1121i32 as int16_t,
        1046i32 as int16_t,
        -2799i32 as int16_t,
        1077i32 as int16_t,
        -2815i32 as int16_t,
        1106i32 as int16_t,
        1061i32 as int16_t,
        789i32 as int16_t,
        789i32 as int16_t,
        1105i32 as int16_t,
        1104i32 as int16_t,
        263i32 as int16_t,
        355i32 as int16_t,
        310i32 as int16_t,
        340i32 as int16_t,
        325i32 as int16_t,
        354i32 as int16_t,
        352i32 as int16_t,
        262i32 as int16_t,
        339i32 as int16_t,
        324i32 as int16_t,
        1091i32 as int16_t,
        1076i32 as int16_t,
        1029i32 as int16_t,
        1090i32 as int16_t,
        1060i32 as int16_t,
        1075i32 as int16_t,
        833i32 as int16_t,
        833i32 as int16_t,
        788i32 as int16_t,
        788i32 as int16_t,
        1088i32 as int16_t,
        1028i32 as int16_t,
        818i32 as int16_t,
        818i32 as int16_t,
        803i32 as int16_t,
        803i32 as int16_t,
        561i32 as int16_t,
        561i32 as int16_t,
        531i32 as int16_t,
        531i32 as int16_t,
        816i32 as int16_t,
        771i32 as int16_t,
        546i32 as int16_t,
        546i32 as int16_t,
        289i32 as int16_t,
        274i32 as int16_t,
        288i32 as int16_t,
        258i32 as int16_t,
        -253i32 as int16_t,
        -317i32 as int16_t,
        -381i32 as int16_t,
        -446i32 as int16_t,
        -478i32 as int16_t,
        -509i32 as int16_t,
        1279i32 as int16_t,
        1279i32 as int16_t,
        -811i32 as int16_t,
        -1179i32 as int16_t,
        -1451i32 as int16_t,
        -1756i32 as int16_t,
        -1900i32 as int16_t,
        -2028i32 as int16_t,
        -2189i32 as int16_t,
        -2253i32 as int16_t,
        -2333i32 as int16_t,
        -2414i32 as int16_t,
        -2445i32 as int16_t,
        -2511i32 as int16_t,
        -2526i32 as int16_t,
        1313i32 as int16_t,
        1298i32 as int16_t,
        -2559i32 as int16_t,
        1041i32 as int16_t,
        1041i32 as int16_t,
        1040i32 as int16_t,
        1040i32 as int16_t,
        1025i32 as int16_t,
        1025i32 as int16_t,
        1024i32 as int16_t,
        1024i32 as int16_t,
        1022i32 as int16_t,
        1007i32 as int16_t,
        1021i32 as int16_t,
        991i32 as int16_t,
        1020i32 as int16_t,
        975i32 as int16_t,
        1019i32 as int16_t,
        959i32 as int16_t,
        687i32 as int16_t,
        687i32 as int16_t,
        1018i32 as int16_t,
        1017i32 as int16_t,
        671i32 as int16_t,
        671i32 as int16_t,
        655i32 as int16_t,
        655i32 as int16_t,
        1016i32 as int16_t,
        1015i32 as int16_t,
        639i32 as int16_t,
        639i32 as int16_t,
        758i32 as int16_t,
        758i32 as int16_t,
        623i32 as int16_t,
        623i32 as int16_t,
        757i32 as int16_t,
        607i32 as int16_t,
        756i32 as int16_t,
        591i32 as int16_t,
        755i32 as int16_t,
        575i32 as int16_t,
        754i32 as int16_t,
        559i32 as int16_t,
        543i32 as int16_t,
        543i32 as int16_t,
        1009i32 as int16_t,
        783i32 as int16_t,
        -575i32 as int16_t,
        -621i32 as int16_t,
        -685i32 as int16_t,
        -749i32 as int16_t,
        496i32 as int16_t,
        -590i32 as int16_t,
        750i32 as int16_t,
        749i32 as int16_t,
        734i32 as int16_t,
        748i32 as int16_t,
        974i32 as int16_t,
        989i32 as int16_t,
        1003i32 as int16_t,
        958i32 as int16_t,
        988i32 as int16_t,
        973i32 as int16_t,
        1002i32 as int16_t,
        942i32 as int16_t,
        987i32 as int16_t,
        957i32 as int16_t,
        972i32 as int16_t,
        1001i32 as int16_t,
        926i32 as int16_t,
        986i32 as int16_t,
        941i32 as int16_t,
        971i32 as int16_t,
        956i32 as int16_t,
        1000i32 as int16_t,
        910i32 as int16_t,
        985i32 as int16_t,
        925i32 as int16_t,
        999i32 as int16_t,
        894i32 as int16_t,
        970i32 as int16_t,
        -1071i32 as int16_t,
        -1087i32 as int16_t,
        -1102i32 as int16_t,
        1390i32 as int16_t,
        -1135i32 as int16_t,
        1436i32 as int16_t,
        1509i32 as int16_t,
        1451i32 as int16_t,
        1374i32 as int16_t,
        -1151i32 as int16_t,
        1405i32 as int16_t,
        1358i32 as int16_t,
        1480i32 as int16_t,
        1420i32 as int16_t,
        -1167i32 as int16_t,
        1507i32 as int16_t,
        1494i32 as int16_t,
        1389i32 as int16_t,
        1342i32 as int16_t,
        1465i32 as int16_t,
        1435i32 as int16_t,
        1450i32 as int16_t,
        1326i32 as int16_t,
        1505i32 as int16_t,
        1310i32 as int16_t,
        1493i32 as int16_t,
        1373i32 as int16_t,
        1479i32 as int16_t,
        1404i32 as int16_t,
        1492i32 as int16_t,
        1464i32 as int16_t,
        1419i32 as int16_t,
        428i32 as int16_t,
        443i32 as int16_t,
        472i32 as int16_t,
        397i32 as int16_t,
        736i32 as int16_t,
        526i32 as int16_t,
        464i32 as int16_t,
        464i32 as int16_t,
        486i32 as int16_t,
        457i32 as int16_t,
        442i32 as int16_t,
        471i32 as int16_t,
        484i32 as int16_t,
        482i32 as int16_t,
        1357i32 as int16_t,
        1449i32 as int16_t,
        1434i32 as int16_t,
        1478i32 as int16_t,
        1388i32 as int16_t,
        1491i32 as int16_t,
        1341i32 as int16_t,
        1490i32 as int16_t,
        1325i32 as int16_t,
        1489i32 as int16_t,
        1463i32 as int16_t,
        1403i32 as int16_t,
        1309i32 as int16_t,
        1477i32 as int16_t,
        1372i32 as int16_t,
        1448i32 as int16_t,
        1418i32 as int16_t,
        1433i32 as int16_t,
        1476i32 as int16_t,
        1356i32 as int16_t,
        1462i32 as int16_t,
        1387i32 as int16_t,
        -1439i32 as int16_t,
        1475i32 as int16_t,
        1340i32 as int16_t,
        1447i32 as int16_t,
        1402i32 as int16_t,
        1474i32 as int16_t,
        1324i32 as int16_t,
        1461i32 as int16_t,
        1371i32 as int16_t,
        1473i32 as int16_t,
        269i32 as int16_t,
        448i32 as int16_t,
        1432i32 as int16_t,
        1417i32 as int16_t,
        1308i32 as int16_t,
        1460i32 as int16_t,
        -1711i32 as int16_t,
        1459i32 as int16_t,
        -1727i32 as int16_t,
        1441i32 as int16_t,
        1099i32 as int16_t,
        1099i32 as int16_t,
        1446i32 as int16_t,
        1386i32 as int16_t,
        1431i32 as int16_t,
        1401i32 as int16_t,
        -1743i32 as int16_t,
        1289i32 as int16_t,
        1083i32 as int16_t,
        1083i32 as int16_t,
        1160i32 as int16_t,
        1160i32 as int16_t,
        1458i32 as int16_t,
        1445i32 as int16_t,
        1067i32 as int16_t,
        1067i32 as int16_t,
        1370i32 as int16_t,
        1457i32 as int16_t,
        1307i32 as int16_t,
        1430i32 as int16_t,
        1129i32 as int16_t,
        1129i32 as int16_t,
        1098i32 as int16_t,
        1098i32 as int16_t,
        268i32 as int16_t,
        432i32 as int16_t,
        267i32 as int16_t,
        416i32 as int16_t,
        266i32 as int16_t,
        400i32 as int16_t,
        -1887i32 as int16_t,
        1144i32 as int16_t,
        1187i32 as int16_t,
        1082i32 as int16_t,
        1173i32 as int16_t,
        1113i32 as int16_t,
        1186i32 as int16_t,
        1066i32 as int16_t,
        1050i32 as int16_t,
        1158i32 as int16_t,
        1128i32 as int16_t,
        1143i32 as int16_t,
        1172i32 as int16_t,
        1097i32 as int16_t,
        1171i32 as int16_t,
        1081i32 as int16_t,
        420i32 as int16_t,
        391i32 as int16_t,
        1157i32 as int16_t,
        1112i32 as int16_t,
        1170i32 as int16_t,
        1142i32 as int16_t,
        1127i32 as int16_t,
        1065i32 as int16_t,
        1169i32 as int16_t,
        1049i32 as int16_t,
        1156i32 as int16_t,
        1096i32 as int16_t,
        1141i32 as int16_t,
        1111i32 as int16_t,
        1155i32 as int16_t,
        1080i32 as int16_t,
        1126i32 as int16_t,
        1154i32 as int16_t,
        1064i32 as int16_t,
        1153i32 as int16_t,
        1140i32 as int16_t,
        1095i32 as int16_t,
        1048i32 as int16_t,
        -2159i32 as int16_t,
        1125i32 as int16_t,
        1110i32 as int16_t,
        1137i32 as int16_t,
        -2175i32 as int16_t,
        823i32 as int16_t,
        823i32 as int16_t,
        1139i32 as int16_t,
        1138i32 as int16_t,
        807i32 as int16_t,
        807i32 as int16_t,
        384i32 as int16_t,
        264i32 as int16_t,
        368i32 as int16_t,
        263i32 as int16_t,
        868i32 as int16_t,
        838i32 as int16_t,
        853i32 as int16_t,
        791i32 as int16_t,
        867i32 as int16_t,
        822i32 as int16_t,
        852i32 as int16_t,
        837i32 as int16_t,
        866i32 as int16_t,
        806i32 as int16_t,
        865i32 as int16_t,
        790i32 as int16_t,
        -2319i32 as int16_t,
        851i32 as int16_t,
        821i32 as int16_t,
        836i32 as int16_t,
        352i32 as int16_t,
        262i32 as int16_t,
        850i32 as int16_t,
        805i32 as int16_t,
        849i32 as int16_t,
        -2399i32 as int16_t,
        533i32 as int16_t,
        533i32 as int16_t,
        835i32 as int16_t,
        820i32 as int16_t,
        336i32 as int16_t,
        261i32 as int16_t,
        578i32 as int16_t,
        548i32 as int16_t,
        563i32 as int16_t,
        577i32 as int16_t,
        532i32 as int16_t,
        532i32 as int16_t,
        832i32 as int16_t,
        772i32 as int16_t,
        562i32 as int16_t,
        562i32 as int16_t,
        547i32 as int16_t,
        547i32 as int16_t,
        305i32 as int16_t,
        275i32 as int16_t,
        560i32 as int16_t,
        515i32 as int16_t,
        290i32 as int16_t,
        290i32 as int16_t,
        288i32 as int16_t,
        258i32 as int16_t,
    ];
    static mut tab32: [uint8_t; 28] = [
        130i32 as uint8_t,
        162i32 as uint8_t,
        193i32 as uint8_t,
        209i32 as uint8_t,
        44i32 as uint8_t,
        28i32 as uint8_t,
        76i32 as uint8_t,
        140i32 as uint8_t,
        9i32 as uint8_t,
        9i32 as uint8_t,
        9i32 as uint8_t,
        9i32 as uint8_t,
        9i32 as uint8_t,
        9i32 as uint8_t,
        9i32 as uint8_t,
        9i32 as uint8_t,
        190i32 as uint8_t,
        254i32 as uint8_t,
        222i32 as uint8_t,
        238i32 as uint8_t,
        126i32 as uint8_t,
        94i32 as uint8_t,
        157i32 as uint8_t,
        157i32 as uint8_t,
        109i32 as uint8_t,
        61i32 as uint8_t,
        173i32 as uint8_t,
        205i32 as uint8_t,
    ];
    static mut tab33: [uint8_t; 16] = [
        252i32 as uint8_t,
        236i32 as uint8_t,
        220i32 as uint8_t,
        204i32 as uint8_t,
        188i32 as uint8_t,
        172i32 as uint8_t,
        156i32 as uint8_t,
        140i32 as uint8_t,
        124i32 as uint8_t,
        108i32 as uint8_t,
        92i32 as uint8_t,
        76i32 as uint8_t,
        60i32 as uint8_t,
        44i32 as uint8_t,
        28i32 as uint8_t,
        12i32 as uint8_t,
    ];
    static mut tabindex: [int16_t; 32] = [
        0i32 as int16_t,
        32i32 as int16_t,
        64i32 as int16_t,
        98i32 as int16_t,
        0i32 as int16_t,
        132i32 as int16_t,
        180i32 as int16_t,
        218i32 as int16_t,
        292i32 as int16_t,
        364i32 as int16_t,
        426i32 as int16_t,
        538i32 as int16_t,
        648i32 as int16_t,
        746i32 as int16_t,
        0i32 as int16_t,
        1126i32 as int16_t,
        1460i32 as int16_t,
        1460i32 as int16_t,
        1460i32 as int16_t,
        1460i32 as int16_t,
        1460i32 as int16_t,
        1460i32 as int16_t,
        1460i32 as int16_t,
        1460i32 as int16_t,
        1842i32 as int16_t,
        1842i32 as int16_t,
        1842i32 as int16_t,
        1842i32 as int16_t,
        1842i32 as int16_t,
        1842i32 as int16_t,
        1842i32 as int16_t,
        1842i32 as int16_t,
    ];
    static mut g_linbits: [uint8_t; 32] = [
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        0i32 as uint8_t,
        1i32 as uint8_t,
        2i32 as uint8_t,
        3i32 as uint8_t,
        4i32 as uint8_t,
        6i32 as uint8_t,
        8i32 as uint8_t,
        10i32 as uint8_t,
        13i32 as uint8_t,
        4i32 as uint8_t,
        5i32 as uint8_t,
        6i32 as uint8_t,
        7i32 as uint8_t,
        8i32 as uint8_t,
        9i32 as uint8_t,
        11i32 as uint8_t,
        13i32 as uint8_t,
    ];
    let mut one: libc::c_float = 0.0f32;
    let mut ireg: libc::c_int = 0i32;
    let mut big_val_cnt: libc::c_int = (*gr_info).big_values as libc::c_int;
    let mut sfb: *const uint8_t = (*gr_info).sfbtab;
    let mut bs_next_ptr: *const uint8_t = (*bs).buf.offset(((*bs).pos / 8i32) as isize);
    let mut bs_cache: uint32_t = (*bs_next_ptr.offset(0isize) as libc::c_uint)
        .wrapping_mul(256u32)
        .wrapping_add(*bs_next_ptr.offset(1isize) as libc::c_uint)
        .wrapping_mul(256u32)
        .wrapping_add(*bs_next_ptr.offset(2isize) as libc::c_uint)
        .wrapping_mul(256u32)
        .wrapping_add(*bs_next_ptr.offset(3isize) as libc::c_uint)
        << ((*bs).pos & 7i32);
    let mut pairs_to_decode: libc::c_int = 0;
    let mut np: libc::c_int = 0;
    let mut bs_sh: libc::c_int = ((*bs).pos & 7i32) - 8i32;
    bs_next_ptr = bs_next_ptr.offset(4isize);
    while big_val_cnt > 0i32 {
        let mut tab_num: libc::c_int = (*gr_info).table_select[ireg as usize] as libc::c_int;
        let fresh25 = ireg;
        ireg = ireg + 1;
        let mut sfb_cnt: libc::c_int = (*gr_info).region_count[fresh25 as usize] as libc::c_int;
        let mut codebook: *const int16_t = tabs
            .as_ptr()
            .offset(tabindex[tab_num as usize] as libc::c_int as isize);
        let mut linbits: libc::c_int = g_linbits[tab_num as usize] as libc::c_int;
        loop {
            let fresh26 = sfb;
            sfb = sfb.offset(1);
            np = *fresh26 as libc::c_int / 2i32;
            pairs_to_decode = if big_val_cnt > np { np } else { big_val_cnt };
            let fresh27 = scf;
            scf = scf.offset(1);
            one = *fresh27;
            loop {
                let mut j: libc::c_int = 0;
                let mut w: libc::c_int = 5i32;
                let mut leaf: libc::c_int =
                    *codebook.offset((bs_cache >> 32i32 - w) as isize) as libc::c_int;
                while leaf < 0i32 {
                    bs_cache <<= w;
                    bs_sh += w;
                    w = leaf & 7i32;
                    leaf = *codebook.offset(
                        (bs_cache >> 32i32 - w).wrapping_sub((leaf >> 3i32) as libc::c_uint)
                            as isize,
                    ) as libc::c_int
                }
                bs_cache <<= leaf >> 8i32;
                bs_sh += leaf >> 8i32;
                j = 0i32;
                while j < 2i32 {
                    let mut lsb: libc::c_int = leaf & 0xfi32;
                    if lsb == 15i32 && 0 != linbits {
                        lsb = (lsb as libc::c_uint).wrapping_add(bs_cache >> 32i32 - linbits)
                            as libc::c_int as libc::c_int;
                        bs_cache <<= linbits;
                        bs_sh += linbits;
                        while bs_sh >= 0i32 {
                            let fresh28 = bs_next_ptr;
                            bs_next_ptr = bs_next_ptr.offset(1);
                            bs_cache |= (*fresh28 as uint32_t) << bs_sh;
                            bs_sh -= 8i32
                        }
                        *dst = one
                            * L3_pow_43(lsb)
                            * (if (bs_cache as int32_t) < 0i32 {
                                -1i32
                            } else {
                                1i32
                            }) as libc::c_float
                    } else {
                        *dst = g_pow43[((16i32 + lsb) as libc::c_uint).wrapping_sub(
                            (16i32 as libc::c_uint).wrapping_mul(bs_cache >> 31i32),
                        ) as usize]
                            * one
                    }
                    bs_cache <<= if 0 != lsb { 1i32 } else { 0i32 };
                    bs_sh += if 0 != lsb { 1i32 } else { 0i32 };
                    j += 1;
                    dst = dst.offset(1isize);
                    leaf >>= 4i32
                }
                while bs_sh >= 0i32 {
                    let fresh29 = bs_next_ptr;
                    bs_next_ptr = bs_next_ptr.offset(1);
                    bs_cache |= (*fresh29 as uint32_t) << bs_sh;
                    bs_sh -= 8i32
                }
                pairs_to_decode -= 1;
                if !(0 != pairs_to_decode) {
                    break;
                }
            }
            big_val_cnt -= np;
            if !(big_val_cnt > 0i32 && {
                sfb_cnt -= 1;
                sfb_cnt >= 0i32
            }) {
                break;
            }
        }
    }
    np = 1i32 - big_val_cnt;
    loop {
        let mut codebook_count1: *const uint8_t = if 0 != (*gr_info).count1_table as libc::c_int {
            tab33.as_ptr()
        } else {
            tab32.as_ptr()
        };
        let mut leaf_0: libc::c_int =
            *codebook_count1.offset((bs_cache >> 32i32 - 4i32) as isize) as libc::c_int;
        if 0 == leaf_0 & 8i32 {
            leaf_0 = *codebook_count1.offset(
                ((leaf_0 >> 3i32) as libc::c_uint)
                    .wrapping_add(bs_cache << 4i32 >> 32i32 - (leaf_0 & 3i32))
                    as isize,
            ) as libc::c_int
        }
        bs_cache <<= leaf_0 & 7i32;
        bs_sh += leaf_0 & 7i32;
        if wrapping_offset_from(bs_next_ptr, (*bs).buf) as libc::c_long * 8i32 as libc::c_long
            - 24i32 as libc::c_long
            + bs_sh as libc::c_long
            > layer3gr_limit as libc::c_long
        {
            break;
        }
        np -= 1;
        if 0 == np {
            let fresh30 = sfb;
            sfb = sfb.offset(1);
            np = *fresh30 as libc::c_int / 2i32;
            if 0 == np {
                break;
            }
            let fresh31 = scf;
            scf = scf.offset(1);
            one = *fresh31
        }
        if 0 != leaf_0 & 128i32 >> 0i32 {
            *dst.offset(0isize) = if (bs_cache as int32_t) < 0i32 {
                -one
            } else {
                one
            };
            bs_cache <<= 1i32;
            bs_sh += 1i32
        }
        if 0 != leaf_0 & 128i32 >> 1i32 {
            *dst.offset(1isize) = if (bs_cache as int32_t) < 0i32 {
                -one
            } else {
                one
            };
            bs_cache <<= 1i32;
            bs_sh += 1i32
        }
        np -= 1;
        if 0 == np {
            let fresh32 = sfb;
            sfb = sfb.offset(1);
            np = *fresh32 as libc::c_int / 2i32;
            if 0 == np {
                break;
            }
            let fresh33 = scf;
            scf = scf.offset(1);
            one = *fresh33
        }
        if 0 != leaf_0 & 128i32 >> 2i32 {
            *dst.offset(2isize) = if (bs_cache as int32_t) < 0i32 {
                -one
            } else {
                one
            };
            bs_cache <<= 1i32;
            bs_sh += 1i32
        }
        if 0 != leaf_0 & 128i32 >> 3i32 {
            *dst.offset(3isize) = if (bs_cache as int32_t) < 0i32 {
                -one
            } else {
                one
            };
            bs_cache <<= 1i32;
            bs_sh += 1i32
        }
        while bs_sh >= 0i32 {
            let fresh34 = bs_next_ptr;
            bs_next_ptr = bs_next_ptr.offset(1);
            bs_cache |= (*fresh34 as uint32_t) << bs_sh;
            bs_sh -= 8i32
        }
        dst = dst.offset(4isize)
    }
    (*bs).pos = layer3gr_limit;
}
static mut g_pow43: [libc::c_float; 145] = [
    0i32 as libc::c_float,
    -1i32 as libc::c_float,
    -2.5198419094085695f32,
    -4.326748847961426f32,
    -6.34960412979126f32,
    -8.549880027770996f32,
    -10.902724266052246f32,
    -13.390518188476563f32,
    -16.0f32,
    -18.720754623413087f32,
    -21.544347763061525f32,
    -24.463781356811525f32,
    -27.473142623901368f32,
    -30.567350387573243f32,
    -33.74199295043945f32,
    -36.99317932128906f32,
    0i32 as libc::c_float,
    1i32 as libc::c_float,
    2.5198419094085695f32,
    4.326748847961426f32,
    6.34960412979126f32,
    8.549880027770996f32,
    10.902724266052246f32,
    13.390518188476563f32,
    16.0f32,
    18.720754623413087f32,
    21.544347763061525f32,
    24.463781356811525f32,
    27.473142623901368f32,
    30.567350387573243f32,
    33.74199295043945f32,
    36.99317932128906f32,
    40.317474365234378f32,
    43.711788177490237f32,
    47.173343658447269f32,
    50.69963073730469f32,
    54.288352966308597f32,
    57.937408447265628f32,
    61.64486312866211f32,
    65.40894317626953f32,
    69.22798156738281f32,
    73.1004409790039f32,
    77.02489471435547f32,
    81.0f32,
    85.02449035644531f32,
    89.0971908569336f32,
    93.21697235107422f32,
    97.38279724121094f32,
    101.59366607666016f32,
    105.8486328125f32,
    110.14680480957031f32,
    114.48731994628906f32,
    118.869384765625f32,
    123.29220581054688f32,
    127.75506591796875f32,
    132.2572479248047f32,
    136.79808044433595f32,
    141.3769073486328f32,
    145.9931182861328f32,
    150.6461181640625f32,
    155.3353271484375f32,
    160.06019592285157f32,
    164.82020568847657f32,
    169.6148223876953f32,
    174.44357299804688f32,
    179.3059844970703f32,
    184.20156860351563f32,
    189.12991333007813f32,
    194.090576171875f32,
    199.08314514160157f32,
    204.10720825195313f32,
    209.16238403320313f32,
    214.248291015625f32,
    219.36456298828126f32,
    224.51084899902345f32,
    229.68678283691407f32,
    234.89205932617188f32,
    240.12632751464845f32,
    245.3892822265625f32,
    250.68060302734376f32,
    256.0f32,
    261.34716796875f32,
    266.7218322753906f32,
    272.12371826171877f32,
    277.55255126953127f32,
    283.008056640625f32,
    288.4899597167969f32,
    293.998046875f32,
    299.5320739746094f32,
    305.0917663574219f32,
    310.6769104003906f32,
    316.2872619628906f32,
    321.9225769042969f32,
    327.58270263671877f32,
    333.2673645019531f32,
    338.97637939453127f32,
    344.7095642089844f32,
    350.4666442871094f32,
    356.2474670410156f32,
    362.0518798828125f32,
    367.8796081542969f32,
    373.73052978515627f32,
    379.60443115234377f32,
    385.5011291503906f32,
    391.4205017089844f32,
    397.3623046875f32,
    403.326416015625f32,
    409.31268310546877f32,
    415.3208923339844f32,
    421.35089111328127f32,
    427.402587890625f32,
    433.4757385253906f32,
    439.5702819824219f32,
    445.68597412109377f32,
    451.82275390625f32,
    457.9804382324219f32,
    464.15887451171877f32,
    470.35797119140627f32,
    476.5775451660156f32,
    482.81744384765627f32,
    489.0776062011719f32,
    495.3578796386719f32,
    501.6580810546875f32,
    507.9781494140625f32,
    514.3179321289063f32,
    520.6773071289063f32,
    527.0562133789063f32,
    533.4544067382813f32,
    539.8718872070313f32,
    546.3084716796875f32,
    552.7640380859375f32,
    559.2385864257813f32,
    565.7318725585938f32,
    572.243896484375f32,
    578.7744140625f32,
    585.323486328125f32,
    591.890869140625f32,
    598.4765625f32,
    605.0804443359375f32,
    611.7023315429688f32,
    618.3422241210938f32,
    625.0f32,
    631.675537109375f32,
    638.3687744140625f32,
    645.07958984375f32,
];
pub unsafe fn L3_pow_43(mut x: libc::c_int) -> libc::c_float {
    let mut frac: libc::c_float = 0.;
    let mut sign: libc::c_int = 0;
    let mut mult: libc::c_int = 256i32;
    if x < 129i32 {
        return g_pow43[(16i32 + x) as usize];
    } else {
        if x < 1024i32 {
            mult = 16i32;
            x <<= 3i32
        }
        sign = 2i32 * x & 64i32;
        frac = ((x & 63i32) - sign) as libc::c_float / ((x & !63i32) + sign) as libc::c_float;
        return g_pow43[(16i32 + (x + sign >> 6i32)) as usize]
            * (1.0f32
                + frac
                    * (4.0f32 / 3i32 as libc::c_float + frac * (2.0f32 / 9i32 as libc::c_float)))
            * mult as libc::c_float;
    };
}
pub unsafe fn L3_decode_scalefactors(
    mut hdr: *const uint8_t,
    mut ist_pos: *mut uint8_t,
    mut bs: *mut bs_t,
    mut gr: *const L3_gr_info_t,
    mut scf: *mut libc::c_float,
    mut ch: libc::c_int,
) {
    static mut g_scfc_decode: [uint8_t; 16] = [
        0i32 as uint8_t,
        1i32 as uint8_t,
        2i32 as uint8_t,
        3i32 as uint8_t,
        12i32 as uint8_t,
        5i32 as uint8_t,
        6i32 as uint8_t,
        7i32 as uint8_t,
        9i32 as uint8_t,
        10i32 as uint8_t,
        11i32 as uint8_t,
        13i32 as uint8_t,
        14i32 as uint8_t,
        15i32 as uint8_t,
        18i32 as uint8_t,
        19i32 as uint8_t,
    ];
    let mut part: libc::c_int = 0;
    static mut g_scf_partitions: [[uint8_t; 28]; 3] = [
        [
            6i32 as uint8_t,
            5i32 as uint8_t,
            5i32 as uint8_t,
            5i32 as uint8_t,
            6i32 as uint8_t,
            5i32 as uint8_t,
            5i32 as uint8_t,
            5i32 as uint8_t,
            6i32 as uint8_t,
            5i32 as uint8_t,
            7i32 as uint8_t,
            3i32 as uint8_t,
            11i32 as uint8_t,
            10i32 as uint8_t,
            0i32 as uint8_t,
            0i32 as uint8_t,
            7i32 as uint8_t,
            7i32 as uint8_t,
            7i32 as uint8_t,
            0i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            3i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            5i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            8i32 as uint8_t,
            9i32 as uint8_t,
            6i32 as uint8_t,
            12i32 as uint8_t,
            6i32 as uint8_t,
            9i32 as uint8_t,
            9i32 as uint8_t,
            9i32 as uint8_t,
            6i32 as uint8_t,
            9i32 as uint8_t,
            12i32 as uint8_t,
            6i32 as uint8_t,
            15i32 as uint8_t,
            18i32 as uint8_t,
            0i32 as uint8_t,
            0i32 as uint8_t,
            6i32 as uint8_t,
            15i32 as uint8_t,
            12i32 as uint8_t,
            0i32 as uint8_t,
            6i32 as uint8_t,
            12i32 as uint8_t,
            9i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            18i32 as uint8_t,
            9i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            9i32 as uint8_t,
            9i32 as uint8_t,
            6i32 as uint8_t,
            12i32 as uint8_t,
            9i32 as uint8_t,
            9i32 as uint8_t,
            9i32 as uint8_t,
            9i32 as uint8_t,
            9i32 as uint8_t,
            9i32 as uint8_t,
            12i32 as uint8_t,
            6i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            0i32 as uint8_t,
            0i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            0i32 as uint8_t,
            12i32 as uint8_t,
            9i32 as uint8_t,
            9i32 as uint8_t,
            6i32 as uint8_t,
            15i32 as uint8_t,
            12i32 as uint8_t,
            9i32 as uint8_t,
            0i32 as uint8_t,
        ],
    ];
    let mut scf_partition: *const uint8_t =
        g_scf_partitions[((0 != (*gr).n_short_sfb) as libc::c_int
                             + (0 == (*gr).n_long_sfb) as libc::c_int)
                             as usize]
            .as_ptr();
    let mut scf_size: [uint8_t; 4] = [0; 4];
    let mut iscf: [uint8_t; 40] = [0; 40];
    let mut i: libc::c_int = 0;
    let mut scf_shift: libc::c_int = (*gr).scalefac_scale as libc::c_int + 1i32;
    let mut gain_exp: libc::c_int = 0;
    let mut scfsi: libc::c_int = (*gr).scfsi as libc::c_int;
    let mut gain: libc::c_float = 0.;
    if 0 != *hdr.offset(1isize) as libc::c_int & 0x8i32 {
        part = g_scfc_decode[(*gr).scalefac_compress as usize] as libc::c_int;
        scf_size[0usize] = (part >> 2i32) as uint8_t;
        scf_size[1usize] = scf_size[0usize];
        scf_size[2usize] = (part & 3i32) as uint8_t;
        scf_size[3usize] = scf_size[2usize]
    } else {
        static mut g_mod: [uint8_t; 24] = [
            5i32 as uint8_t,
            5i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            5i32 as uint8_t,
            5i32 as uint8_t,
            4i32 as uint8_t,
            1i32 as uint8_t,
            4i32 as uint8_t,
            3i32 as uint8_t,
            1i32 as uint8_t,
            1i32 as uint8_t,
            5i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            1i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            1i32 as uint8_t,
            4i32 as uint8_t,
            3i32 as uint8_t,
            1i32 as uint8_t,
            1i32 as uint8_t,
        ];
        let mut k: libc::c_int = 0;
        let mut modprod: libc::c_int = 0;
        let mut sfc: libc::c_int = 0;
        let mut ist: libc::c_int =
            (0 != *hdr.offset(3isize) as libc::c_int & 0x10i32 && 0 != ch) as libc::c_int;
        sfc = (*gr).scalefac_compress as libc::c_int >> ist;
        k = ist * 3i32 * 4i32;
        while sfc >= 0i32 {
            modprod = 1i32;
            i = 3i32;
            while i >= 0i32 {
                scf_size[i as usize] =
                    (sfc / modprod % g_mod[(k + i) as usize] as libc::c_int) as uint8_t;
                modprod *= g_mod[(k + i) as usize] as libc::c_int;
                i -= 1
            }
            sfc -= modprod;
            k += 4i32
        }
        scf_partition = scf_partition.offset(k as isize);
        scfsi = -16i32
    }
    L3_read_scalefactors(
        iscf.as_mut_ptr(),
        ist_pos,
        scf_size.as_mut_ptr(),
        scf_partition,
        bs,
        scfsi,
    );
    if 0 != (*gr).n_short_sfb {
        let mut sh: libc::c_int = 3i32 - scf_shift;
        i = 0i32;
        while i < (*gr).n_short_sfb as libc::c_int {
            iscf[((*gr).n_long_sfb as libc::c_int + i + 0i32) as usize] =
                (iscf[((*gr).n_long_sfb as libc::c_int + i + 0i32) as usize] as libc::c_int
                    + (((*gr).subblock_gain[0usize] as libc::c_int) << sh))
                    as uint8_t;
            iscf[((*gr).n_long_sfb as libc::c_int + i + 1i32) as usize] =
                (iscf[((*gr).n_long_sfb as libc::c_int + i + 1i32) as usize] as libc::c_int
                    + (((*gr).subblock_gain[1usize] as libc::c_int) << sh))
                    as uint8_t;
            iscf[((*gr).n_long_sfb as libc::c_int + i + 2i32) as usize] =
                (iscf[((*gr).n_long_sfb as libc::c_int + i + 2i32) as usize] as libc::c_int
                    + (((*gr).subblock_gain[2usize] as libc::c_int) << sh))
                    as uint8_t;
            i += 3i32
        }
    } else if 0 != (*gr).preflag {
        static mut g_preamp: [uint8_t; 10] = [
            1i32 as uint8_t,
            1i32 as uint8_t,
            1i32 as uint8_t,
            1i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            3i32 as uint8_t,
            3i32 as uint8_t,
            3i32 as uint8_t,
            2i32 as uint8_t,
        ];
        i = 0i32;
        while i < 10i32 {
            iscf[(11i32 + i) as usize] = (iscf[(11i32 + i) as usize] as libc::c_int
                + g_preamp[i as usize] as libc::c_int)
                as uint8_t;
            i += 1
        }
    }
    gain_exp = (*gr).global_gain as libc::c_int + -1i32 * 4i32 - 210i32 - if *hdr.offset(3isize)
        as libc::c_int
        & 0xe0i32
        == 0x60i32
    {
        2i32
    } else {
        0i32
    };
    gain = L3_ldexp_q2(
        (1i32 << (255i32 + -1i32 * 4i32 - 210i32 + 3i32 & !3i32) / 4i32) as libc::c_float,
        (255i32 + -1i32 * 4i32 - 210i32 + 3i32 & !3i32) - gain_exp,
    );
    i = 0i32;
    while i < (*gr).n_long_sfb as libc::c_int + (*gr).n_short_sfb as libc::c_int {
        *scf.offset(i as isize) = L3_ldexp_q2(gain, (iscf[i as usize] as libc::c_int) << scf_shift);
        i += 1
    }
}
pub unsafe fn L3_read_scalefactors(
    mut scf: *mut uint8_t,
    mut ist_pos: *mut uint8_t,
    mut scf_size: *const uint8_t,
    mut scf_count: *const uint8_t,
    mut bitbuf: *mut bs_t,
    mut scfsi: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 0i32;
    while i < 4i32 && 0 != *scf_count.offset(i as isize) as libc::c_int {
        let mut cnt: libc::c_int = *scf_count.offset(i as isize) as libc::c_int;
        if 0 != scfsi & 8i32 {
            memcpy(
                scf as *mut libc::c_void,
                ist_pos as *const libc::c_void,
                cnt as libc::c_ulong,
            );
        } else {
            let mut bits: libc::c_int = *scf_size.offset(i as isize) as libc::c_int;
            if 0 == bits {
                memset(scf as *mut libc::c_void, 0i32, cnt as libc::c_ulong);
                memset(ist_pos as *mut libc::c_void, 0i32, cnt as libc::c_ulong);
            } else {
                let mut max_scf: libc::c_int = if scfsi < 0i32 {
                    (1i32 << bits) - 1i32
                } else {
                    -1i32
                };
                k = 0i32;
                while k < cnt {
                    let mut s: libc::c_int = get_bits(bitbuf, bits) as libc::c_int;
                    *ist_pos.offset(k as isize) = (if s == max_scf { -1i32 } else { s }) as uint8_t;
                    *scf.offset(k as isize) = s as uint8_t;
                    k += 1
                }
            }
        }
        ist_pos = ist_pos.offset(cnt as isize);
        scf = scf.offset(cnt as isize);
        i += 1;
        scfsi *= 2i32
    }
    let ref mut fresh36 = *scf.offset(1isize);
    let ref mut fresh35 = *scf.offset(2isize);
    *fresh35 = 0i32 as uint8_t;
    *fresh36 = *fresh35;
    *scf.offset(0isize) = *fresh36;
}
/* MINIMP3_ONLY_MP3 */
pub unsafe fn L3_read_side_info(
    mut bs: *mut bs_t,
    mut gr: *mut L3_gr_info_t,
    mut hdr: *const uint8_t,
) -> libc::c_int {
    static mut g_scf_long: [[uint8_t; 23]; 8] = [
        [
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            16i32 as uint8_t,
            20i32 as uint8_t,
            24i32 as uint8_t,
            28i32 as uint8_t,
            32i32 as uint8_t,
            38i32 as uint8_t,
            46i32 as uint8_t,
            52i32 as uint8_t,
            60i32 as uint8_t,
            68i32 as uint8_t,
            58i32 as uint8_t,
            54i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            16i32 as uint8_t,
            20i32 as uint8_t,
            24i32 as uint8_t,
            28i32 as uint8_t,
            32i32 as uint8_t,
            40i32 as uint8_t,
            48i32 as uint8_t,
            56i32 as uint8_t,
            64i32 as uint8_t,
            76i32 as uint8_t,
            90i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            16i32 as uint8_t,
            20i32 as uint8_t,
            24i32 as uint8_t,
            28i32 as uint8_t,
            32i32 as uint8_t,
            38i32 as uint8_t,
            46i32 as uint8_t,
            52i32 as uint8_t,
            60i32 as uint8_t,
            68i32 as uint8_t,
            58i32 as uint8_t,
            54i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            16i32 as uint8_t,
            18i32 as uint8_t,
            22i32 as uint8_t,
            26i32 as uint8_t,
            32i32 as uint8_t,
            38i32 as uint8_t,
            46i32 as uint8_t,
            54i32 as uint8_t,
            62i32 as uint8_t,
            70i32 as uint8_t,
            76i32 as uint8_t,
            36i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            16i32 as uint8_t,
            20i32 as uint8_t,
            24i32 as uint8_t,
            28i32 as uint8_t,
            32i32 as uint8_t,
            38i32 as uint8_t,
            46i32 as uint8_t,
            52i32 as uint8_t,
            60i32 as uint8_t,
            68i32 as uint8_t,
            58i32 as uint8_t,
            54i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            16i32 as uint8_t,
            20i32 as uint8_t,
            24i32 as uint8_t,
            28i32 as uint8_t,
            34i32 as uint8_t,
            42i32 as uint8_t,
            50i32 as uint8_t,
            54i32 as uint8_t,
            76i32 as uint8_t,
            158i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            16i32 as uint8_t,
            18i32 as uint8_t,
            22i32 as uint8_t,
            28i32 as uint8_t,
            34i32 as uint8_t,
            40i32 as uint8_t,
            46i32 as uint8_t,
            54i32 as uint8_t,
            54i32 as uint8_t,
            192i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            16i32 as uint8_t,
            20i32 as uint8_t,
            24i32 as uint8_t,
            30i32 as uint8_t,
            38i32 as uint8_t,
            46i32 as uint8_t,
            56i32 as uint8_t,
            68i32 as uint8_t,
            84i32 as uint8_t,
            102i32 as uint8_t,
            26i32 as uint8_t,
            0i32 as uint8_t,
        ],
    ];
    static mut g_scf_short: [[uint8_t; 40]; 8] = [
        [
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            30i32 as uint8_t,
            30i32 as uint8_t,
            30i32 as uint8_t,
            40i32 as uint8_t,
            40i32 as uint8_t,
            40i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            16i32 as uint8_t,
            16i32 as uint8_t,
            16i32 as uint8_t,
            20i32 as uint8_t,
            20i32 as uint8_t,
            20i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            28i32 as uint8_t,
            28i32 as uint8_t,
            28i32 as uint8_t,
            36i32 as uint8_t,
            36i32 as uint8_t,
            36i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            32i32 as uint8_t,
            32i32 as uint8_t,
            32i32 as uint8_t,
            42i32 as uint8_t,
            42i32 as uint8_t,
            42i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            32i32 as uint8_t,
            32i32 as uint8_t,
            32i32 as uint8_t,
            44i32 as uint8_t,
            44i32 as uint8_t,
            44i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            30i32 as uint8_t,
            30i32 as uint8_t,
            30i32 as uint8_t,
            40i32 as uint8_t,
            40i32 as uint8_t,
            40i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            22i32 as uint8_t,
            22i32 as uint8_t,
            22i32 as uint8_t,
            30i32 as uint8_t,
            30i32 as uint8_t,
            30i32 as uint8_t,
            56i32 as uint8_t,
            56i32 as uint8_t,
            56i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            16i32 as uint8_t,
            16i32 as uint8_t,
            16i32 as uint8_t,
            20i32 as uint8_t,
            20i32 as uint8_t,
            20i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            66i32 as uint8_t,
            66i32 as uint8_t,
            66i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            16i32 as uint8_t,
            16i32 as uint8_t,
            16i32 as uint8_t,
            20i32 as uint8_t,
            20i32 as uint8_t,
            20i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            34i32 as uint8_t,
            34i32 as uint8_t,
            34i32 as uint8_t,
            42i32 as uint8_t,
            42i32 as uint8_t,
            42i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            0i32 as uint8_t,
        ],
    ];
    static mut g_scf_mixed: [[uint8_t; 40]; 8] = [
        [
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            30i32 as uint8_t,
            30i32 as uint8_t,
            30i32 as uint8_t,
            40i32 as uint8_t,
            40i32 as uint8_t,
            40i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            0i32 as uint8_t,
            0,
            0,
            0,
        ],
        [
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            16i32 as uint8_t,
            16i32 as uint8_t,
            16i32 as uint8_t,
            20i32 as uint8_t,
            20i32 as uint8_t,
            20i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            28i32 as uint8_t,
            28i32 as uint8_t,
            28i32 as uint8_t,
            36i32 as uint8_t,
            36i32 as uint8_t,
            36i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            2i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            0i32 as uint8_t,
        ],
        [
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            32i32 as uint8_t,
            32i32 as uint8_t,
            32i32 as uint8_t,
            42i32 as uint8_t,
            42i32 as uint8_t,
            42i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            0i32 as uint8_t,
            0,
            0,
            0,
        ],
        [
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            32i32 as uint8_t,
            32i32 as uint8_t,
            32i32 as uint8_t,
            44i32 as uint8_t,
            44i32 as uint8_t,
            44i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            0i32 as uint8_t,
            0,
            0,
            0,
        ],
        [
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            24i32 as uint8_t,
            30i32 as uint8_t,
            30i32 as uint8_t,
            30i32 as uint8_t,
            40i32 as uint8_t,
            40i32 as uint8_t,
            40i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            0i32 as uint8_t,
            0,
            0,
            0,
        ],
        [
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            18i32 as uint8_t,
            22i32 as uint8_t,
            22i32 as uint8_t,
            22i32 as uint8_t,
            30i32 as uint8_t,
            30i32 as uint8_t,
            30i32 as uint8_t,
            56i32 as uint8_t,
            56i32 as uint8_t,
            56i32 as uint8_t,
            0i32 as uint8_t,
            0,
        ],
        [
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            10i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            14i32 as uint8_t,
            16i32 as uint8_t,
            16i32 as uint8_t,
            16i32 as uint8_t,
            20i32 as uint8_t,
            20i32 as uint8_t,
            20i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            66i32 as uint8_t,
            66i32 as uint8_t,
            66i32 as uint8_t,
            0i32 as uint8_t,
            0,
        ],
        [
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            4i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            6i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            8i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            16i32 as uint8_t,
            16i32 as uint8_t,
            16i32 as uint8_t,
            20i32 as uint8_t,
            20i32 as uint8_t,
            20i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            26i32 as uint8_t,
            34i32 as uint8_t,
            34i32 as uint8_t,
            34i32 as uint8_t,
            42i32 as uint8_t,
            42i32 as uint8_t,
            42i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            12i32 as uint8_t,
            0i32 as uint8_t,
            0,
        ],
    ];
    let mut tables: libc::c_uint = 0;
    let mut scfsi: libc::c_uint = 0i32 as libc::c_uint;
    let mut main_data_begin: libc::c_int = 0;
    let mut part_23_sum: libc::c_int = 0i32;
    let mut sr_idx: libc::c_int = (*hdr.offset(2isize) as libc::c_int >> 2i32 & 3i32)
        + ((*hdr.offset(1isize) as libc::c_int >> 3i32 & 1i32)
            + (*hdr.offset(1isize) as libc::c_int >> 4i32 & 1i32))
            * 3i32;
    sr_idx -= (sr_idx != 0i32) as libc::c_int;
    let mut gr_count: libc::c_int = if *hdr.offset(3isize) as libc::c_int & 0xc0i32 == 0xc0i32 {
        1i32
    } else {
        2i32
    };
    if 0 != *hdr.offset(1isize) as libc::c_int & 0x8i32 {
        gr_count *= 2i32;
        main_data_begin = get_bits(bs, 9i32) as libc::c_int;
        scfsi = get_bits(bs, 7i32 + gr_count)
    } else {
        main_data_begin = (get_bits(bs, 8i32 + gr_count) >> gr_count) as libc::c_int
    }
    loop {
        if *hdr.offset(3isize) as libc::c_int & 0xc0i32 == 0xc0i32 {
            scfsi <<= 4i32
        }
        (*gr).part_23_length = get_bits(bs, 12i32) as uint16_t;
        part_23_sum += (*gr).part_23_length as libc::c_int;
        (*gr).big_values = get_bits(bs, 9i32) as uint16_t;
        if (*gr).big_values as libc::c_int > 288i32 {
            return -1i32;
        } else {
            (*gr).global_gain = get_bits(bs, 8i32) as uint8_t;
            (*gr).scalefac_compress = get_bits(
                bs,
                if 0 != *hdr.offset(1isize) as libc::c_int & 0x8i32 {
                    4i32
                } else {
                    9i32
                },
            ) as uint16_t;
            (*gr).sfbtab = g_scf_long[sr_idx as usize].as_ptr();
            (*gr).n_long_sfb = 22i32 as uint8_t;
            (*gr).n_short_sfb = 0i32 as uint8_t;
            if 0 != get_bits(bs, 1i32) {
                (*gr).block_type = get_bits(bs, 2i32) as uint8_t;
                if 0 == (*gr).block_type {
                    return -1i32;
                } else {
                    (*gr).mixed_block_flag = get_bits(bs, 1i32) as uint8_t;
                    (*gr).region_count[0usize] = 7i32 as uint8_t;
                    (*gr).region_count[1usize] = 255i32 as uint8_t;
                    if (*gr).block_type as libc::c_int == 2i32 {
                        scfsi &= 0xf0fi32 as libc::c_uint;
                        if 0 == (*gr).mixed_block_flag {
                            (*gr).region_count[0usize] = 8i32 as uint8_t;
                            (*gr).sfbtab = g_scf_short[sr_idx as usize].as_ptr();
                            (*gr).n_long_sfb = 0i32 as uint8_t;
                            (*gr).n_short_sfb = 39i32 as uint8_t
                        } else {
                            (*gr).sfbtab = g_scf_mixed[sr_idx as usize].as_ptr();
                            (*gr).n_long_sfb = (if 0 != *hdr.offset(1isize) as libc::c_int & 0x8i32
                            {
                                8i32
                            } else {
                                6i32
                            }) as uint8_t;
                            (*gr).n_short_sfb = 30i32 as uint8_t
                        }
                    }
                    tables = get_bits(bs, 10i32);
                    tables <<= 5i32;
                    (*gr).subblock_gain[0usize] = get_bits(bs, 3i32) as uint8_t;
                    (*gr).subblock_gain[1usize] = get_bits(bs, 3i32) as uint8_t;
                    (*gr).subblock_gain[2usize] = get_bits(bs, 3i32) as uint8_t
                }
            } else {
                (*gr).block_type = 0i32 as uint8_t;
                (*gr).mixed_block_flag = 0i32 as uint8_t;
                tables = get_bits(bs, 15i32);
                (*gr).region_count[0usize] = get_bits(bs, 4i32) as uint8_t;
                (*gr).region_count[1usize] = get_bits(bs, 3i32) as uint8_t;
                (*gr).region_count[2usize] = 255i32 as uint8_t
            }
            (*gr).table_select[0usize] = (tables >> 10i32) as uint8_t;
            (*gr).table_select[1usize] = (tables >> 5i32 & 31i32 as libc::c_uint) as uint8_t;
            (*gr).table_select[2usize] = (tables & 31i32 as libc::c_uint) as uint8_t;
            (*gr).preflag = (if 0 != *hdr.offset(1isize) as libc::c_int & 0x8i32 {
                get_bits(bs, 1i32)
            } else {
                ((*gr).scalefac_compress as libc::c_int >= 500i32) as libc::c_int as libc::c_uint
            }) as uint8_t;
            (*gr).scalefac_scale = get_bits(bs, 1i32) as uint8_t;
            (*gr).count1_table = get_bits(bs, 1i32) as uint8_t;
            (*gr).scfsi = (scfsi >> 12i32 & 15i32 as libc::c_uint) as uint8_t;
            scfsi <<= 4i32;
            gr = gr.offset(1isize);
            gr_count -= 1;
            if !(0 != gr_count) {
                break;
            }
        }
    }
    if part_23_sum + (*bs).pos > (*bs).limit + main_data_begin * 8i32 {
        return -1i32;
    } else {
        return main_data_begin;
    };
}
pub unsafe fn L3_restore_reservoir(
    mut h: *mut mp3dec_t,
    mut bs: *mut bs_t,
    mut s: *mut mp3dec_scratch_t,
    mut main_data_begin: libc::c_int,
) -> libc::c_int {
    let mut frame_bytes: libc::c_int = ((*bs).limit - (*bs).pos) / 8i32;
    let mut bytes_have: libc::c_int = if (*h).reserv > main_data_begin {
        main_data_begin
    } else {
        (*h).reserv
    };
    memcpy(
        (*s).maindata.as_mut_ptr() as *mut libc::c_void,
        (*h).reserv_buf.as_mut_ptr().offset(
            (if 0i32 < (*h).reserv - main_data_begin {
                (*h).reserv - main_data_begin
            } else {
                0i32
            }) as isize,
        ) as *const libc::c_void,
        (if (*h).reserv > main_data_begin {
            main_data_begin
        } else {
            (*h).reserv
        }) as libc::c_ulong,
    );
    memcpy(
        (*s).maindata.as_mut_ptr().offset(bytes_have as isize) as *mut libc::c_void,
        (*bs).buf.offset(((*bs).pos / 8i32) as isize) as *const libc::c_void,
        frame_bytes as libc::c_ulong,
    );
    bs_init(
        &mut (*s).bs,
        (*s).maindata.as_mut_ptr(),
        bytes_have + frame_bytes,
    );
    return ((*h).reserv >= main_data_begin) as libc::c_int;
}
pub unsafe fn bs_init(
    mut bs: *mut bs_t,
    mut data: *const uint8_t,
    mut bytes: libc::c_int,
) {
    (*bs).buf = data;
    (*bs).pos = 0i32;
    (*bs).limit = bytes * 8i32;
}
pub unsafe fn hdr_sample_rate_hz(mut h: *const uint8_t) -> libc::c_uint {
    static mut g_hz: [libc::c_uint; 3] = [
        44100i32 as libc::c_uint,
        48000i32 as libc::c_uint,
        32000i32 as libc::c_uint,
    ];
    return g_hz[(*h.offset(2isize) as libc::c_int >> 2i32 & 3i32) as usize]
        >> (0 == *h.offset(1isize) as libc::c_int & 0x8i32) as libc::c_int
        >> (0 == *h.offset(1isize) as libc::c_int & 0x10i32) as libc::c_int;
}
pub unsafe fn mp3d_find_frame(
    mut mp3: *const uint8_t,
    mut mp3_bytes: libc::c_int,
    mut free_format_bytes: *mut libc::c_int,
    mut ptr_frame_bytes: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 0i32;
    while i < mp3_bytes - 4i32 {
        if 0 != hdr_valid(mp3) {
            let mut frame_bytes: libc::c_int = hdr_frame_bytes(mp3, *free_format_bytes);
            let mut frame_and_padding: libc::c_int = frame_bytes + hdr_padding(mp3);
            k = 4i32;
            while 0 == frame_bytes && k < 2304i32 && i + 2i32 * k < mp3_bytes - 4i32 {
                if 0 != hdr_compare(mp3, mp3.offset(k as isize)) {
                    let mut fb: libc::c_int = k - hdr_padding(mp3);
                    let mut nextfb: libc::c_int = fb + hdr_padding(mp3.offset(k as isize));
                    if !(i + k + nextfb + 4i32 > mp3_bytes
                        || 0 == hdr_compare(mp3, mp3.offset(k as isize).offset(nextfb as isize)))
                    {
                        frame_and_padding = k;
                        frame_bytes = fb;
                        *free_format_bytes = fb
                    }
                }
                k += 1
            }
            if 0 != frame_bytes
                && i + frame_and_padding <= mp3_bytes
                && 0 != mp3d_match_frame(mp3, mp3_bytes - i, frame_bytes)
                || 0 == i && frame_and_padding == mp3_bytes
            {
                *ptr_frame_bytes = frame_and_padding;
                return i;
            } else {
                *free_format_bytes = 0i32
            }
        }
        i += 1;
        mp3 = mp3.offset(1isize)
    }
    *ptr_frame_bytes = 0i32;
    return i;
}
pub unsafe fn hdr_padding(mut h: *const uint8_t) -> libc::c_int {
    return if 0 != *h.offset(2isize) as libc::c_int & 0x2i32 {
        if *h.offset(1isize) as libc::c_int & 6i32 == 6i32 {
            4i32
        } else {
            1i32
        }
    } else {
        0i32
    };
}
pub unsafe fn hdr_frame_bytes(
    mut h: *const uint8_t,
    mut free_format_size: libc::c_int,
) -> libc::c_int {
    let mut frame_bytes: libc::c_int = hdr_frame_samples(h)
        .wrapping_mul(hdr_bitrate_kbps(h))
        .wrapping_mul(125i32 as libc::c_uint)
        .wrapping_div(hdr_sample_rate_hz(h)) as libc::c_int;
    if *h.offset(1isize) as libc::c_int & 6i32 == 6i32 {
        /* slot align */
        frame_bytes &= !3i32
    }
    return if 0 != frame_bytes {
        frame_bytes
    } else {
        free_format_size
    };
}
pub unsafe fn mp3d_match_frame(
    mut hdr: *const uint8_t,
    mut mp3_bytes: libc::c_int,
    mut frame_bytes: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut nmatch: libc::c_int = 0;
    i = 0i32;
    nmatch = 0i32;
    while nmatch < 10i32 {
        i += hdr_frame_bytes(hdr.offset(i as isize), frame_bytes)
            + hdr_padding(hdr.offset(i as isize));
        if i + 4i32 > mp3_bytes {
            return (nmatch > 0i32) as libc::c_int;
        } else if 0 == hdr_compare(hdr, hdr.offset(i as isize)) {
            return 0i32;
        } else {
            nmatch += 1
        }
    }
    return 1i32;
}
pub unsafe fn hdr_compare(
    mut h1: *const uint8_t,
    mut h2: *const uint8_t,
) -> libc::c_int {
    return (0 != hdr_valid(h2)
        && (*h1.offset(1isize) as libc::c_int ^ *h2.offset(1isize) as libc::c_int) & 0xfei32
            == 0i32
        && (*h1.offset(2isize) as libc::c_int ^ *h2.offset(2isize) as libc::c_int) & 0xci32 == 0i32
        && 0 == (*h1.offset(2isize) as libc::c_int & 0xf0i32 == 0i32) as libc::c_int
            ^ (*h2.offset(2isize) as libc::c_int & 0xf0i32 == 0i32) as libc::c_int)
        as libc::c_int;
}
pub unsafe fn hdr_valid(mut h: *const uint8_t) -> libc::c_int {
    return (*h.offset(0isize) as libc::c_int == 0xffi32
        && (*h.offset(1isize) as libc::c_int & 0xf0i32 == 0xf0i32
            || *h.offset(1isize) as libc::c_int & 0xfei32 == 0xe2i32)
        && *h.offset(1isize) as libc::c_int >> 1i32 & 3i32 != 0i32
        && *h.offset(2isize) as libc::c_int >> 4i32 != 15i32
        && *h.offset(2isize) as libc::c_int >> 2i32 & 3i32 != 3i32) as libc::c_int;
}
