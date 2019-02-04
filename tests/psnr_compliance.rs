extern crate minimp3port;

use byteorder::ByteOrder;
use minimp3port::*;

macro_rules! test_all {
    (
        // Start a repetition:
        $(
            // each repeat has the name of te module and the name of the files without extentions
            $element:ident: name=$string:literal samples=$samples:literal rate=$rate:literal
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
                super::decode(MP3, PCM, $samples, $rate);
            }
        }
        )*
    };
}

test_all![
    ill2_center2: name="ILL2_center2" samples=2304 rate=48000,
    ill2_dual: name="ILL2_dual" samples=2304 rate=48000,
    ill2_dynx22: name="ILL2_dynx22" samples=2304 rate=48000,
    ill2_dynx31: name="ILL2_dynx31" samples=2304 rate=48000,
    ill2_dynx32: name="ILL2_dynx32" samples=2304 rate=48000,
    ill2_ext_switching: name="ILL2_ext_switching" samples=20736 rate=48000,
    ill2_layer1: name="ILL2_layer1" samples=2304 rate=44100,
    ill2_layer3: name="ILL2_layer3" samples=23040 rate=48000,
    ill2_mono: name="ILL2_mono" samples=1152 rate=48000,
    ill2_multilingual: name="ILL2_multilingual" samples=2304 rate=48000,
    ill2_overalloc1: name="ILL2_overalloc1" samples=2304 rate=48000,
    ill2_overalloc2: name="ILL2_overalloc2" samples=0 rate=0,
    ill2_prediction: name="ILL2_prediction" samples=2304 rate=48000,
    ill2_samples: name="ILL2_samples" samples=2304 rate=48000,
    ill2_scf63: name="ILL2_scf63" samples=2304 rate=48000,
    ill2_tca21: name="ILL2_tca21" samples=2304 rate=48000,
    ill2_tca30: name="ILL2_tca30" samples=2304 rate=48000,
    ill2_tca30_pc: name="ILL2_tca30_PC" samples=2304 rate=48000,
    ill2_tca31_pc: name="ILL2_tca31_PC" samples=2304 rate=48000,
    ill2_tca31_mtx0: name="ILL2_tca31_mtx0" samples=2304 rate=48000,
    ill2_tca31_mtx2: name="ILL2_tca31_mtx2" samples=2304 rate=48000,
    ill2_tca32_pc: name="ILL2_tca32_PC" samples=2304 rate=48000,
    ill2_wrongcrc: name="ILL2_wrongcrc" samples=2304 rate=48000,
    ill4_ext_id1: name="ILL4_ext_id1" samples=2304 rate=48000,
    ill4_sync: name="ILL4_sync" samples=2304 rate=48000,
    ill4_wrong_length1: name="ILL4_wrong_length1" samples=2304 rate=48000,
    ill4_wrong_length2: name="ILL4_wrong_length2" samples=2304 rate=48000,
    ill4_wrongcrc: name="ILL4_wrongcrc" samples=2304 rate=48000,
    m2l3_bitrate_16_all: name="M2L3_bitrate_16_all" samples=274176 rate=16000,
    m2l3_bitrate_22_all: name="M2L3_bitrate_22_all" samples=274176 rate=22050,
    m2l3_bitrate_24_all: name="M2L3_bitrate_24_all" samples=274176 rate=24000,
    m2l3_compl24: name="M2L3_compl24" samples=122112 rate=24000,
    m2l3_noise: name="M2L3_noise" samples=444672 rate=22050,
    l1_fl1: name="l1-fl1" samples=37632 rate=32000,
    l1_fl2: name="l1-fl2" samples=37632 rate=44100,
    l1_fl3: name="l1-fl3" samples=37632 rate=48000,
    l1_fl4: name="l1-fl4" samples=18816 rate=32000,
    l1_fl5: name="l1-fl5" samples=37632 rate=48000,
    l1_fl6: name="l1-fl6" samples=37632 rate=44100,
    l1_fl7: name="l1-fl7" samples=48384 rate=44100,
    l1_fl8: name="l1-fl8" samples=37632 rate=44100,
    l2_fl10: name="l2-fl10" samples=112896 rate=32000,
    l2_fl11: name="l2-fl11" samples=112896 rate=44100,
    l2_fl12: name="l2-fl12" samples=112896 rate=48000,
    l2_fl13: name="l2-fl13" samples=56448 rate=32000,
    l2_fl14: name="l2-fl14" samples=36864 rate=48000,
    l2_fl15: name="l2-fl15" samples=36864 rate=48000,
    l2_fl16: name="l2-fl16" samples=145152 rate=48000,
    l2_nonstandard_fl1_fl2_ff: name="l2-nonstandard-fl1_fl2_ff" samples=3072 rate=44100,
    l2_nonstandard_free_format: name="l2-nonstandard-free_format" samples=112896 rate=32000,
    l2_nonstandard_test32_size: name="l2-nonstandard-test32-size" samples=142848 rate=24000,
    l2_test32: name="l2-test32" samples=145152 rate=24000,
    l3_compl: name="l3-compl" samples=248832 rate=48000,
    l3_he_32khz: name="l3-he_32khz" samples=172800 rate=32000,
    l3_he_44khz: name="l3-he_44khz" samples=472320 rate=44100,
    l3_he_48khz: name="l3-he_48khz" samples=172800 rate=48000,
    l3_he_free: name="l3-he_free" samples=156672 rate=44100,
    l3_he_mode: name="l3-he_mode" samples=262656 rate=44100,
    l3_hecommon: name="l3-hecommon" samples=69120 rate=44100,
    l3_id3v2: name="l3-id3v2" samples=1152 rate=48000,
    l3_nonstandard_big_iscf: name="l3-nonstandard-big-iscf" samples=2304 rate=12000,
    l3_nonstandard_compl_sideinfo_bigvalues: name="l3-nonstandard-compl-sideinfo-bigvalues" samples=244224 rate=48000,
    l3_nonstandard_compl_sideinfo_blocktype: name="l3-nonstandard-compl-sideinfo-blocktype" samples=244224 rate=48000,
    l3_nonstandard_compl_sideinfo_size: name="l3-nonstandard-compl-sideinfo-size" samples=244224 rate=48000,
    l3_nonstandard_sideinfo_size: name="l3-nonstandard-sideinfo-size" samples=0 rate=0,
    l3_si: name="l3-si" samples=135936 rate=44100,
    l3_si_block: name="l3-si_block" samples=73728 rate=44100,
    l3_si_huff: name="l3-si_huff" samples=86400 rate=44100,
    l3_sin1k0db: name="l3-sin1k0db" samples=725760 rate=44100,
    l3_test45: name="l3-test45" samples=946944 rate=22050,
    l3_test46: name="l3-test46" samples=288000 rate=22050
];

