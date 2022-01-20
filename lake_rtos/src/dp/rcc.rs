//! # Reset and clock controller
//!
//! Peripheral is located at [AHB1][crate::dp::bus::AHB1]

use crate::util::register::Register;
/// Reset and clock controller
///
/// [Reference Manual](https://www.st.com/resource/en/reference_manual/dm00043574-stm32f303xb-c-d-e-stm32f303x6-8-stm32f328x8-stm32f358xc-stm32f398xe-advanced-arm-based-mcus-stmicroelectronics.pdf)
/// RCC register map - Section 9.4.14
#[repr(C)]
pub struct RCC {
    cr: Register,
    cfgr: Register,
    cir: Register,
    apb2rstr: Register,
    apb1rstr: Register,
    ahbenr: Register,
    apb2enr: Register,
    apb1enr: Register,
    bdcr: Register,
    csr: Register,
    ahbrstr: Register,
    cfgr2: Register,
    cfgr3: Register,
}

impl RCC {
    /// Enables port e
    pub fn iopeen(&mut self) -> &mut RCC {
        self.ahbenr.set_bit(21);
        self
    }
}
