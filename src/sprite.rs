use sdl2::{
	rect::{Point, Rect},
	render::{Canvas, Texture},
	video::Window,
};

pub struct Sprite {
	frames: i32,
	source_rect: Rect,
	dest_rect: Rect,
}

impl Sprite {
	pub fn new(origin: (i32, i32), tile_size: (u32, u32), frames: i32, center: Point) -> Sprite {
		let source_rect = Rect::new(0, 0, tile_size.0, tile_size.1);

		let mut dest_rect = Rect::new(
			origin.0,
			origin.1,
			tile_size.0 * (frames as u32),
			tile_size.1 * (frames as u32),
		);
		dest_rect.center_on(center);

		Sprite {
			frames,
			source_rect,
			dest_rect,
		}
	}

	pub fn update(&mut self, ticks: i32) {
		// set the current frame for time
		self.source_rect.set_x(32 * ((ticks / 100) % self.frames));
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
