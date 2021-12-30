//! # Reset and clock controller
//! 
//! Peripheral is located at [AHB1][crate::dp::bus::AHB1]

use core::ptr::{read_volatile, write_volatile};
/// Reset and clock controller
///
/// [Reference Manual](https://www.st.com/resource/en/reference_manual/dm00043574-stm32f303xb-c-d-e-stm32f303x6-8-stm32f328x8-stm32f358xc-stm32f398xe-advanced-arm-based-mcus-stmicroelectronics.pdf)
/// RCC register map - Section 9.4.14
#[repr(C)]
pub struct RCC {
    cr: u32,
    cfgr: u32,
    cir: u32,
    apb2rstr: u32,
    apb1rstr: u32,
    ahbenr: u32,
    apb2enr: u32,
    apb1enr: u32,
    bdcr: u32,
    csr: u32,
    ahbrstr: u32,
    cfgr2: u32,
    cfgr3: u32,
}

impl RCC {
    /// Enables port e
    pub fn iopeen(&mut self) -> &mut RCC {
        unsafe {
            write_volatile(
                &mut self.ahbenr as *mut u32,
                read_volatile(&mut self.ahbenr) | (1 << 21),
            )
        };
        self
    }
}
