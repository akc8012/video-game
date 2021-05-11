use std::path::Path;

use crate::sprite::Sprite;

use sdl2::{
	rect::Point,
	render::{Canvas, Texture, TextureCreator},
	surface,
	video::{Window, WindowContext},
};

pub struct Game<'a> {
	texture: Texture<'a>,

	baby: Sprite,
	king: Sprite,
	soldier: Sprite,
}

impl<'a> Game<'a> {
	pub fn start(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Game<'a>, String> {
		// animation sheet and extras are available from
		// https://opengameart.org/content/a-platformer-in-the-forest
		let surface = surface::Surface::load_bmp(Path::new("assets/characters.bmp"))?;
		let texture = texture_creator
			.create_texture_from_surface(&surface)
			.map_err(|e| e.to_string())?;

		let tile_size = (32, 32);
		let frames = 4;

		let baby = Sprite::new((0, 0), tile_size, frames, Point::new(-64, 120));
		let king = Sprite::new((0, 32), tile_size, frames, Point::new(0, 240));
		let soldier = Sprite::new((0, 64), tile_size, frames, Point::new(440, 360));

		Ok(Game {
			texture,
			baby,
			king,
			soldier,
		})
	}

	pub fn update(&mut self, ticks: i32) {
		self.baby.update(ticks, ((ticks / 14) % 768) - 128);
		self.king.update(ticks, -(((ticks / 12) % 768) - 672));
		self.soldier.update(ticks, ((ticks / 10) % 768) - 128);
	}

	pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
		self.baby.draw(&self.texture, false, canvas)?;
		self.king.draw(&self.texture, true, canvas)?;
		self.soldier.draw(&self.texture, false, canvas)?;

		Ok(())
	}
}
