use luminance_glfw::{Action, Key, WindowEvent};

use std::collections::HashSet;

#[derive(Debug)]
pub struct KeyRegistry {
	active_keys: HashSet<Key>,
}

impl KeyRegistry {
	pub fn new() -> Self {
		Self {
			active_keys: HashSet::new(),
		}
	}

	pub fn event(&mut self, e: WindowEvent) {
		match e {
			WindowEvent::Key(k, _, Action::Press, _) => {
				if self.active_keys.contains(&k) {
					// ? Key started press two times
				} else {
					self.active_keys.insert(k);
				}
			}
			WindowEvent::Key(k, _, Action::Release, _) => {
				if self.active_keys.contains(&k) {
					self.active_keys.remove(&k);
				} else {
					// ? Key released two times
				}
			}
			_ => (),
		};
	}

	pub fn for_pressed_keys<F>(&self, mut f: F)
	where
		F: FnMut(&Key),
	{
		for key in &self.active_keys {
			f(key)
		}
	}
}
