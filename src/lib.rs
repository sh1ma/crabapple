use objc::runtime::*;
use objc::*;
use std::os::raw::{c_char, c_double, c_void};

pub static mut ORIGIMP: Option<Imp> = None;

extern "C" {
    fn OBJC_NSString(str: *const c_char) -> *mut c_void;
    fn OBJC_NSLog(str: *const c_char);
    fn NSLogv(nsFormat: *mut c_void); // format from inside rust or it dies
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
extern "C" fn my_set_background_alpha(_this: &Object, _cmd: Sel, _alpha: c_double) -> c_double {
    return 0.0;
}

#[used]
#[cfg_attr(target_os = "ios", link_section = "__DATA,__mod_init_func")]
static LOAD: extern "C" fn() = {
    extern "C" fn ctor() {
        unsafe {
            let method = class_getInstanceMethod(
                objc_getClass(to_c_str("SBDockView")),
                sel!(backgroundAlpha:),
            ) as *mut Method;
            // first need to get a function pointer
            let f: set_background_alpha = my_set_background_alpha;
            // then we can transmute it to change its type
            let swizz_imp: Imp = std::mem::transmute(f);
            OBJC_NSLog(to_c_str(&format!(
                "ReachCCRust: method = {:#?}, f = {:#?}, swizz_imp = {:#?}",
                std::mem::transmute::<*mut Method, *mut c_void>(method),
                std::mem::transmute::<set_background_alpha, *mut c_void>(f),
                std::mem::transmute::<Imp, *mut c_void>(swizz_imp)
            )));
            ORIGIMP = Some(method_setImplementation(method, swizz_imp));
        }
    }
    ctor
};
