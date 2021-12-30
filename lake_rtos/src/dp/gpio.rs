//! # General purpose input/output (GPIO)

/// General purpose input/output
///
/// [Reference Manual](https://www.st.com/resource/en/reference_manual/dm00043574-stm32f303xb-c-d-e-stm32f303x6-8-stm32f328x8-stm32f358xc-stm32f398xe-advanced-arm-based-mcus-stmicroelectronics.pdf)
/// GPIO registers - Section 11.4
#[repr(C)]
pub struct GPIO {
    pub moder: u32,
    pub otyper: u32,
    pub ospeedr: u32,
    pub pupdr: u32,
    pub idr: u32,
    pub odr: u32,
    pub bsrr: u32,
    pub lckr: u32,
    pub afrl: u32,
    pub afrh: u32,
    pub brr: u32,
}
