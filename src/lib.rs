use objc::runtime::*;
use objc::*;
use std::os::raw::{c_char, c_double, c_void};
use std::ptr::NonNull;

pub static mut ORIGIMP: Option<NonNull<Imp>> = None;

extern "C" {
    fn OBJC_NSString(str: *const c_char) -> *mut c_void;
    fn OBJC_NSLog(str: *const c_char);
    fn NSLogv(nsFormat: *mut c_void); // format from inside rust or it dies
    fn MSHookMessageEx(class: *const Class, selector: Sel, replacement: *mut c_void, result: &mut Option<NonNull<Imp>>);
}

#[inline(always)]
fn to_c_str(s: &str) -> *const c_char {
    let mut bytes = String::from(s).into_bytes();
    bytes.push(0);
    let ptr = bytes.as_ptr();
    std::mem::forget(bytes);
    unsafe { std::ffi::CStr::from_ptr(ptr as *const c_string::c_char).as_ptr() }
}

#[inline(always)]
fn to_nsstr(s: &str) -> *const c_void {
    unsafe { OBJC_NSString(to_c_str(s)) }
}

type set_background_alpha = unsafe extern "C" fn(this: &Object, cmd: Sel, alpha: f64);

#[no_mangle]
extern "C" fn my_set_background_alpha(this: &Object, cmd: Sel, alpha: c_double) {
    unsafe {
        OBJC_NSLog(to_c_str(&format!(
            "ReachCCRust my_set_background_alpha: this = {:#?}, cmd = {:#?}, alpha = {}",
            this, cmd, alpha
        )));
        if let Some(orig) = ORIGIMP {
            let x: set_background_alpha = std::mem::transmute(orig);
            OBJC_NSLog(to_c_str(&format!(
                "ReachCCRust my_set_background_alpha = {:#?}",
                orig
            )));
            x(this, cmd, 0.0);
        }
    }
}

#[used]
#[cfg_attr(target_os = "ios", link_section = "__DATA,__mod_init_func")]
static LOAD: extern "C" fn() = {
    extern "C" fn ctor() {
        let sba_sel = sel!(setBackgroundAlpha:);
        unsafe {
            let sb_dock_view = objc_getClass(to_c_str("SBDockView"));
            let swizz_imp: *mut c_void = std::mem::transmute(my_set_background_alpha as set_background_alpha);
            OBJC_NSLog(to_c_str(&format!(
                "ReachCCRust hooking: swizz_imp = {:#?}, sb_dock_view = {:#?}, sba_sel = {:#?}",
                swizz_imp, sb_dock_view, sba_sel
            )));
            MSHookMessageEx(sb_dock_view, sba_sel, swizz_imp, &mut ORIGIMP);
        }
    }
    ctor
};
