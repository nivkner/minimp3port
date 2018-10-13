mod ffi;

#[no_mangle]
pub extern "C" fn mp3dec_init(dec: *mut ffi::mp3dec_t) {
    unsafe { (*dec).header[0] = 0 }
}

#[no_mangle]
pub extern "C" fn mp3dec_decode_frame(
    dec: &mut ffi::mp3dec_t,
    mp3: *const u8,
    mp3_bytes: ::std::os::raw::c_int,
    pcm: &mut ffi::mp3d_sample_t,
    info: &mut ffi::mp3dec_frame_info_t,
) -> ::std::os::raw::c_int {
    unsafe { ffi::__mp3dec_decode_frame(dec, mp3, mp3_bytes, pcm, info) }
}
