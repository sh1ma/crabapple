pub mod ffi;
pub mod objc;
pub mod util;

pub mod deps {
	pub use ::objc;
	pub use paste;
}

/*
pub static ORIGIMP: AtomicPtr<c_void> = AtomicPtr::new(0 as *mut c_void);

type SetBackgroundAlpha = unsafe extern "C" fn(this: &Object, cmd: Sel, alpha: f64);

#[no_mangle]
extern "C" fn my_set_background_alpha(this: &Object, cmd: Sel, alpha: c_double) {
	log(&format!(
		"ReachCCRust my_set_background_alpha: this = {:#?}, cmd = {:#?}, alpha = {}",
		this, cmd, alpha
	));
	let ptr: *mut c_void = ORIGIMP.load(Ordering::Relaxed) as *mut _ as *mut c_void;
	unsafe {
		log(&format!("ReachCCRust my_set_background_alpha = {:?}", ptr));
		let nopac = ffi::ptr_strip(ptr);
		let x: SetBackgroundAlpha = std::mem::transmute(nopac);
		x(this, cmd, 0.0);
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
		hook("SBDockView", sba_sel, swizz_imp, &ORIGIMP);
	}
	ctor
};*/

#[macro_export]
macro_rules! sel {
	($name:expr) => {{
		$crate::deps::objc::sel_impl!(concat!($name, '\0'))
		}};
}

#[macro_export]
macro_rules! hook_it {
    (mod $mod_name:ident {
		imports {
            $($prefix:item)*
        }
        $(
            #[hook(class = $class:expr, sel = $sel:expr)]
            fn $fn_name:ident($orig:ident, $($arg:ident: $ty_:ty),*) $body:tt
        )*
    }) => {
        mod $mod_name {
			$($prefix)*
            $(
                $crate::deps::paste::item! {
                    type [<$fn_name _fn>] = unsafe extern "C" fn($($arg: $ty_),*);
                    pub static [<$fn_name _orig>]: std::sync::atomic::AtomicPtr<$crate::deps::objc::runtime::Imp> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

                    #[no_mangle]
                    extern "C" fn $fn_name($($arg: $ty_),*) {
                        unsafe {
							let [<$fn_name _ptr>]: *mut std::os::raw::c_void = [<$fn_name _orig>].load(std::sync::atomic::Ordering::Relaxed) as *mut _ as *mut std::os::raw::c_void;
							let [<$fn_name _nopac>] = $crate::ffi::ptr_strip([<$fn_name _ptr>]);
							let $orig: [<$fn_name _fn>] = std::mem::transmute([<$fn_name _nopac>]);
							$body
                        }
                    }
                }
            )*

            pub fn _INIT_HOOKS() {
                unsafe {
                    $(
						$crate::deps::paste::expr! {
							let target_sel = $crate::sel!($sel);
							let [<$fn_name _ptr>] = $fn_name as [<$fn_name _fn>] as usize as *mut std::os::raw::c_void;
							$crate::objc::log(&format!("Crabapple - Initializing class {}[{}] with hook {:#?}", $class, $sel, [<$fn_name _ptr>]));
							let mut trampoline: Option<std::ptr::NonNull<$crate::deps::objc::runtime::Imp>> = None;
							$crate::objc::hook($class, target_sel, [<$fn_name _ptr>], &mut trampoline);
							match trampoline {
								Some(t) => {
									let trampoline_ptr = t.as_ptr();
									[<$fn_name _orig>].store(trampoline_ptr, std::sync::atomic::Ordering::Relaxed);
									$crate::objc::log(&format!("Crabapple - Hooked class {}[{}] with trampoline {:#?}", $class, $sel, trampoline_ptr));
								},
								_ => {
									$crate::objc::log(&format!("Crabapple - Failed to hook class {}[{}]", $class, $sel));
								}
							}
						};
                    )*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! init_hooks {
	($($hook_mod:ident),*) => {
		#[used]
		#[cfg_attr(target_os = "ios", link_section = "__DATA,__mod_init_func")]
		static LOAD: extern "C" fn() = {
			extern "C" fn ctor() {
				$(
					$hook_mod::_INIT_HOOKS();
				)*
			}
		ctor
		};
	}
}
