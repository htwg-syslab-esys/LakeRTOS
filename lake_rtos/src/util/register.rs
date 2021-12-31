//! # Register

use core::ptr::{write_volatile, read_volatile};
#[repr(C)]
pub struct Register {
    register: u32,
}

impl Register {
    pub fn read_bit(left_shift: u32, length: u32) {}

    pub fn set_bit(&mut self, left_shift: u32) -> &mut Register {
        unsafe {
            write_volatile(
                &mut self.register as *mut u32,
                read_volatile(&mut self.register) | (1 << left_shift),
            );
        }
        self
    }

    pub fn set_bits(left_shift: u32, bits: u32) {}

    pub fn clear_bit(left_shift: u32, length: u32) {}

    pub fn clear_bits(left_shits: u32, bits: u32) {}
}