struct FileInfo {
    samples: usize,
    channels: i32,
    hz: i32,
    layer: i32,
    avg_bitrate_kbps: i32,
}

// calculate the MSE between the decoded buffer and the reference buffer
fn compare_buffers(
    dec: &mut Decoder,
    buf: &[u8],
    info: &mut FileInfo,
    ref_buffer: &[i16],
) -> (f64, i32) {
    let mut pcm: [i16; 2304] = [0; 2304];
    let mut frame_info = FrameInfo {
        frame_bytes: 0,
        channels: 0,
        hz: 0,
        layer: 0,
        bitrate_kbps: 0,
    };
    let mut mse = 0.0;
    let mut maxdiff = 0;
    let id3v2size = skip_id3v2(buf);
    if id3v2size > buf.len() {
        return (mse, maxdiff);
    }
    let mut buf_slice = &buf[(id3v2size as usize)..];
    let mut samples: i32;
    loop {
        samples = decode_frame(dec, buf_slice, &mut pcm, &mut frame_info);
        buf_slice = &buf_slice[(frame_info.frame_bytes as usize)..];
        if 0 != samples || frame_info.frame_bytes == 0 {
            break;
        }
    }
    if 0 == samples {
        return (mse, maxdiff);
    }
    samples *= frame_info.channels;
    info.samples = samples as usize;
    // save info
    info.channels = frame_info.channels;
    info.hz = frame_info.hz;
    info.layer = frame_info.layer;
    let mut avg_bitrate_kbps: usize = frame_info.bitrate_kbps as usize;
    let mut frames: usize = 1i32 as usize;
    // decode rest frames
    let mut total = samples as usize;
    if !ref_buffer.is_empty() {
        let (m, diff) = get_mse(total, &pcm[..(samples as usize)], ref_buffer);
        mse += m;
        if diff > maxdiff {
            maxdiff = diff;
        }
    }
    loop {
        samples = decode_frame(dec, buf_slice, &mut pcm, &mut frame_info);
        let all_samples = samples as usize * frame_info.channels as usize;
        buf_slice = &buf_slice[(frame_info.frame_bytes as usize)..];

        let ref_slice = if total < ref_buffer.len() {
            &ref_buffer[total..]
        } else {
            &[]
        };

        if !ref_buffer.is_empty() {
            let (m, diff) = get_mse(all_samples, &pcm[..(all_samples)], ref_slice);
            mse += m;
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
                // mark file with mono-stereo transition
                info.channels = 0i32
            }
            info.samples += (samples * frame_info.channels) as usize;
            avg_bitrate_kbps += frame_info.bitrate_kbps as usize;
            frames += 1;
        }
        if 0 == frame_info.frame_bytes {
            break;
        }
    }
    info.avg_bitrate_kbps = (avg_bitrate_kbps / frames) as i32;
    (mse, maxdiff)
}

