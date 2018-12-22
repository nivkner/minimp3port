#![allow(bad_style)]
#![allow(unused)]

extern crate minimp3port;

use minimp3port::*;

fn wrapping_offset_from<T>(this: *const T, origin: *const T) -> isize {
    let pointee_size = ::std::mem::size_of::<T>();
    assert!(0 < pointee_size && pointee_size <= isize::max_value() as usize);

    let d = isize::wrapping_sub(this as _, origin as _);
    d.wrapping_div(pointee_size as _)
}

extern crate libc;
#[cfg(target_os = "android")]
use libc::__errno as __errno_location;
#[cfg(not(target_os = "android"))]
use libc::__errno_location;
extern "C" {
    /* __cplusplus */
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, ...) -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn printf(_: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    fn rewind(__stream: *mut FILE);
    #[no_mangle]
    fn log10(_: libc::c_double) -> libc::c_double;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
/*
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp3dec_frame_info_t {
    pub frame_bytes: libc::c_int,
    pub channels: libc::c_int,
    pub hz: libc::c_int,
    pub layer: libc::c_int,
    pub bitrate_kbps: libc::c_int,
}
*/
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
/* __cplusplus */
/* MINIMP3_H */
/* more than ISO spec's */
/* MAX_FRAME_SYNC_MATCHES */
/* MUST be >= 320000/8/32000*1152 = 1440 */
/* !defined(MINIMP3_NO_SIMD) */
/* !defined(MINIMP3_NO_SIMD) */
/*
    https://github.com/lieff/minimp3
    To the extent possible under law, the author(s) have dedicated all copyright and related and neighboring rights to this software to the public domain worldwide.
    This software is distributed without any warranty.
    See <http://creativecommons.org/publicdomain/zero/1.0/>.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp3dec_file_info_t {
    pub buffer: *mut mp3d_sample_t,
    pub samples: size_t,
    pub channels: libc::c_int,
    pub hz: libc::c_int,
    pub layer: libc::c_int,
    pub avg_bitrate_kbps: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp3dec_map_info_t {
    pub buffer: *const uint8_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp3dec_ex_t {
    pub mp3d: mp3dec_t,
    pub file: mp3dec_map_info_t,
    pub seek_method: libc::c_int,
    pub is_file: libc::c_int,
}
pub type MP3D_ITERATE_CB = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *const uint8_t,
        _: libc::c_int,
        _: size_t,
        _: *mut mp3dec_frame_info_t,
    ) -> libc::c_int,
>;
pub type MP3D_PROGRESS_CB = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: size_t,
        _: size_t,
        _: *mut mp3dec_frame_info_t,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
/* decode whole buffer block */
#[no_mangle]
pub unsafe extern "C" fn mp3dec_load_buf(
    dec: *mut mp3dec_t,
    mut buf: *const uint8_t,
    mut buf_size: size_t,
    info: *mut mp3dec_file_info_t,
    progress_cb: MP3D_PROGRESS_CB,
    user_data: *mut libc::c_void,
) {
    let orig_buf_size: size_t = buf_size;
    let mut pcm: [mp3d_sample_t; 2304] = [0; 2304];
    let mut frame_info: mp3dec_frame_info_t = mp3dec_frame_info_t {
        frame_bytes: 0,
        channels: 0,
        hz: 0,
        layer: 0,
        bitrate_kbps: 0,
    };
    memset(
        info as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<mp3dec_file_info_t>() as libc::c_ulong,
    );
    memset(
        &mut frame_info as *mut mp3dec_frame_info_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<mp3dec_frame_info_t>() as libc::c_ulong,
    );
    /* skip id3v2 */
    let id3v2size: size_t = mp3dec_skip_id3v2(buf, buf_size);
    if id3v2size > buf_size {
        return;
    } else {
        buf = buf.offset(id3v2size as isize);
        buf_size = (buf_size as libc::c_ulong).wrapping_sub(id3v2size) as size_t as size_t;
        /* try to make allocation size assumption by first frame */
        mp3dec_init(dec);
        let mut samples: libc::c_int = 0;
        loop {
            samples = mp3dec_decode_frame(
                dec,
                buf,
                buf_size as libc::c_int,
                pcm.as_mut_ptr(),
                &mut frame_info,
            );
            buf = buf.offset(frame_info.frame_bytes as isize);
            buf_size = (buf_size as libc::c_ulong)
                .wrapping_sub(frame_info.frame_bytes as libc::c_ulong)
                as size_t as size_t;
            if 0 != samples {
                break;
            }
            if !(0 != frame_info.frame_bytes) {
                break;
            }
        }
        if 0 == samples {
            return;
        } else {
            samples *= frame_info.channels;
            let mut allocated: size_t = buf_size
                .wrapping_div(frame_info.frame_bytes as libc::c_ulong)
                .wrapping_mul(samples as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<mp3d_sample_t>() as libc::c_ulong)
                .wrapping_add(
                    ((1152i32 * 2i32) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<mp3d_sample_t>() as libc::c_ulong),
                );
            (*info).buffer = malloc(allocated) as *mut mp3d_sample_t;
            if (*info).buffer.is_null() {
                return;
            } else {
                (*info).samples = samples as size_t;
                memcpy(
                    (*info).buffer as *mut libc::c_void,
                    pcm.as_mut_ptr() as *const libc::c_void,
                    (*info)
                        .samples
                        .wrapping_mul(::std::mem::size_of::<mp3d_sample_t>() as libc::c_ulong),
                );
                /* save info */
                (*info).channels = frame_info.channels;
                (*info).hz = frame_info.hz;
                (*info).layer = frame_info.layer;
                let mut avg_bitrate_kbps: size_t = frame_info.bitrate_kbps as size_t;
                let mut frames: size_t = 1i32 as size_t;
                /* decode rest frames */
                let mut frame_bytes: libc::c_int = 0;
                loop {
                    if allocated.wrapping_sub(
                        (*info)
                            .samples
                            .wrapping_mul(::std::mem::size_of::<mp3d_sample_t>() as libc::c_ulong),
                    ) < ((1152i32 * 2i32) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<mp3d_sample_t>() as libc::c_ulong)
                    {
                        allocated = (allocated as libc::c_ulong).wrapping_mul(2i32 as libc::c_ulong)
                            as size_t as size_t;
                        (*info).buffer = realloc((*info).buffer as *mut libc::c_void, allocated)
                            as *mut mp3d_sample_t
                    }
                    samples = mp3dec_decode_frame(
                        dec,
                        buf,
                        buf_size as libc::c_int,
                        (*info).buffer.offset((*info).samples as isize),
                        &mut frame_info,
                    );
                    frame_bytes = frame_info.frame_bytes;
                    buf = buf.offset(frame_bytes as isize);
                    buf_size = (buf_size as libc::c_ulong)
                        .wrapping_sub(frame_bytes as libc::c_ulong)
                        as size_t as size_t;
                    if 0 != samples {
                        if (*info).hz != frame_info.hz || (*info).layer != frame_info.layer {
                            break;
                        }
                        if 0 != (*info).channels && (*info).channels != frame_info.channels {
                            /* mark file with mono-stereo transition */
                            (*info).channels = 0i32
                        }
                        (*info).samples = ((*info).samples as libc::c_ulong)
                            .wrapping_add((samples * frame_info.channels) as libc::c_ulong)
                            as size_t as size_t;
                        avg_bitrate_kbps = (avg_bitrate_kbps as libc::c_ulong)
                            .wrapping_add(frame_info.bitrate_kbps as libc::c_ulong)
                            as size_t as size_t;
                        frames = frames.wrapping_add(1);
                        if progress_cb.is_some() {
                            progress_cb.expect("non-null function pointer")(
                                user_data,
                                orig_buf_size,
                                orig_buf_size.wrapping_sub(buf_size),
                                &mut frame_info,
                            );
                        }
                    }
                    if !(0 != frame_bytes) {
                        break;
                    }
                }
                /* reallocate to normal buffer size */
                if allocated
                    != (*info)
                        .samples
                        .wrapping_mul(::std::mem::size_of::<mp3d_sample_t>() as libc::c_ulong)
                {
                    (*info).buffer = realloc(
                        (*info).buffer as *mut libc::c_void,
                        (*info)
                            .samples
                            .wrapping_mul(::std::mem::size_of::<mp3d_sample_t>() as libc::c_ulong),
                    ) as *mut mp3d_sample_t
                }
                (*info).avg_bitrate_kbps = avg_bitrate_kbps.wrapping_div(frames) as libc::c_int;
                return;
            }
        }
    };
}
/*MINIMP3_EXT_H*/
unsafe extern "C" fn mp3dec_skip_id3v2(buf: *const uint8_t, buf_size: size_t) -> size_t {
    if buf_size > 10i32 as libc::c_ulong
        && 0 == strncmp(
            buf as *mut libc::c_char,
            b"ID3\x00" as *const u8 as *const libc::c_char,
            3i32 as libc::c_ulong,
        )
    {
        return (((*buf.offset(6isize) as libc::c_int & 0x7fi32) << 21i32
            | (*buf.offset(7isize) as libc::c_int & 0x7fi32) << 14i32
            | (*buf.offset(8isize) as libc::c_int & 0x7fi32) << 7i32
            | *buf.offset(9isize) as libc::c_int & 0x7fi32)
            + 10i32) as size_t;
    } else {
        return 0i32 as size_t;
    };
}
/* iterate through frames with optional decoding */
#[no_mangle]
pub unsafe extern "C" fn mp3dec_iterate_buf(
    mut buf: *const uint8_t,
    mut buf_size: size_t,
    callback: MP3D_ITERATE_CB,
    user_data: *mut libc::c_void,
) {
    if callback.is_none() {
        return;
    } else {
        let mut frame_info: mp3dec_frame_info_t = mp3dec_frame_info_t {
            frame_bytes: 0,
            channels: 0,
            hz: 0,
            layer: 0,
            bitrate_kbps: 0,
        };
        memset(
            &mut frame_info as *mut mp3dec_frame_info_t as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<mp3dec_frame_info_t>() as libc::c_ulong,
        );
        /* skip id3v2 */
        let id3v2size: size_t = mp3dec_skip_id3v2(buf, buf_size);
        if id3v2size > buf_size {
            return;
        } else {
            let orig_buf: *const uint8_t = buf;
            buf = buf.offset(id3v2size as isize);
            buf_size = (buf_size as libc::c_ulong).wrapping_sub(id3v2size) as size_t as size_t;
            loop {
                let mut free_format_bytes: libc::c_int = 0i32;
                let mut frame_size: libc::c_int = 0i32;
                let i: libc::c_int = mp3d_find_frame(
                    buf,
                    buf_size as libc::c_int,
                    &mut free_format_bytes,
                    &mut frame_size,
                );
                buf = buf.offset(i as isize);
                buf_size = (buf_size as libc::c_ulong).wrapping_sub(i as libc::c_ulong) as size_t
                    as size_t;
                if 0 != i && 0 == frame_size {
                    continue;
                }
                if 0 == frame_size {
                    break;
                }
                let hdr: *const uint8_t = buf;
                frame_info.channels = if *hdr.offset(3isize) as libc::c_int & 0xc0i32 == 0xc0i32 {
                    1i32
                } else {
                    2i32
                };
                frame_info.hz = hdr_sample_rate_hz(hdr) as libc::c_int;
                frame_info.layer = 4i32 - (*hdr.offset(1isize) as libc::c_int >> 1i32 & 3i32);
                frame_info.bitrate_kbps = hdr_bitrate_kbps(hdr) as libc::c_int;
                frame_info.frame_bytes = frame_size;
                if 0 != callback.expect("non-null function pointer")(
                    user_data,
                    hdr,
                    frame_size,
                    wrapping_offset_from(hdr, orig_buf) as libc::c_long as size_t,
                    &mut frame_info,
                ) {
                    break;
                }
                buf = buf.offset(frame_size as isize);
                buf_size = (buf_size as libc::c_ulong).wrapping_sub(frame_size as libc::c_ulong)
                    as size_t as size_t
            }
            return;
        }
    };
}
/* decoder with seeking capability */
#[no_mangle]
pub unsafe extern "C" fn mp3dec_ex_open_buf(
    dec: *mut mp3dec_ex_t,
    buf: *const uint8_t,
    buf_size: size_t,
    seek_method: libc::c_int,
) -> libc::c_int {
    memset(
        dec as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<mp3dec_ex_t>() as libc::c_ulong,
    );
    (*dec).file.buffer = buf;
    (*dec).file.size = buf_size;
    (*dec).seek_method = seek_method;
    mp3dec_init(&mut (*dec).mp3d);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp3dec_ex_close(dec: *mut mp3dec_ex_t) {
    if 0 != (*dec).is_file {
        mp3dec_close_file(&mut (*dec).file);
    } else {
        free((*dec).file.buffer as *mut libc::c_void);
    }
    memset(
        dec as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<mp3dec_ex_t>() as libc::c_ulong,
    );
}
/*void mp3dec_ex_seek(mp3dec_ex_t *dec, size_t position)
{
}

int mp3dec_ex_read(mp3dec_ex_t *dec, int16_t *buf, int samples)
{
    return 0;
}*/
unsafe extern "C" fn mp3dec_close_file(map_info: *mut mp3dec_map_info_t) {
    if !(*map_info).buffer.is_null()
        && -1i32 as *mut libc::c_void != (*map_info).buffer as *mut libc::c_void
    {
        munmap((*map_info).buffer as *mut libc::c_void, (*map_info).size);
    }
    (*map_info).buffer = 0 as *const uint8_t;
    (*map_info).size = 0i32 as size_t;
}
/* stdio versions with file pre-load */
#[no_mangle]
pub unsafe extern "C" fn mp3dec_load(
    dec: *mut mp3dec_t,
    file_name: *const libc::c_char,
    info: *mut mp3dec_file_info_t,
    progress_cb: MP3D_PROGRESS_CB,
    user_data: *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut map_info: mp3dec_map_info_t = mp3dec_map_info_t {
        buffer: 0 as *const uint8_t,
        size: 0,
    };
    ret = mp3dec_open_file(file_name, &mut map_info);
    if 0 != ret {
        return ret;
    } else {
        mp3dec_load_buf(
            dec,
            map_info.buffer,
            map_info.size,
            info,
            progress_cb,
            user_data,
        );
        mp3dec_close_file(&mut map_info);
        return 0i32;
    };
}
unsafe extern "C" fn mp3dec_open_file(
    file_name: *const libc::c_char,
    map_info: *mut mp3dec_map_info_t,
) -> libc::c_int {
    let mut file: libc::c_int = 0;
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    memset(
        map_info as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<mp3dec_map_info_t>() as libc::c_ulong,
    );
    loop {
        file = open(file_name, 0i32);
        if !(file < 0i32 && (*__errno_location() == 11i32 || *__errno_location() == 4i32)) {
            break;
        }
    }
    if file < 0i32 || fstat(file, &mut st) < 0i32 {
        close(file);
        return -1i32;
    } else {
        (*map_info).size = st.st_size as size_t;
        loop {
            (*map_info).buffer = mmap(
                0 as *mut libc::c_void,
                st.st_size as size_t,
                0x1i32,
                0x2i32 | 0x8000i32,
                file,
                0i32 as __off_t,
            ) as *const uint8_t;
            if !(-1i32 as *mut libc::c_void == (*map_info).buffer as *mut libc::c_void
                && (*__errno_location() == 11i32 || *__errno_location() == 4i32))
            {
                break;
            }
        }
        close(file);
        if -1i32 as *mut libc::c_void == (*map_info).buffer as *mut libc::c_void {
            return -1i32;
        } else {
            return 0i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mp3dec_iterate(
    file_name: *const libc::c_char,
    callback: MP3D_ITERATE_CB,
    user_data: *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut map_info: mp3dec_map_info_t = mp3dec_map_info_t {
        buffer: 0 as *const uint8_t,
        size: 0,
    };
    ret = mp3dec_open_file(file_name, &mut map_info);
    if 0 != ret {
        return ret;
    } else {
        mp3dec_iterate_buf(map_info.buffer, map_info.size, callback, user_data);
        mp3dec_close_file(&mut map_info);
        return 0i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mp3dec_ex_open(
    dec: *mut mp3dec_ex_t,
    file_name: *const libc::c_char,
    seek_method: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    memset(
        dec as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<mp3dec_ex_t>() as libc::c_ulong,
    );
    ret = mp3dec_open_file(file_name, &mut (*dec).file);
    if 0 != ret {
        return ret;
    } else {
        (*dec).seek_method = seek_method;
        (*dec).is_file = 1i32;
        mp3dec_init(&mut (*dec).mp3d);
        return 0i32;
    };
}
/*#define MINIMP3_ONLY_MP3*/
/*#define MINIMP3_ONLY_SIMD*/
/*#define MINIMP3_NONSTANDARD_BUT_LOGICAL*/
unsafe extern "C" fn read16le(p: *const libc::c_void) -> int16_t {
    let src: *const uint8_t = p as *const uint8_t;
    return ((*src.offset(0isize) as libc::c_int) << 0i32
        | (*src.offset(1isize) as libc::c_int) << 8i32) as int16_t;
}
unsafe extern "C" fn wav_header(
    hz: libc::c_int,
    ch: libc::c_int,
    bips: libc::c_int,
    data_bytes: libc::c_int,
) -> *mut libc::c_char {
    static mut hdr: [libc::c_char; 44] = [
        82, 73, 70, 70, 115, 105, 122, 101, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0,
        99, 104, 95, 104, 122, 95, 97, 98, 112, 115, 98, 97, 98, 115, 100, 97, 116, 97, 115, 105,
        122, 101,
    ];
    let nAvgBytesPerSec: libc::c_ulong = (bips * ch * hz >> 3i32) as libc::c_ulong;
    let nBlockAlign: libc::c_uint = (bips * ch >> 3i32) as libc::c_uint;
    /* File size - 8 */
    *(hdr.as_mut_ptr().offset(0x4i32 as isize) as *mut libc::c_void as *mut int32_t) =
        44i32 + data_bytes - 8i32;
    /* Integer PCM format */
    *(hdr.as_mut_ptr().offset(0x14i32 as isize) as *mut libc::c_void as *mut int16_t) =
        1i32 as int16_t;
    *(hdr.as_mut_ptr().offset(0x16i32 as isize) as *mut libc::c_void as *mut int16_t) =
        ch as int16_t;
    *(hdr.as_mut_ptr().offset(0x18i32 as isize) as *mut libc::c_void as *mut int32_t) = hz;
    *(hdr.as_mut_ptr().offset(0x1ci32 as isize) as *mut libc::c_void as *mut int32_t) =
        nAvgBytesPerSec as int32_t;
    *(hdr.as_mut_ptr().offset(0x20i32 as isize) as *mut libc::c_void as *mut int16_t) =
        nBlockAlign as int16_t;
    *(hdr.as_mut_ptr().offset(0x22i32 as isize) as *mut libc::c_void as *mut int16_t) =
        bips as int16_t;
    *(hdr.as_mut_ptr().offset(0x28i32 as isize) as *mut libc::c_void as *mut int32_t) = data_bytes;
    return hdr.as_mut_ptr();
}
unsafe extern "C" fn preload(file: *mut FILE, data_size: *mut libc::c_int) -> *mut libc::c_uchar {
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    *data_size = 0i32;
    if file.is_null() {
        return 0 as *mut libc::c_uchar;
    } else if 0 != fseek(file, 0i32 as libc::c_long, 2i32) {
        return 0 as *mut libc::c_uchar;
    } else {
        *data_size = ftell(file) as libc::c_int;
        if *data_size < 0i32 {
            return 0 as *mut libc::c_uchar;
        } else if 0 != fseek(file, 0i32 as libc::c_long, 0i32) {
            return 0 as *mut libc::c_uchar;
        } else {
            data = malloc(*data_size as libc::c_ulong) as *mut libc::c_uchar;
            if data.is_null() {
                return 0 as *mut libc::c_uchar;
            } else if fread(
                data as *mut libc::c_void,
                1i32 as size_t,
                *data_size as size_t,
                file,
            ) as libc::c_int
                != *data_size
            {
                exit(1i32);
            } else {
                return data;
            }
        }
    };
}
unsafe extern "C" fn decode_file(
    input_file_name: *const libc::c_char,
    buf_ref: *const libc::c_uchar,
    ref_size: libc::c_int,
    file_out: *mut FILE,
    wave_out: libc::c_int,
) {
    let mut mp3d: mp3dec_t = mp3dec_t {
        mdct_overlap: [[0.; 288]; 2],
        qmf_state: [0.; 960],
        reserv: 0,
        free_format_bytes: 0,
        header: [0; 4],
        reserv_buf: [0; 511],
    };
    let mut i: libc::c_int = 0;
    let mut data_bytes: libc::c_int = 0;
    let mut total_samples: libc::c_int = 0i32;
    let mut maxdiff: libc::c_int = 0i32;
    let mut MSE: libc::c_double = 0.0f64;
    let mut psnr: libc::c_double = 0.;
    let mut info: mp3dec_file_info_t = mp3dec_file_info_t {
        buffer: 0 as *mut mp3d_sample_t,
        samples: 0,
        channels: 0,
        hz: 0,
        layer: 0,
        avg_bitrate_kbps: 0,
    };
    if 0 != mp3dec_load(
        &mut mp3d,
        input_file_name,
        &mut info,
        None,
        0 as *mut libc::c_void,
    ) {
        printf(b"error: file not found or read error\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    } else {
        let buffer: *mut int16_t = info.buffer;
        if 0 != wave_out && !file_out.is_null() {
            fwrite(
                wav_header(0i32, 0i32, 0i32, 0i32) as *const libc::c_void,
                1i32 as size_t,
                44i32 as size_t,
                file_out,
            );
        }
        if 0 != info.samples {
            total_samples = (total_samples as libc::c_ulong).wrapping_add(info.samples)
                as libc::c_int as libc::c_int;
            if !buf_ref.is_null() {
                let max_samples: libc::c_int =
                    (if (ref_size as size_t).wrapping_div(2i32 as libc::c_ulong) > info.samples {
                        info.samples
                    } else {
                        (ref_size as size_t).wrapping_div(2i32 as libc::c_ulong)
                    }) as libc::c_int;
                i = 0i32;
                while i < max_samples {
                    let MSEtemp: libc::c_int = abs(*buffer.offset(i as isize) as libc::c_int
                        - read16le(
                            &*buf_ref.offset(
                                (i as libc::c_ulong)
                                    .wrapping_mul(::std::mem::size_of::<int16_t>() as libc::c_ulong)
                                    as isize,
                            ) as *const libc::c_uchar
                                as *const libc::c_void,
                        ) as libc::c_int);
                    if MSEtemp > maxdiff {
                        maxdiff = MSEtemp
                    }
                    MSE += (MSEtemp as libc::c_float * MSEtemp as libc::c_float) as libc::c_double;
                    i += 1
                }
            }
            if !file_out.is_null() {
                fwrite(
                    buffer as *const libc::c_void,
                    info.samples,
                    ::std::mem::size_of::<int16_t>() as libc::c_ulong,
                    file_out,
                );
            }
            free(buffer as *mut libc::c_void);
        }
        MSE /= (if 0 != total_samples {
            total_samples
        } else {
            1i32
        }) as libc::c_double;
        if 0i32 as libc::c_double == MSE {
            psnr = 99.0f64
        } else {
            psnr = 10.0f64 * log10(0x7fffi32 as libc::c_double * 0x7fffi32 as libc::c_double / MSE)
        }
        printf(
            b"rate=%d samples=%d max_diff=%d PSNR=%f\n\x00" as *const u8 as *const libc::c_char,
            info.hz,
            total_samples,
            maxdiff,
            psnr,
        );
        if psnr < 96i32 as libc::c_double {
            printf(b"PSNR compliance failed\n\x00" as *const u8 as *const libc::c_char);
            exit(1i32);
        } else {
            if 0 != wave_out && !file_out.is_null() {
                data_bytes = (ftell(file_out) - 44i32 as libc::c_long) as libc::c_int;
                rewind(file_out);
                fwrite(
                    wav_header(info.hz, info.channels, 16i32, data_bytes) as *const libc::c_void,
                    1i32 as size_t,
                    44i32 as size_t,
                    file_out,
                );
            }
            return;
        }
    };
}
unsafe fn main_0(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut wave_out: libc::c_int = 0i32;
    let mut ref_size: libc::c_int = 0;
    let ref_file_name: *mut libc::c_char = if argc > 2i32 {
        *argv.offset(2isize)
    } else {
        0 as *mut libc::c_char
    };
    let output_file_name: *mut libc::c_char = if argc > 3i32 {
        *argv.offset(3isize)
    } else {
        0 as *mut libc::c_char
    };
    let mut file_out: *mut FILE = 0 as *mut FILE;
    if !output_file_name.is_null() {
        file_out = fopen(
            output_file_name,
            b"wb\x00" as *const u8 as *const libc::c_char,
        );
        let ext: *mut libc::c_char = strrchr(output_file_name, '.' as i32);
        if !ext.is_null()
            && 0 == strcasecmp(
                ext.offset(1isize),
                b"wav\x00" as *const u8 as *const libc::c_char,
            )
        {
            wave_out = 1i32
        }
    }
    let file_ref: *mut FILE = if !ref_file_name.is_null() {
        fopen(ref_file_name, b"rb\x00" as *const u8 as *const libc::c_char)
    } else {
        0 as *mut FILE
    };
    let buf_ref: *mut libc::c_uchar = preload(file_ref, &mut ref_size);
    if !file_ref.is_null() {
        fclose(file_ref);
    }
    let input_file_name: *mut libc::c_char = if argc > 1i32 {
        *argv.offset(1isize)
    } else {
        0 as *mut libc::c_char
    };
    if input_file_name.is_null() {
        printf(b"error: no file names given\n\x00" as *const u8 as *const libc::c_char);
        return 1i32;
    } else {
        decode_file(input_file_name, buf_ref, ref_size, file_out, wave_out);
        if !buf_ref.is_null() {
            free(buf_ref as *mut libc::c_void);
        }
        if !file_out.is_null() {
            fclose(file_out);
        }
        return 0i32;
    };
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
