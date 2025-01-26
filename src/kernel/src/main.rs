#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
mod core_os;
mod drivers;

use bootloader;

use drivers::uart::Uart;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    bootloader::init();
    Uart::init(0x10_000_000);

    let mut uart = Uart::create(0x10_000_000);
    uart.write_str("Chufty Chufty Chufty!\n").expect("");

    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    core_os::panic::handle_panic(info);
}
