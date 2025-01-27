use core::{arch::global_asm, include_str};

global_asm!(include_str!("asm/entry.S"));
global_asm!(include_str!("asm/trap.S"));

#[no_mangle]
pub fn init() {
    unsafe {
        setup_trap_vector();
    }
}

#[no_mangle]
unsafe fn setup_trap_vector() {
    extern "C" {
        fn trap_vector();
    }
}
