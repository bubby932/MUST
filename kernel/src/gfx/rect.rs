#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Rect {
	x: usize,
	y: usize,
	width: usize,
	height: usize
}

impl Rect {
	pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
		Self {
			x,
			y,
			width,
			height
		}
	}
}