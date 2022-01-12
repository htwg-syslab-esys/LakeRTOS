//!
//! This file contains a struct containing the registers for the USART device. The fields of the struct are in C presentation
//! to prevent compiler mangling. The fields then match the offsets of the according register.
//!
//! 
use crate::util::register::Register;
use super::UART_BASE;
#[repr(C)]
pub struct UART {
    pub cr1: Register,
    pub cr2: Register,
    pub cr3: Register,
    pub brr: Register,
    pub gtpr: Register,
    pub rtor: Register,
    pub rqr: Register,
    pub isr: Register,
    pub icr: Register,
    pub rdr: Register,
    pub tdr: Register,
}

impl UART {
    ///
    /// Returns a new UART Struct based on the registers base adress. This adress gets
    /// casted to the struct, as a result the first field will equals the base
    /// adress. The following ones are stacked ontop each other with an offset of
    /// 4 byte / 32 bit.
    ///
    pub fn new() -> &'static mut UART {
        unsafe { &mut *(UART_BASE as *mut UART) }
    }
}
