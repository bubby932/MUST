#![no_std]
#![no_main]
bootloader_api::entry_point!(kernel_main);

pub mod drivers;
pub mod utilities;

use drivers::serial;

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    unsafe {
        serial::COM1.init()
    }
    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    todo!()
}
