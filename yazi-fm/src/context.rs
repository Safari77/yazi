use ratatui::layout::Rect;
use yazi_core::{completion::Completion, confirm::Confirm, help::Help, input::Input, manager::Manager, notify::Notify, pick::Pick, tab::{Folder, Tab}, tasks::Tasks, which::Which};
use yazi_shared::Layer;

pub struct Ctx {
	pub manager:    Manager,
	pub tasks:      Tasks,
	pub pick:       Pick,
	pub input:      Input,
	pub confirm:    Confirm,
	pub help:       Help,
	pub completion: Completion,
	pub which:      Which,
	pub notify:     Notify,
}

impl Ctx {
	pub fn make() -> Self {
		Self {
			manager:    Manager::make(),
			tasks:      Tasks::serve(),
			pick:       Default::default(),
			input:      Default::default(),
			confirm:    Default::default(),
			help:       Default::default(),
			completion: Default::default(),
			which:      Default::default(),
			notify:     Default::default(),
		}
	}

	#[inline]
	pub fn cursor(&self) -> Option<(u16, u16)> {
		if self.input.visible {
			let Rect { x, y, .. } = self.manager.area(self.input.position);
			return Some((x + 1 + self.input.cursor(), y + 1));
		}
		if let Some((x, y)) = self.help.cursor() {
			return Some((x, y));
		}
		None
	}

	#[inline]
	pub fn layer(&self) -> Layer {
		if self.which.visible {
			Layer::Which
		} else if self.completion.visible {
			Layer::Completion
		} else if self.help.visible {
			Layer::Help
		} else if self.confirm.visible {
			Layer::Confirm
		} else if self.input.visible {
			Layer::Input
		} else if self.pick.visible {
			Layer::Pick
		} else if self.active().spot.visible() {
			Layer::Spot
		} else if self.tasks.visible {
			Layer::Tasks
		} else {
			Layer::Manager
		}
	}
}

impl Ctx {
	#[inline]
	pub fn active(&self) -> &Tab { self.manager.active() }

	#[inline]
	pub fn active_mut(&mut self) -> &mut Tab { self.manager.active_mut() }

	#[inline]
	pub fn current(&self) -> &Folder { self.manager.current() }

	#[inline]
	pub fn current_mut(&mut self) -> &mut Folder { self.manager.current_mut() }
}
