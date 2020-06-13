use crabapple::{hook_it, init_hooks};

hook_it! {
	mod hooks {
		#[hook(class = "SBDockView", sel = "setBackgroundAlpha:")]
		fn sba(orig, this: &crabapple::deps::objc::runtime::Object, cmd: crabapple::deps::objc::runtime::Sel, alpha: std::os::raw::c_double) {
			orig(this, cmd, 0.0);
		}
	}
}

init_hooks!(hooks);
