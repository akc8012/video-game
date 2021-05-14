use sdl2::{keyboard::Keycode, EventPump};
use std::{collections::HashSet, ops::Index};

static TRUE: bool = true;
static FALSE: bool = false;

pub struct Input<'a> {
	input_core: &'a InputCore,
}

impl<'a> Input<'a> {
	pub fn new(input_core: &'a InputCore) -> Input<'a> {
		Input { input_core }
	}
}

impl<'a> Index<&Keycode> for Input<'a> {
	type Output = bool;

	fn index(&self, index: &Keycode) -> &Self::Output {
		if self.input_core.keys.contains(index) {
			&TRUE
		} else {
			&FALSE
		}
	}
}

pub struct InputCore {
	pub keys: HashSet<Keycode>,
	last_keys: HashSet<Keycode>,
}

impl InputCore {
	pub fn new() -> InputCore {
		InputCore {
			keys: HashSet::new(),
			last_keys: HashSet::new(),
		}
	}

	pub fn update(&mut self, events: &EventPump) {
		// Create a set of pressed Keys.
		self.keys = events
			.keyboard_state()
			.pressed_scancodes()
			.filter_map(Keycode::from_scancode)
			.collect();

		// Get the difference between the new and old sets.
		let new_keys = &self.keys - &self.last_keys;
		let old_keys = &self.last_keys - &self.keys;

		if !new_keys.is_empty() || !old_keys.is_empty() {
			println!("new_keys: {:?}\told_keys:{:?}", new_keys, old_keys);
		}
	}

	pub fn refresh_keys(&mut self) {
		self.last_keys = self.keys.clone();
	}
}
