use core::arch::asm;
use core::panic::PanicInfo;
#[no_mangle]
fn on_panic_tick() {
    unsafe { asm!("wfi") }
}

#[no_mangle]
pub fn handle_panic(_panic_info: &PanicInfo) -> ! {
    loop {
        on_panic_tick();
    }
}
