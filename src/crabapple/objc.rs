use crate::ffi::*;
use crate::util::*;
use ::objc::runtime::*;

pub fn get_class(class: &str) -> *const Class {
	unsafe { objc_getClass(to_c_str(class)) }
}

pub fn log(data: &str) {
	unsafe { OBJC_NSLog(to_c_str(data)) }
}
