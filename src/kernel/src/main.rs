#![no_std]
#![no_main]

use core::fmt::Write;
use core::ops::Deref;
use core::panic::PanicInfo;

mod core_os;
mod devices;
mod drivers;

use core_os::{critical_section, initialize_kernel};
use devices::output::Output;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    initialize_kernel();
    write!(Output::writer(), "Hello!").expect("");
    panic!("Uh oh spaghettio!");
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    core_os::panic::handle_panic(info);
}
