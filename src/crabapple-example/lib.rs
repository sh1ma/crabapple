use crabapple::{hook_it, init_hooks};

hook_it! {
	mod dock_example {
		#[hook(class = "SBDockView", sel = "setBackgroundAlpha:")]
		fn sba(orig, this: &crabapple::deps::objc::runtime::Object, cmd: crabapple::deps::objc::runtime::Sel, _alpha: std::os::raw::c_double) {
			orig(this, cmd, 0.0);
		}
	}
}

hook_it! {
	mod notification_example {
		#[hook(class = "BBServer", sel = "_publishBulletinRequest:")]
		fn pbr(orig, this: &crabapple::deps::objc::runtime::Object, cmd: crabapple::deps::objc::runtime::Sel,
			request: &crabapple::deps::objc::runtime::Object,
			appid: &crabapple::deps::objc::runtime::Object,
			arg3: u64)
		{
			use crabapple::deps::objc::runtime::*;
			use crabapple::util::from_nsstr;

			let title = *request.get_ivar::<&Object>("title");
			let subtitle = *request.get_ivar::<&Object>("subtitle");
			let message = *request.get_ivar::<&Object>("message");
			crabapple::objc::log(&format!("Crabapple | {} - {} - {}", from_nsstr(title), from_nsstr(subtitle), from_nsstr(message)));
			orig(this, cmd, request, appid, arg3);
		}
	}
}

init_hooks!(dock_example, notification_example);
