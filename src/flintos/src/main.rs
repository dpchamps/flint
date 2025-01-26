#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod core_os;
use bootloader;
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    bootloader::init();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    core_os::panic::handle_panic(info);
}
