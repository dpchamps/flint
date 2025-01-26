#![no_std]

#[cfg(target_arch = "riscv64")]
pub mod riscv;

#[cfg(target_arch = "riscv64")]
pub use riscv::init;
