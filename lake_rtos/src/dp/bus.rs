//! # Bus
use super::{gpio::GPIO, rcc::RCC, GPIOA_BASE, GPIOE_BASE, RCC_BASE};

/// The AHB slave interface allows internal CPUs and other bus master peripherals to access
/// the external memories.
pub struct BusInterface;

impl BusInterface {
    pub fn ahb1(&self) -> AHB1 {
        AHB1 {
            rcc: unsafe { &mut *(RCC_BASE as *mut RCC) },
        }
    }
    pub fn ahb2(&self) -> AHB2 {
        AHB2 {
            gpioa: unsafe { &mut *(GPIOA_BASE as *mut GPIO) },
            gpioe: unsafe { &mut *(GPIOE_BASE as *mut GPIO) },
        }
    }
}

/// Advanced high-performance bus 1
///
/// Allows access to
/// - [RCC]
pub struct AHB1 {
    rcc: &'static mut RCC,
}

impl AHB1 {
    pub fn rcc(&mut self, f: fn(&mut RCC) -> &mut RCC) {
        f(self.rcc);
    }
}

/// Advanced high-performance bus 2
///
/// Allows access to
/// - [GPIO]
pub struct AHB2 {
    gpioe: &'static mut GPIO,
    gpioa: &'static mut GPIO,
}

impl AHB2 {
    pub fn gpioe(self) -> &'static mut GPIO {
        self.gpioe
    }
    pub fn gpioa(self) -> &'static mut GPIO {
        self.gpioa
    }
}
