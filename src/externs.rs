extern crate libc;

pub fn set_main_loop(func: extern "C" fn()) {
    unsafe {
        emscripten_set_main_loop(func, 60, 0);
    }
}

extern "C" {
    pub fn emscripten_set_main_loop(func: extern "C" fn(), fps: libc::c_int, infinite: libc::c_int);
}
