pub mod ffi;
pub mod objc;
pub mod util;

use crate::objc::*;
use ::objc::runtime::*;
use ::objc::*;
use std::os::raw::{c_double, c_void};
use std::ptr::NonNull;

pub static mut ORIGIMP: Option<NonNull<Imp>> = None;

type SetBackgroundAlpha = unsafe extern "C" fn(this: &Object, cmd: Sel, alpha: f64);

#[no_mangle]
extern "C" fn my_set_background_alpha(this: &Object, cmd: Sel, alpha: c_double) {
	log(&format!(
		"ReachCCRust my_set_background_alpha: this = {:#?}, cmd = {:#?}, alpha = {}",
		this, cmd, alpha
	));
	unsafe {
		if let Some(orig) = ORIGIMP {
			log(&format!("ReachCCRust my_set_background_alpha = {:?}", orig));
			let ptr: *mut c_void = std::mem::transmute(orig);
			let nopac = ffi::ptr_strip(ptr);
			let x: SetBackgroundAlpha = std::mem::transmute(nopac);
			x(this, cmd, 0.0);
		}
	}
}

#[used]
#[cfg_attr(target_os = "ios", link_section = "__DATA,__mod_init_func")]
static LOAD: extern "C" fn() = {
	extern "C" fn ctor() {
		let sba_sel = sel!(setBackgroundAlpha:);
		let swizz_imp: *mut c_void =
			my_set_background_alpha as SetBackgroundAlpha as usize as *mut c_void;
		let sb_dock_view = get_class("SBDockView");
		log(&format!(
			"ReachCCRust hooking: swizz_imp = {:#?}, sb_dock_view = {:#?}, sba_sel = {:#?}",
			swizz_imp, sb_dock_view, sba_sel
		));
		unsafe {
			ffi::MSHookMessageEx(sb_dock_view, sba_sel, swizz_imp, &mut ORIGIMP);
		}
	}
	ctor
};
