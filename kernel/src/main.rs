#![no_std]
#![no_main]
#![feature(lang_items)]
bootloader_api::entry_point!(kernel_main, config = &CONFIG);

pub mod drivers;
pub mod utilities;
pub mod gfx;

use core::panic::PanicInfo;

use drivers::serial;
use gfx::{Rect, Framebuffer};

const FRAME_BUFFER_WIDTH: usize = 1024;
const FRAME_BUFFER_HEIGHT: usize = 768;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();

    config.frame_buffer.minimum_framebuffer_width = Some(FRAME_BUFFER_WIDTH as u64);
    config.frame_buffer.minimum_framebuffer_height = Some(FRAME_BUFFER_HEIGHT as u64);

    config
};

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let scm = serial::init_serial();
    if scm.is_some() {
        unsafe {
            serial::COM1.lock().putln("COM1 init OK.");
        }
    }

    let fb_metadata = boot_info.framebuffer.as_mut().unwrap();

    let primary_framebuffer = Framebuffer::new(
        FRAME_BUFFER_WIDTH,
        FRAME_BUFFER_HEIGHT,
        2,
        fb_metadata.buffer_mut()
    );

    loop {}
}

#[panic_handler]
unsafe fn panic_handler(_info: &PanicInfo) -> ! {
    serial::COM1.lock().putln("Kernel panic!");
    loop {}
}
