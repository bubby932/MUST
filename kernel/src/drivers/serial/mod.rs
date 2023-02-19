use crate::utilities::driver::*;

use super::CharacterDevice;

/// COM1, pretty ubiquitous serial port on x86. Unlikely to
/// see this missing except on a personal computer, where
/// you don't need it anyway.
pub const COM1: ComPort = ComPort::new(0x3F8);
/// COM2, pretty ubiquitous serial port on x86. Unlikely to
/// see this missing except on a personal computer, where
/// you don't need it anyway. That being said, this is less
/// readily available than COM1, since it's not as common.
pub const COM2: ComPort = ComPort::new(0x2F8);
/// Not very common, not much reason to support this.
pub const COM3: ComPort = ComPort::new(0x3E8);
/// Not very common, not much reason to support this.
pub const COM4: ComPort = ComPort::new(0x2E8);
/// Not very common, not much reason to support this.
pub const COM5: ComPort = ComPort::new(0x5F8);
/// Not very common, not much reason to support this.
pub const COM6: ComPort = ComPort::new(0x4F8);
/// Not very common, not much reason to support this.
pub const COM7: ComPort = ComPort::new(0x5E8);
/// Not very common, not much reason to support this.
pub const COM8: ComPort = ComPort::new(0x4E8);

/// Value used for testing the COM port.
const COM_TEST_VALUE: u8 = 0xAE;

/// Simple struct representing a COM port and some
/// basic information about it.
pub struct ComPort {
    port: u16
}

impl CharacterDevice for ComPort {
	// TODO
}

impl ComPort {
	/// Creates a new structure representing a COM port.
	/// This function does not prevent duplicates, so it's
	/// exclusively internal. This also means it's const-compatible.
    const fn new(port: u16) -> Self {
        Self { port }
    }

	/// Initializes this COM port.
	pub unsafe fn init(self) -> Result<Self, ()> {
		outb(self.port + 1, 0x00);
		outb(self.port + 3, 0x80);
		outb(self.port + 0, 0x03);
		outb(self.port + 1, 0x00);
		outb(self.port + 3, 0x03);
		outb(self.port + 2, 0xC7);
		outb(self.port + 4, 0x0B);
		outb(self.port + 4, 0x1E);
		
		// Port check
		outb(self.port + 0, COM_TEST_VALUE);
		if inb(self.port) != COM_TEST_VALUE {
			return Err(());
		}
		
		// Port check OK.
		// Set back to normal operation.
		outb(self.port + 4, 0x0F);

		// Initialization complete! :)
		Ok(self)
	}

	/// Writes to the data register of this COM port.
	pub unsafe fn write(&self, byte: u8) {
		outb(self.port, byte);
	}
}
