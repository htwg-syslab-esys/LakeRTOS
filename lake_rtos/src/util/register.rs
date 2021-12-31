//! # Register

use core::ptr::{read_volatile, write_volatile};

#[repr(C)]
pub struct Register {
    register: u32,
}

#[allow(dead_code)]
impl Register {
    pub fn read(&mut self) -> u32 {
        unsafe { read_volatile(&mut self.register) }
    }

    pub fn read_bits(&mut self, pos: u32, length: u32) -> u32 {
        unimplemented!()
    }

    pub fn set_bit(&mut self, pos: u32) -> &mut Register {
        unsafe {
            write_volatile(
                &mut self.register as *mut u32,
                read_volatile(&mut self.register) | (1 << pos),
            );
        }
        self
    }

    pub fn set_bits(&mut self, pos: u32, length: u32) -> &mut Register {
        let bit_ones = Register::length_to_ones_in_bit(length);
        unsafe {
            write_volatile(
                &mut self.register as *mut u32,
                read_volatile(&mut self.register) | (bit_ones << pos),
            );
        }
        self
    }

    pub fn replace_bits(&mut self, pos: u32, new_value: u32, length: u32) {
        self.clear_bits(pos, length);
        unsafe {
            write_volatile(
                &mut self.register as *mut u32,
                read_volatile(&mut self.register) | (new_value << pos),
            );
        }
    }

    pub fn flip_bits(&mut self, pos: u32, length: u32) -> &mut Register {
        let bit_ones = Register::length_to_ones_in_bit(length);
        let mask = bit_ones << pos;
        unsafe {
            write_volatile(
                &mut self.register as *mut u32,
                read_volatile(&mut self.register) ^ mask
            );
        }
        self
    }

    pub fn clear_bit(&mut self, pos: u32) {
        unsafe {
            write_volatile(
                &mut self.register as *mut u32,
                read_volatile(&mut self.register) & !(1 << pos),
            );
        }
    }

    pub fn clear_bits(&mut self, pos: u32, length: u32) {
        let bit_ones = Register::length_to_ones_in_bit(length);
        unsafe {
            write_volatile(
                &mut self.register as *mut u32,
                read_volatile(&mut self.register) & !(bit_ones << pos),
            );
        }
    }

    pub fn length_to_ones_in_bit(length: u32) -> u32 {
        let mut bit_ones = 0;
        for _ in 0..length {
            bit_ones = (bit_ones << 1) | 0b1;
        }
        bit_ones
    }
}
