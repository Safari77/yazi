use std::borrow::Cow;

use yazi_macro::emit;
use yazi_shared::{event::Cmd, url::Url};

use crate::options::SearchOpt;

pub struct TabProxy;

impl TabProxy {
	#[inline]
	pub fn cd(target: &Url) {
		emit!(Call(Cmd::args("mgr:cd", [target])));
	}

	#[inline]
	pub fn reveal(target: &Url) {
		emit!(Call(Cmd::args("mgr:reveal", [target]).with("no-dummy", true)));
	}

	#[inline]
	pub fn arrow(step: impl Into<Cow<'static, str>>) {
		emit!(Call(Cmd::args("mgr:arrow", [step.into()])));
	}

	#[inline]
	pub fn search_do(opt: SearchOpt) {
		emit!(Call(
			// TODO: use second positional argument instead of `args` parameter
			Cmd::args("mgr:search_do", [opt.subject])
				.with("via", Cow::Borrowed(opt.via.into_str()))
				.with("args", opt.args_raw.into_owned())
		));
	}
}
