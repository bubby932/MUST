use spin::Mutex;

use crate::utilities::driver::*;

use super::CharacterDevice;

/// COM1, pretty ubiquitous serial port on x86. Unlikely to
/// see this missing except on a personal computer, where
/// you don't need it anyway.
pub const COM1: Mutex<ComPort> = Mutex::new(ComPort::new(0x3F8));
/// COM2, pretty ubiquitous serial port on x86. Unlikely to
/// see this missing except on a personal computer, where
/// you don't need it anyway. That being said, this is less
/// readily available than COM1, since it's not as common.
pub const COM2: Mutex<ComPort> = Mutex::new(ComPort::new(0x2F8));
/// Not very common, not much reason to support this.
pub const COM3: Mutex<ComPort> = Mutex::new(ComPort::new(0x3E8));
/// Not very common, not much reason to support this.
pub const COM4: Mutex<ComPort> = Mutex::new(ComPort::new(0x2E8));
/// Not very common, not much reason to support this.
pub const COM5: Mutex<ComPort> = Mutex::new(ComPort::new(0x5F8));
/// Not very common, not much reason to support this.
pub const COM6: Mutex<ComPort> = Mutex::new(ComPort::new(0x4F8));
/// Not very common, not much reason to support this.
pub const COM7: Mutex<ComPort> = Mutex::new(ComPort::new(0x5E8));
/// Not very common, not much reason to support this.
pub const COM8: Mutex<ComPort> = Mutex::new(ComPort::new(0x4E8));

/// Value used for testing the COM port.
const COM_TEST_VALUE: u8 = 0xAE;

/// Simple struct representing a COM port and some
/// basic information about it.
pub struct ComPort {
    port: u16
}

impl CharacterDevice for ComPort {
    fn read(&self) -> u8 {
		// You should only ever access this through a mutex,
		// although that's not exactly enforced.
        unsafe { self.recv() }
    }

    fn write(&mut self, value: u8) {
		// Once again, this should be locked behind
		// a mutex
        unsafe { self.xmit(value) }
    }
}

#[derive(Default)]
pub struct SerialCapabilityMap {
	com1: bool,
	com2: bool,
	com3: bool,
	com4: bool,
	com5: bool,
	com6: bool,
	com7: bool,
	com8: bool
}

/// Initializes all serial ports if possible,
/// tests them all, and returns a list of those
/// that are both available and functional.
/// Returns None if no serial ports are available.
pub fn init_serial() -> Option<SerialCapabilityMap> {
	let mut map = SerialCapabilityMap::default();

	// Initialize all serial ports.
	unsafe {
		// These two are the most likely to be available.
		map.com1 = COM1.lock().init().is_ok();
		map.com2 = COM2.lock().init().is_ok();

		// These get gradually less likely to be there as we go.
		map.com3 = COM3.lock().init().is_ok();
		map.com4 = COM4.lock().init().is_ok();
		map.com5 = COM5.lock().init().is_ok();

		// These are the least likely to be there.
		map.com6 = COM6.lock().init().is_ok();
		map.com7 = COM7.lock().init().is_ok();
		map.com8 = COM8.lock().init().is_ok();
	}

	// Check if any serial ports are available.
	if map.com1 || map.com2 || map.com3 || map.com4 || map.com5 || map.com6 || map.com7 || map.com8 {
		Some(map)
	} else {
		None
	}
}

impl ComPort {
	/// Creates a new structure representing a COM port.
	/// This function does not prevent duplicates, so it's
	/// exclusively internal. This also means it's const-compatible.
    const fn new(port: u16) -> Self {
        Self { port }
    }

	/// Initializes this COM port.
	pub unsafe fn init(&self) -> Result<(), ()> {
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
		Ok(())
	}

	unsafe fn recv_ready(&self) -> bool {
		inb(self.port + 5) & 1 == 1
	}

	unsafe fn xmit_ready(&self) -> bool {
		inb(self.port + 5) & 0x20 == 0x20
	}

	/// Writes to the data register of this COM port.
	pub unsafe fn xmit(&self, byte: u8) {
		while !self.xmit_ready() { }
		outb(self.port, byte);
	}

	/// Reads from the data register of this COM port.
	pub unsafe fn recv(&self) -> u8 {
		while !self.recv_ready() { }
		inb(self.port)
	}

	pub unsafe fn puts(&self, string: &str) {
		for byte in string.bytes() {
			self.xmit(byte);
		}
	}

	pub unsafe fn putln(&self, string: &str) {
		self.puts(string);
		self.puts("\n\r");
	}
}
