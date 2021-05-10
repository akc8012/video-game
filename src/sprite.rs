use sdl2::{
	rect::{Point, Rect},
	render::{Canvas, Texture},
	video::Window,
};

pub struct Sprite {
	tile_size: (u32, u32),
	frames: u32,
	source_rect: Rect,
	dest_rect: Rect,
}

impl Sprite {
	pub fn new(origin: (i32, i32), tile_size: (u32, u32), frames: u32, center: Point) -> Sprite {
		let source_rect = Rect::new(0, 0, tile_size.0, tile_size.1);

		let mut dest_rect = Rect::new(origin.0, origin.1, tile_size.0 * frames, tile_size.1 * frames);
		dest_rect.center_on(center);

		Sprite {
			tile_size,
			frames,
			source_rect,
			dest_rect,
		}
	}

	pub fn update(&mut self, ticks: i32) {
		// animate sprite sheet
		self.source_rect
			.set_x((self.tile_size.0 as i32) * ((ticks / 100) % self.frames as i32));

		// glide across the screen
		self.dest_rect.set_x(((ticks / 14) % 768) - 128);
	}

	pub fn draw(&self, texture: &Texture, canvas: &mut Canvas<Window>) -> Result<(), String> {
		// copy the frame to the canvas
		canvas.copy_ex(
			texture,
			Some(self.source_rect),
			Some(self.dest_rect),
			0.0,
			None,
			false,
			false,
		)
	}
}
