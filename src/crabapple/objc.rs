use crate::ffi::*;
use crate::util::*;
use ::objc::runtime::*;
use objc_foundation::{INSString, NSString};
use std::fmt;
use std::os::raw::c_void;
use std::ptr::NonNull;

pub fn get_class(class: &str) -> *const Class {
	unsafe { objc_getClass(to_c_str(class)) }
}

pub fn log(data: &str) {
	unsafe { NSLogv(&*NSString::from_str(data)) }
}

pub fn hook(class: &str, selector: Sel, replacement: *mut c_void, orig: &mut Option<NonNull<Imp>>) {
	unsafe {
		MSHookMessageEx(get_class(class), selector, replacement, orig);
	}
}
