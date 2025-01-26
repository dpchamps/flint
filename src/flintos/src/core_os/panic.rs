use core::panic::PanicInfo;
use core::arch::asm;

fn on_panic_tick() {
    unsafe {
        asm!("wfi")
    }
}

pub fn handle_panic(_panic_info: &PanicInfo) -> ! {
    loop {
        on_panic_tick();
    }
}