use std::os::raw::{c_char, c_void};


#[inline(always)]
pub fn to_c_str(s: &str) -> *const c_char {
    let mut bytes = String::from(s).into_bytes();
    bytes.push(0);
    let ptr = bytes.as_ptr();
    std::mem::forget(bytes);
    unsafe { std::ffi::CStr::from_ptr(ptr as *const c_string::c_char).as_ptr() }
}

#[inline(always)]
pub fn to_nsstr(s: &str) -> *const c_void {
    unsafe { crate::ffi::OBJC_NSString(to_c_str(s)) }
}
