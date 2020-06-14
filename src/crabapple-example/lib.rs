use crabapple::{hook_it, init_hooks};

hook_it! {
	mod dock_example {
		imports {
			use crabapple::deps::objc::runtime::*;
			use std::os::raw::c_double;
		}
		#[hook(class = "SBDockView", sel = "setBackgroundAlpha:")]
		fn sba(orig, this: &Object, cmd: Sel, alpha: c_double) {
			crabapple::objc::log(&format!("Crabapple dock_example | {:#?} - {:#?} - {:#?}", this, cmd, alpha));
			orig(this, cmd, 0.0);
		}
	}
}

hook_it! {
	mod notification_example {
		imports {
			use crabapple::deps::objc::runtime::*;
			use crabapple::util::from_nsstr;
		}
		#[hook(class = "BBServer", sel = "_publishBulletinRequest:")]
		fn pbr(orig, this: &crabapple::deps::objc::runtime::Object, cmd: crabapple::deps::objc::runtime::Sel,
			request: &crabapple::deps::objc::runtime::Object,
			appid: &crabapple::deps::objc::runtime::Object,
			arg3: u64)
		{
			let title = *request.get_ivar::<&Object>("title");
			let subtitle = *request.get_ivar::<&Object>("subtitle");
			let message = *request.get_ivar::<&Object>("message");
			crabapple::objc::log(&format!("Crabapple notification_example | {} - {} - {}", from_nsstr(title), from_nsstr(subtitle), from_nsstr(message)));
			orig(this, cmd, request, appid, arg3);
		}
	}
}

init_hooks!(dock_example, notification_example);
