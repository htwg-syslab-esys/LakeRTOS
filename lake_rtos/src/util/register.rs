//! # Register
//!
//! Light wrapper for register manipulation without error handling.
//!
//! For invalid or wrong inputs chances are that you destroy your system.
//!
//! Be careful!

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

    pub fn read_bits(&mut self, _pos: u32, _length: u32) -> u32 {
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
                read_volatile(&mut self.register) ^ mask,
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

    /// 
    /// # Arguments
    /// 
    /// * `pos` - A u32 which represents the bit position to be altered (LSB)
    /// * `length` - A u32 which represents the number of bits to be altered
    ///
    /// # Returns
    /// * `None`
    /// 
    /// 
    /// Example:
    /// let bits = 0b00101_1100;
    /// 
    /// clear_bits(2, 2);
    /// 
    /// *step 1:* get block of bits containg only 1's
    /// bit_ones = Register::length_to_ones_in_bit(length); // 0b11
    /// 
    /// *step 2:* move to desired position
    /// to_clear = bit_ones << pos; // 0b1100
    /// 
    /// *step 3:* perform NAND operation
    /// ```
    ///     0b0010_1100
    ///     0b0000_1100
    /// NAND___________
    ///     0b0010_0000
    /// ```
    ///
    pub fn clear_bits(&mut self, pos: u32, length: u32) {
        let bit_ones = Register::length_to_ones_in_bit(length);
        unsafe {
            write_volatile(
                &mut self.register as *mut u32,
                read_volatile(&mut self.register) & !(bit_ones << pos),
            );
        }
    }

    /// 
    /// # Arguments
    /// 
    /// * `length` - A u32 which represents the number of bits 
    ///
    /// # Returns
    /// * `u32` - Format will be the number of bits a block like 4 : 0b1111
    /// 
    /// 
    /// Example:
    /// let length = 3;
    /// 
    /// ```
    /// bit_ones = 0;
    /// 
    /// // 1st iteration
    /// bit_ones = 0b0<<1  | 0b1 -> 0b1
    /// // 2nd iteration  
    /// bit_ones = 0b1<<1  | 0b1 -> 0b11
    /// // 3rd iteration
    /// bit_ones = 0b11<<1 | 0b1 -> 0b111
    /// ```
    ///
    pub fn length_to_ones_in_bit(length: u32) -> u32 {
        let mut bit_ones = 0;
        for _ in 0..length {
            bit_ones = (bit_ones << 1) | 0b1;
        }
        bit_ones
    }
}
