use crate::devices::output::Output;
use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;

#[no_mangle]
pub fn handle_panic(panic_info: &PanicInfo) -> ! {
    match Output::try_writer() {
        None => {}
        Some(mut writer) => write!(writer, "\n{}\n", panic_info).unwrap_or_else(|_| {}),
    }

    unsafe {
        loop {
            // TODO: jank for now, but we probably want to do something like this:
            // wait for some time, then try to reboot
            // It would be good to dump registers here as well and output as much as possible
            // about the state of the computer
            for _ in 1..10000000 {
                asm!("nop")
            }
            (0x100000 as *mut u32).write_volatile(0x5555);
        }
    }
}
