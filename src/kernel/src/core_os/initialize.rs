use crate::devices::output::{Device, Output};
use crate::drivers::uart::Uart;
use bootloader;

pub fn initialize_kernel() {
    bootloader::init();

    // TODO: better scanning for uart address
    Uart::init(0x10_000_000);

    // TODO: here, we're going to want to scan for devices.
    // This probably looks like this instead:
    // `Output::initialize(Device::scanOutputDevice())`
    Output::initialize(Device::Uart(Uart::create(0x10_000_000)))
}
