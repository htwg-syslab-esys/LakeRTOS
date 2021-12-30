//! # Reset and clock controller
use core::ptr::{read_volatile, write_volatile};

/// Reset and clock controller
#[repr(C)]
pub struct RCC {
    ahbenr: u32,
}

impl RCC {
    /// IO PORT E ENABLE
    ///
    ///  p.166 "io port e enable"
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