fn skip_id3v2(buf: &[u8]) -> usize {
    if buf.len() > 10 && &buf[..3] == b"ID3" {
        ((buf[6] as usize & 0x7f) << 21
            | (buf[7] as usize & 0x7f) << 14
            | (buf[8] as usize & 0x7f) << 7
            | buf[9] as usize & 0x7f)
            + 10
    } else {
        0
    }
}

fn get_mse(samples: usize, frame_buf: &[i16], buf_ref: &[i16]) -> (f64, i32) {
    let mut mse = 0.0;
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
            mse += f64::from(diff.pow(2));
        }
    }
    (mse, maxdiff)
}

fn decode(input_buffer: &[u8], buf: &[u8], expected_samples: usize, expected_sample_rate: usize) {
    let mut mp3d = Decoder {
        mdct_overlap: [[0.; 288]; 2],
        qmf_state: [0.; 960],
        reserv: 0,
        free_format_bytes: 0,
        header: [0; 4],
        reserv_buf: [0; 511],
    };
    let mut info = FileInfo {
        samples: 0,
        channels: 0,
        hz: 0,
        layer: 0,
        avg_bitrate_kbps: 0,
    };
    let mut vec_ref: Vec<i16> = Vec::with_capacity(buf.len() / 2);
    // this is safe because the capacity is the right size
    // the uninitialized data is immediately overwritten
    // and i16 isn't read when dropped
    unsafe { vec_ref.set_len(buf.len() / 2) };
    byteorder::LittleEndian::read_i16_into(buf, &mut vec_ref);
    let (mut mse, maxdiff) = compare_buffers(&mut mp3d, input_buffer, &mut info, &vec_ref);
    mse /= if 0 != info.samples {
        info.samples as f64
    } else {
        1.0
    };
    let psnr = if mse == 0.0 {
        99.0
    } else {
        10.0 * (0x7fffu32.pow(2) as f64 / mse).log10()
    };
    assert_eq!(info.hz as usize, expected_sample_rate, "sample rate");
    assert_eq!(info.samples, expected_samples, "number of samples");
    assert!(
        psnr > 96.0,
        "PSNR compliance failed: rate={} samples={} max_diff={} PSNR={}",
        info.hz,
        info.samples,
        maxdiff,
        psnr
    )
}
