#![allow(bad_style)]
#![allow(clippy::all)]

extern crate libc;
extern crate minimp3port;

use byteorder::ByteOrder;
use minimp3port::*;

macro_rules! test_all {
    (
        // Start a repetition:
        $(
            // each repeat has the name of te module and the name of the files without extentions
            $element:ident: $string:expr
        )
        // ...separated by commas...
        ,
        // ...zero or more times.
        *
    ) => {
        $(
        mod $element {
            static MP3: &[u8] = include_bytes!(concat!("../vectors/", $string, ".bit"));
            static PCM: &[u8] = include_bytes!(concat!("../vectors/", $string, ".pcm"));

            #[test]
            fn test() {
                super::decode(MP3, PCM);
            }
        }
        )*
    };
}

test_all![
    ILL2_center2: "ILL2_center2",
    ILL2_dual: "ILL2_dual",
    ILL2_dynx22: "ILL2_dynx22",
    ILL2_dynx31: "ILL2_dynx31",
    ILL2_dynx32: "ILL2_dynx32",
    ILL2_ext_switching: "ILL2_ext_switching",
    ILL2_layer1: "ILL2_layer1",
    ILL2_layer3: "ILL2_layer3",
    ILL2_mono: "ILL2_mono",
    ILL2_multilingual: "ILL2_multilingual",
    ILL2_overalloc1: "ILL2_overalloc1",
    ILL2_overalloc2: "ILL2_overalloc2",
    ILL2_prediction: "ILL2_prediction",
    ILL2_samples: "ILL2_samples",
    ILL2_scf63: "ILL2_scf63",
    ILL2_tca21: "ILL2_tca21",
    ILL2_tca30: "ILL2_tca30",
    ILL2_tca30_PC: "ILL2_tca30_PC",
    ILL2_tca31_PC: "ILL2_tca31_PC",
    ILL2_tca31_mtx0: "ILL2_tca31_mtx0",
    ILL2_tca31_mtx2: "ILL2_tca31_mtx2",
    ILL2_tca32_PC: "ILL2_tca32_PC",
    ILL2_wrongcrc: "ILL2_wrongcrc",
    ILL4_ext_id1: "ILL4_ext_id1",
    ILL4_sync: "ILL4_sync",
    ILL4_wrong_length1: "ILL4_wrong_length1",
    ILL4_wrong_length2: "ILL4_wrong_length2",
    ILL4_wrongcrc: "ILL4_wrongcrc",
    M2L3_bitrate_16_all: "M2L3_bitrate_16_all",
    M2L3_bitrate_22_all: "M2L3_bitrate_22_all",
    M2L3_bitrate_24_all: "M2L3_bitrate_24_all",
    M2L3_compl24: "M2L3_compl24",
    M2L3_noise: "M2L3_noise",
    l1_fl1: "l1-fl1",
    l1_fl2: "l1-fl2",
    l1_fl3: "l1-fl3",
    l1_fl4: "l1-fl4",
    l1_fl5: "l1-fl5",
    l1_fl6: "l1-fl6",
    l1_fl7: "l1-fl7",
    l1_fl8: "l1-fl8",
    l2_fl10: "l2-fl10",
    l2_fl11: "l2-fl11",
    l2_fl12: "l2-fl12",
    l2_fl13: "l2-fl13",
    l2_fl14: "l2-fl14",
    l2_fl15: "l2-fl15",
    l2_fl16: "l2-fl16",
    l2_nonstandard_fl1_fl2_ff: "l2-nonstandard-fl1_fl2_ff",
    l2_nonstandard_free_format: "l2-nonstandard-free_format",
    l2_nonstandard_test32_size: "l2-nonstandard-test32-size",
    l2_test32: "l2-test32",
    l3_compl: "l3-compl",
    l3_he_32khz: "l3-he_32khz",
    l3_he_44khz: "l3-he_44khz",
    l3_he_48khz: "l3-he_48khz",
    l3_he_free: "l3-he_free",
    l3_he_mode: "l3-he_mode",
    l3_hecommon: "l3-hecommon",
    l3_id3v2: "l3-id3v2",
    l3_nonstandard_big_iscf: "l3-nonstandard-big-iscf",
    l3_nonstandard_compl_sideinfo_bigvalues: "l3-nonstandard-compl-sideinfo-bigvalues",
    l3_nonstandard_compl_sideinfo_blocktype: "l3-nonstandard-compl-sideinfo-blocktype",
    l3_nonstandard_compl_sideinfo_size: "l3-nonstandard-compl-sideinfo-size",
    l3_nonstandard_sideinfo_size: "l3-nonstandard-sideinfo-size",
    l3_si: "l3-si",
    l3_si_block: "l3-si_block",
    l3_si_huff: "l3-si_huff",
    l3_sin1k0db: "l3-sin1k0db",
    l3_test45: "l3-test45",
    l3_test46: "l3-test46"
];

struct mp3dec_file_info_t {
    samples: libc::size_t,
    channels: libc::c_int,
    hz: libc::c_int,
    layer: libc::c_int,
    avg_bitrate_kbps: libc::c_int,
}

