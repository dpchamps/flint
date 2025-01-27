use crate::drivers::uart::Uart;
use core::fmt::Write;
use once_cell::sync::Lazy;
use spin::{Mutex, MutexGuard};

pub enum Device {
    Uart(Uart),
}

/// Before getting into userspace, we need a place to write to somewhere.
/// Instead of making assumptions about what device to write to every time we write,
/// manage this through abstraction.
/// When the kernel boots, it will scan, choose the best target and then set this up.
pub struct Output {
    active_device: Option<Device>,
}

impl Output {
    fn new() -> Output {
        Self {
            active_device: None,
        }
    }
    fn set_active_device(&mut self, device: Device) {
        self.active_device = Some(device);
    }

    pub fn initialize(device: Device) {
        match KERNEL_OUTPUT_MANAGER.try_lock() {
            None => panic!(""),
            Some(mut output) => output.set_active_device(device),
        }
    }

    pub fn writer<'t>() -> MutexGuard<'t, Output> {
        KERNEL_OUTPUT_MANAGER.lock()
    }

    pub fn try_writer<'t>() -> Option<MutexGuard<'t, Output>> {
        KERNEL_OUTPUT_MANAGER.try_lock()
    }
}

impl Write for Output {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match &mut self.active_device {
            None => panic!("tried to write to output but no device was initialized."),
            Some(Device::Uart(uart)) => write!(uart, "{}", s),
        }
    }
}

static KERNEL_OUTPUT_MANAGER: Lazy<Mutex<Output>> = Lazy::new(|| Mutex::new(Output::new()));
