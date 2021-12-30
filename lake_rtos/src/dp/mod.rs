//! # Device Peripherals

pub mod bus;
pub mod gpio;
pub mod rcc;

use self::bus::BusInterface;
use core::mem::replace;

const GPIOE_BASE: u32 = 0x4800_1000;
const RCC_AHBENR: u32 = 0x4002_1000;

/// This static mut is used for a singleton pattern. Static muts are unsafe by default.
/// It is the programmers responsibility to make sure the logic behind it is safe.
pub static mut DEVICE_PERIPHERALS: DevicePeripherals = DevicePeripherals {
    bus_interface: Some(BusInterface),
};

/// Holds the bus interface that connects to other peripherals
pub struct DevicePeripherals {
    bus_interface: Option<BusInterface>,
}

impl DevicePeripherals {
    pub fn take() -> BusInterface {
        let p = replace(unsafe { &mut DEVICE_PERIPHERALS.bus_interface }, None);
        p.unwrap()
    }
}
