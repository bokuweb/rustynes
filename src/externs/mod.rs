use std::ffi::CString;
use std::cell::RefCell;
use std::ptr::null_mut;
use std::os::raw::{c_int, c_void, c_float};
use libc::*;

#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern "C" fn();
type em_str_callback_func = unsafe extern "C" fn(*const c_char);

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

pub fn eval(x: &str) -> i32 {
    let x = CString::new(x).unwrap();
    let ptr = x.as_ptr();
    unsafe { emscripten_run_script_int(ptr) }
}

pub fn wget(url: &str, file: &str, onload: em_callback_func) -> i32 {
    let url = CString::new(url).unwrap();
    let file = CString::new(file).unwrap();
    unsafe { emscripten_async_wget(url.as_ptr(), file.as_ptr(), onload, onload) };
    10
}


extern "C" {
    // This extern is built in by Emscripten.
    pub fn emscripten_run_script_int(x: *const c_uchar) -> c_int;
    pub fn emscripten_set_main_loop(func: em_callback_func,
                                    fps: c_int,
                                    simulate_infinite_loop: c_int);
    pub fn emscripten_async_wget(url: *const c_uchar,
                                 file: *const c_uchar,
                                 onload: em_callback_func,
                                 onerror: em_callback_func);
}


pub fn set_main_loop_callback<F>(callback: F)
    where F: FnMut()
{
    MAIN_LOOP_CALLBACK.with(|log| { *log.borrow_mut() = &callback as *const _ as *mut c_void; });

    unsafe {
        emscripten_set_main_loop(wrapper::<F>, 0, 1);
    }

    unsafe extern "C" fn wrapper<F>()
        where F: FnMut()
    {
        MAIN_LOOP_CALLBACK.with(|z| {
                                    let closure = *z.borrow_mut() as *mut F;
                                    (*closure)();
                                });
    }
}
