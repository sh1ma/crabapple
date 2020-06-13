use crabapple::{hook_it, init_hooks};

hook_it! {
	mod hooks {
		#[hook(class = "SBDockView", sel = "setBackgroundAlpha:")]
		fn sba(this: &crabapple::deps::objc::runtime::Object, cmd: crabapple::deps::objc::runtime::Sel, alpha: std::os::raw::c_double) {
			let a = 25;
		}
	}
}

init_hooks!(hooks);
