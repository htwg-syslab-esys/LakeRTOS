//! # General purpose input/output (GPIO)
//!
//! Peripheral is located at [AHB2][crate::dp::bus::AHB2]

use crate::util::register::Register;

/// General purpose input/output
///
/// [Reference Manual](https://www.st.com/resource/en/reference_manual/dm00043574-stm32f303xb-c-d-e-stm32f303x6-8-stm32f328x8-stm32f358xc-stm32f398xe-advanced-arm-based-mcus-stmicroelectronics.pdf)
/// GPIO registers - Section 11.4
#[repr(C)]
pub struct GPIO {
    pub moder: Register,
    pub otyper: Register,
    pub ospeedr: Register,
    pub pupdr: Register,
    pub idr: Register,
    pub odr: Register,
    pub bsrr: Register,
    pub lckr: Register,
    pub afrl: Register,
    pub afrh: Register,
    pub brr: Register,
}