/* decode whole buffer block */
fn load_buffer(
    dec: &mut mp3dec_t,
    buf: &[u8],
    info: &mut mp3dec_file_info_t,
    ref_buffer: &[i16],
) -> (f64, i32) {
    let mut pcm: [mp3d_sample_t; 2304] = [0; 2304];
    let mut frame_info: mp3dec_frame_info_t = mp3dec_frame_info_t {
        frame_bytes: 0,
        channels: 0,
        hz: 0,
        layer: 0,
        bitrate_kbps: 0,
    };
    let mut MSE = 0.0;
    let mut maxdiff = 0;
    /* skip id3v2 */
    let id3v2size = mp3dec_skip_id3v2(buf);
    if id3v2size > buf.len() {
        return (MSE, maxdiff);
    }
    let mut buf_slice = &buf[(id3v2size as usize)..];
    unsafe { mp3dec_init(dec) };
    let mut samples: libc::c_int;
    loop {
        samples = decode_frame(dec, buf_slice, Some(&mut pcm), &mut frame_info);
        buf_slice = &buf_slice[(frame_info.frame_bytes as usize)..];
        if 0 != samples || frame_info.frame_bytes == 0 {
            break;
        }
    }
    if 0 == samples {
        return (MSE, maxdiff);
    }
    samples *= frame_info.channels;
    info.samples = samples as libc::size_t;
    /* save info */
    info.channels = frame_info.channels;
    info.hz = frame_info.hz;
    info.layer = frame_info.layer;
    let mut avg_bitrate_kbps: libc::size_t = frame_info.bitrate_kbps as libc::size_t;
    let mut frames: libc::size_t = 1i32 as libc::size_t;
    /* decode rest frames */
    let mut total = samples as usize;
    if !ref_buffer.is_empty() {
        let (m, diff) = mse(total, &pcm[..(samples as usize)], ref_buffer);
        MSE += m;
        if diff > maxdiff {
            maxdiff = diff;
        }
    }
    loop {
        samples = decode_frame(dec, buf_slice, Some(&mut pcm), &mut frame_info);
        let all_samples = samples as usize * frame_info.channels as usize;
        buf_slice = &buf_slice[(frame_info.frame_bytes as usize)..];

        let ref_slice = if total < ref_buffer.len() {
            &ref_buffer[total..]
        } else {
            &[]
        };

        if !ref_buffer.is_empty() {
            let (m, diff) = mse(all_samples, &pcm[..(all_samples)], ref_slice);
            MSE += m;
            if diff > maxdiff {
                maxdiff = diff;
            }
        }
        total += all_samples;
        if 0 != samples {
            if info.hz != frame_info.hz || info.layer != frame_info.layer {
                break;
            }
            if 0 != info.channels && info.channels != frame_info.channels {
                /* mark file with mono-stereo transition */
                info.channels = 0i32
            }
            info.samples += (samples * frame_info.channels) as libc::size_t;
            avg_bitrate_kbps += frame_info.bitrate_kbps as libc::size_t;
            frames += 1;
        }
        if 0 == frame_info.frame_bytes {
            break;
        }
    }
    info.avg_bitrate_kbps = avg_bitrate_kbps.wrapping_div(frames) as libc::c_int;
    (MSE, maxdiff)
}

fn mp3dec_skip_id3v2(buf: &[u8]) -> usize {
    if buf.len() > 10 && &buf[..3] == b"ID3" {
        return ((buf[6] as usize & 0x7f) << 21
            | (buf[7] as usize & 0x7f) << 14
            | (buf[8] as usize & 0x7f) << 7
            | buf[9] as usize & 0x7f)
            + 10;
    } else {
        return 0;
    };
}

fn mse(samples: usize, frame_buf: &[i16], buf_ref: &[i16]) -> (f64, i32) {
    let mut MSE = 0.0;
    let mut maxdiff = 0;
    if 0 != samples {
        let max_samples = std::cmp::min(buf_ref.len() / 2, samples as usize);
        for i in 0..max_samples {
            let ref_res = buf_ref[i];
            let info_res = frame_buf[i as usize];
            let diff = (ref_res - info_res).abs();
            if diff as i32 > maxdiff {
                maxdiff = diff.into()
            }
            MSE += f64::from(diff.pow(2));
        }
    }
    (MSE, maxdiff)
}

fn decode(input_buffer: &[u8], buf: &[u8]) {
    let mut mp3d: mp3dec_t = mp3dec_t {
        mdct_overlap: [[0.; 288]; 2],
        qmf_state: [0.; 960],
        reserv: 0,
        free_format_bytes: 0,
        header: [0; 4],
        reserv_buf: [0; 511],
    };
    let mut info: mp3dec_file_info_t = mp3dec_file_info_t {
        samples: 0,
        channels: 0,
        hz: 0,
        layer: 0,
        avg_bitrate_kbps: 0,
    };
    let mut vec_ref: Vec<i16> = Vec::with_capacity(buf.len() / 2);
    unsafe { vec_ref.set_len(buf.len() / 2) };
    byteorder::LittleEndian::read_i16_into(buf, &mut vec_ref);
    let (mut MSE, maxdiff) = load_buffer(&mut mp3d, input_buffer, &mut info, &vec_ref);
    MSE /= if 0 != info.samples {
        info.samples as f64
    } else {
        1.0
    };
    let psnr = if MSE == 0.0 {
        99.0
    } else {
        10.0 * (0x7fffu32.pow(2) as f64 / MSE).log10()
    };
    assert!(
        psnr > 96.0,
        "PSNR compliance failed: rate={} samples={} max_diff={} PSNR={}",
        info.hz,
        info.samples,
        maxdiff,
        psnr
    )
}
