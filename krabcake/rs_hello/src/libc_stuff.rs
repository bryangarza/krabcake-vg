//! The definitions in this file are a hack to work around issues when using
//! types like `CStr` in the staticlib we create here. Namely, libcore has
//! references to functions like `strlen` that are usually associated with
//! `libc`, as well as functions like `memcmp` (which can be rewritten into
//! `bcmp` by LLVM), which should be provided by *something*, but apparently are
//! not readily provided here.
//!
//! The easiest thing to do was to just call out to all the local definitions
//! for these methods that Valgrind itself provides.

use core::ffi::{c_char, c_int, c_size_t, c_void, CStr};

#[no_mangle]
pub unsafe extern "C" fn printf(s: *const c_char) -> usize {
    extern "C" {
        fn vgPlain_printf(s: *const c_char, ...) -> usize;
    }
    unsafe { vgPlain_printf(s) }
}

#[no_mangle]
unsafe extern "C" fn strlen(s: *const c_char) -> usize {
    extern "C" {
        fn vgPlain_strlen(s: *const c_char) -> usize;
    }
    unsafe { vgPlain_strlen(s) }
}

#[no_mangle]
unsafe extern "C" fn memcmp(s1: *const c_void, s2: *const c_void, n: c_size_t) -> usize {
    extern "C" {
        fn vgPlain_memcmp(s1: *const c_void, s2: *const c_void, n: c_size_t) -> usize;
    }
    unsafe { vgPlain_memcmp(s1, s2, n) }
}

#[no_mangle]
unsafe extern "C" fn bcmp(s1: *const c_void, s2: *const c_void, n: c_size_t) -> usize {
    unsafe { memcmp(s1, s2, n) }
}

#[repr(C)]
pub struct _Unwind_Exception {}

#[no_mangle]
unsafe extern "C-unwind" fn _Unwind_Resume(exception: *mut _Unwind_Exception) -> ! {
    loop {}
}
