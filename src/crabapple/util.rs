use objc::runtime::{Imp, Object};
use objc::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

use crate::objc::*;

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

#[inline(always)]
pub fn from_nsstr(s: &Object) -> String {
	let nschar: *mut std::os::raw::c_char = unsafe { msg_send![s, UTF8String] };
	let c_str: &CStr = unsafe { CStr::from_ptr(nschar) };
	match c_str.to_str() {
		Ok(e) => e.to_string(),
		Err(_) => "".to_string(),
	}
}

#[inline(always)]
pub fn strip_pac(ptr: *mut c_void) -> *mut c_void {
	unsafe { crate::ffi::ptr_strip(ptr) }
}

// TODO: make this shit work
#[inline(always)]
pub fn imp_to_func<R>(implementation: Imp) -> R {
	unsafe {
		let ptr: *mut c_void = std::mem::transmute(implementation);
		std::mem::transmute_copy(&strip_pac(ptr))
	}
}
