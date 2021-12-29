//! # Device Peripherals

pub mod bus;
pub mod gpio;
pub mod rcc;

use self::bus::BusInterface;
use core::mem::replace;

const GPIOE_BASE: u32 = 0x4800_1000;
const RCC_AHBENR: u32 = 0x4002_1000 | 0x14;

/// This is the singleon Pattern static mut. Static muts are by default unsafe.
pub static mut DEVICE_PERIPHERALS: DevicePeripherals = DevicePeripherals {
    serial: Some(BusInterface),
};

pub struct DevicePeripherals {
    serial: Option<BusInterface>,
}

impl DevicePeripherals {
    pub fn take() -> BusInterface {
        let p = replace(unsafe { &mut DEVICE_PERIPHERALS.serial }, None);
        p.unwrap()
    }
}
