use objc::runtime::*;
use objc::*;
use std::os::raw::{c_char, c_double, c_void};

pub static mut ORIGIMP: Option<Imp> = None;

extern "C" {
    fn OBJC_NSString(str: *const c_char) -> *mut c_void;
    fn OBJC_NSLog(str: *const c_char);
    fn NSLogv(nsFormat: *mut c_void); // format from inside rust or it dies
    fn MSHookMessageEx(class: *mut c_void, selector: *mut c_void, replacement: *mut c_void, result: *mut c_void);
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

type set_background_alpha = unsafe extern "C" fn(this: &Object, _cmd: Sel, alpha: f64) -> c_double;

#[no_mangle]
extern "C" fn my_set_background_alpha(this: &Object, cmd: Sel, alpha: c_double) -> c_double {
    unsafe {
        OBJC_NSLog(to_c_str(&format!(
            "ReachCCRust my_set_background_alpha: this = {:#?}, cmd = {:#?}, alpha = {}",
            this, cmd, alpha
        )));
    }
    return 0.0;
}

#[used]
#[cfg_attr(target_os = "ios", link_section = "__DATA,__mod_init_func")]
static LOAD: extern "C" fn() = {
    extern "C" fn ctor() {
        unsafe {
            let f: set_background_alpha = my_set_background_alpha;
            let swizz_imp: *mut c_void = std::mem::transmute(f);
            let sb_dock_view: *mut c_void = std::mem::transmute(objc_getClass(to_c_str("SBDockView")));
            let sba_sel: *mut c_void = std::mem::transmute(sel!(setBackgroundAlpha:));
            let mut replaced: *mut c_void = std::ptr::null_mut();
            OBJC_NSLog(to_c_str(&format!(
                "ReachCCRust hooking: swizz_imp = {:#?}, sb_dock_view = {:#?}, sba_sel = {:#?}",
                swizz_imp, sb_dock_view, sba_sel
            )));
            MSHookMessageEx(sb_dock_view, sba_sel, swizz_imp, &mut replaced as *mut *mut c_void as *mut c_void);
            OBJC_NSLog(to_c_str(&format!(
                "ReachCCRust hooking: replaced = {:#?}",
                replaced
            )));
            ORIGIMP = Some(std::mem::transmute::<*mut c_void, Imp>(replaced));
        }
    }
    ctor
};
