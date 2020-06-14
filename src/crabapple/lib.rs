pub mod ffi;
pub mod objc;
pub mod util;

pub mod deps {
	pub use ::objc;
	pub use objc_foundation as foundation;
	pub use paste;
}

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
