extern crate objc;


use std::os::raw::{c_char, c_void};

extern "C" {
    fn OBJC_NSString(str: *const c_char) -> *mut c_void;
    fn OBJC_NSLog(str: *const c_char);
    fn NSLogv(nsFormat: *mut c_void); // formast from inside rust or it dies
}

#[inline(always)]
fn to_c_str(s: &str) -> *const c_char {
    let mut bytes = String::from(str).into_bytes();
    bytes.push(0);
    let ptr = bytes.as_ptr();
    std::mem::forget(bytes);
    unsafe {
        std::ffi::CStr::from_ptr(ptr as *const c_string::c_char).as_ptr()
    }
}

#[used]
#[cfg_attr(target_os = "ios", link_section = "__DATA,__mod_init_func")]
static LOAD: extern fn() = {
    extern fn ctor() {
        unsafe {
            let a1 = OBJC_NSString(to_c_str("HELLO FROM RUST (NSlogv)"));
            println!("a1={:#?}", a1);
            NSLogv(a1);
            OBJC_NSLog(to_c_str("TESTING! From RUST! (OBJC_NSLog)"));
        } 
    }
    ctor  
};

/*
pub static mut ORIGIMP: Option<Imp> = None;

type setBackgroundAlpha = unsafe extern fn(this: &Object, _cmd: Sel, alpha: f64);

#[no_mangle]
extern "C" fn my_setBackgroundAlpha(this: &Object, _cmd: Sel, _alpha: f64) {
    unsafe {
        if let Some(orig) = ORIGIMP {
            let orig: setBackgroundAlpha = std::mem::transmute(orig);
            orig(this, _cmd, 0.0)
        }
    }
}

#[used]
#[cfg_attr(target_os = "ios", link_section = "__DATA,__mod_init_func")]
static LOAD: extern fn() = {
    extern fn ctor() {
        unsafe {
            let method = class_getInstanceMethod(
                objc_getClass(to_c_str("SBDockView")),
                sel!(setBackgroundAlpha:),
            ) as *mut Method;
            let swizz_imp: Imp = std::mem::transmute(&my_setBackgroundAlpha);
            ORIGIMP = Some(method_setImplementation(method, swizz_imp));
        } 
    }
    ctor  
};
*/