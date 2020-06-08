use crate::ffi::*;
use objc::runtime::*;

fn get_class(class: &str) -> *const Class {
    unsafe { ffi::objc_getClass(util::to_c_str("SBDockView")) }
}

fn log(data: &str) {
    unsafe { ffi::OBJC_NSLog(util::to_c_str(data)) }
}
