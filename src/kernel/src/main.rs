#![no_std]
#![no_main]
extern crate alloc;

use alloc::{boxed::Box, vec::Vec, string::String, format, vec};
use alloc::string::ToString;
use core::fmt::Write;
use core::ops::Deref;
use core::panic::PanicInfo;

mod core_os;
mod devices;
mod drivers;

use core_os::{critical_section, initialize_kernel};
use talc::{Talck, ClaimOnOom, Talc, Span};

static mut ARENA: [u8; 10000] = [0; 10000];
#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ClaimOnOom> = Talc::new(unsafe {
    ClaimOnOom::new(Span::from_array(core::ptr::addr_of!(ARENA).cast_mut()))
}).lock();

#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			use devices::output::Output;
			let _ = write!(Output::writer(), $($args)+);
	});
}
#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}


#[no_mangle]
pub extern "C" fn kmain() -> ! {
    initialize_kernel();
    let boxed = Box::new(10);
    println!("Hello!");
    println!("Boxed: {}", boxed);
	println!("Vector: {:?}", vec![1,2,3,4,5]);
    // panic!("Uh oh spaghettio!");
    loop {
        unsafe {
            (0x100000 as *mut u32).write_volatile(0x5555);
            core::arch::asm!("wfi")
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    core_os::panic::handle_panic(info);
}
