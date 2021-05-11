extern crate sdl2;

use game::Game;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

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

	canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));

	let timer = sdl_context.timer()?;

	let mut event_pump = sdl_context.event_pump()?;
	let mut use_original_framerate = true;

	let mut game = Game::start(&texture_creator)?;

	'running: loop {
		for event in event_pump.poll_iter() {
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

		let ticks = timer.ticks() as i32;
		game.update(ticks);

		canvas.clear();
		game.draw(&mut canvas)?;

		canvas.present();

		if use_original_framerate {
			std::thread::sleep(Duration::from_millis(100));
		} else {
			std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	}

	Ok(())
}
