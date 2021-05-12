extern crate sdl2;

use game::Game;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{collections::HashSet, time::Duration};

mod game;
mod sprite;

fn main() -> Result<(), String> {
	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;

	let window = video_subsystem
		.window("video-game", 640, 480)
		.position_centered()
		.build()
		.map_err(|e| e.to_string())?;

	let mut canvas: sdl2::render::Canvas<sdl2::video::Window> = window
		.into_canvas()
		.accelerated()
		.build()
		.map_err(|e| e.to_string())?;
	let texture_creator = canvas.texture_creator();

	let timer = sdl_context.timer()?;
	let mut events = sdl_context.event_pump()?;
	let mut use_original_framerate = true;

	let mut game = Game::start(&mut canvas, &texture_creator)?;
	let mut last_keys = HashSet::new();

	'running: loop {
		for event in events.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => break 'running,
				Event::KeyDown {
					keycode: Some(Keycode::F),
					..
				} => use_original_framerate = !use_original_framerate,
				_ => {}
			}
		}

		// Create a set of pressed Keys.
		let keys = events
			.keyboard_state()
			.pressed_scancodes()
			.filter_map(Keycode::from_scancode)
			.collect();

		// Get the difference between the new and old sets.
		let new_keys = &keys - &last_keys;
		let old_keys = &last_keys - &keys;

		if !new_keys.is_empty() || !old_keys.is_empty() {
			println!("new_keys: {:?}\told_keys:{:?}", new_keys, old_keys);
		}

		game.update(timer.ticks() as i32);

		canvas.clear();
		game.draw(&mut canvas)?;

		canvas.present();
		last_keys = keys;

		if use_original_framerate {
			std::thread::sleep(Duration::from_millis(100));
		} else {
			std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	}

	Ok(())
}
