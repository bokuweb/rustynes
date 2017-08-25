#[cfg(target_arch = "wasm32")]
extern crate emscripten_sys;

use std::ffi::CString;

pub fn fetch(url: &str) -> emscripten_sys::emscripten_fetch_t {
    let url = CString::new(url).unwrap();
    let ptr = url.as_ptr();
    unsafe { ffi::emscripten_fetch({}, ptr) }
}

pub unsafe extern "C" fn fetch(url: &str) -> emscripten_sys::emscripten_fetch_t {
    unsafe { emscripten_sys::emscripten_fetch() }
}
