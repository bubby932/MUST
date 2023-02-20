pub mod serial;

/// TODO
pub trait CharacterDevice {
	fn read(&self) -> u8;
	fn write(&mut self, value: u8);
}
/// TODO
pub trait BlockDevice {}