use std::ffi::CString;

pub fn eval(x: &str) -> i32 {
    let x = CString::new(x).unwrap();
    let ptr = x.as_ptr();
    unsafe { ffi::emscripten_run_script_int(ptr) }
}

pub fn set_main_loop(func: extern "C" fn()) {
    unsafe {
        ffi::emscripten_set_main_loop(func, 0, 0);
    }
}

mod ffi {
    use libc::*;

    extern "C" {
        // This extern is built in by Emscripten.
        pub fn emscripten_run_script_int(x: *const c_uchar) -> c_int;
        pub fn emscripten_set_main_loop(func: extern "C" fn(), fps: c_int, infinite: c_int);
    }
}
