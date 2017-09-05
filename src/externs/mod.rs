use std::cell::RefCell;
use std::os::raw::{c_void, c_int};
use std::ptr::null_mut;
use std::ffi::CString;
use libc::*;

pub fn eval(x: &str) -> i32 {
    let x = CString::new(x).unwrap();
    let ptr = x.as_ptr();
    unsafe { emscripten_run_script_int(ptr) }
}

type EmCallbackFunc = unsafe extern "C" fn();

thread_local!(static RAF_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

extern "C" {
    // This extern is built in by Emscripten.
    pub fn emscripten_run_script_int(x: *const c_uchar) -> c_int;
    pub fn emscripten_set_main_loop(
        func: EmCallbackFunc,
        fps: c_int,
        simulate_infinite_loop: c_int,
    );
}

pub fn set_main_loop_callback<F>(callback: F)
where
    F: FnMut(),
{
    RAF_CALLBACK.with(|log| {
        *log.borrow_mut() = &callback as *const _ as *mut c_void;
    });
    unsafe {
        emscripten_set_main_loop(wrapper::<F>, 0, 0);
    }
}

unsafe extern "C" fn wrapper<F>()
where
    F: FnMut(),
{
    RAF_CALLBACK.with(|z| {
        let closure = *z.borrow_mut() as *mut F;
        (*closure)();
    });
}
