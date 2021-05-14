extern crate sdl2;

use game::Game;
use input::InputCore;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod game;
mod input;
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
	let mut input_core = InputCore::new();

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

		input_core.update(&events);
		game.update(timer.ticks() as i32);

		canvas.clear();
		game.draw(&mut canvas)?;

		canvas.present();
		input_core.refresh_keys();

		if use_original_framerate {
			std::thread::sleep(Duration::from_millis(100));
		} else {
			std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	}

	Ok(())
}
