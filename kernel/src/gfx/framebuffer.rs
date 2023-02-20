/// Used for internal graphics calculations involving windows/rendering/VGA.
pub struct Framebuffer<'a> {
	/// The width of the backing buffer for this frame.
	width: usize,
	/// The height of the backing buffer for this frame.
	height: usize,
	/// The number of bytes per pixel.
	depth: usize,
	/// The backing buffer for this frame.
	buffer: &'a mut [u8]
}

impl<'a> Framebuffer<'a> {
	/// Constructs a new framebuffer from the provided components.
	pub fn new(width: usize, height: usize, depth: usize, buffer: &'a mut [u8]) -> Framebuffer<'a> {
		Framebuffer {
			width,
			height,
			depth,
			buffer
		}
	}
}