extern crate sdl2;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

use sprite::Sprite;
mod sprite;

fn main() -> Result<(), String> {
	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;

	let window = video_subsystem
		.window("video-game", 640, 480)
		.position_centered()
		.build()
		.map_err(|e| e.to_string())?;

	let mut canvas = window
		.into_canvas()
		.accelerated()
		.build()
		.map_err(|e| e.to_string())?;
	let texture_creator = canvas.texture_creator();

	canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));

	let timer = sdl_context.timer()?;

	let mut event_pump = sdl_context.event_pump()?;

	// animation sheet and extras are available from
	// https://opengameart.org/content/a-platformer-in-the-forest
	let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp"))?;
	let texture = texture_creator
		.create_texture_from_surface(&temp_surface)
		.map_err(|e| e.to_string())?;

	let frames_per_anim = 4;
	let sprite_tile_size = (32, 32);

	let mut baby = Sprite::new((0, 0), sprite_tile_size, frames_per_anim, Point::new(-64, 120));
	let mut king = Sprite::new((0, 32), sprite_tile_size, frames_per_anim, Point::new(0, 240));
	let mut soldier = Sprite::new((0, 64), sprite_tile_size, frames_per_anim, Point::new(440, 360));

	let mut use_original_framerate = true;

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

		baby.update(ticks, ((ticks / 14) % 768) - 128);
		king.update(ticks, -(((ticks / 12) % 768) - 672));
		soldier.update(ticks, ((ticks / 10) % 768) - 128);

		canvas.clear();

		baby.draw(&texture, false, &mut canvas)?;
		king.draw(&texture, true, &mut canvas)?;
		soldier.draw(&texture, false, &mut canvas)?;

		canvas.present();

		if use_original_framerate {
			std::thread::sleep(Duration::from_millis(100));
		} else {
			std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	}

	Ok(())
}
