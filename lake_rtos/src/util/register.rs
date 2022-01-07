//! # Register
//!
//! Light wrapper for register manipulation without error handling.
//!
//! For invalid or wrong inputs chances are that you destroy your system.
//!
//! Be careful!

use core::ptr::{read_volatile, write_volatile};

#[repr(C)]
#[derive(Debug)]
pub struct Register {
    register: u32,
}

#[allow(dead_code)]
impl Register {
    /// Returns the content of the according register
    ///
    /// # Arguments
    ///
    /// * `Nothing`
    ///
    /// # Returns
    /// * `u32` - The register content
    ///
    pub fn read(&mut self) -> u32 {
        unsafe { read_volatile(&mut self.register) }
    }

    pub fn read_bits(&mut self, _pos: u32, _length: u32) -> u32 {
        unimplemented!()
    }

    /// Sets a single bit to '1'
    ///
    /// Clears out the matching positions first.
    ///
    /// # Arguments
    ///
    /// * `pos` - A u32 which represents the bit position to be altered (LSB)
    ///
    /// # Returns
    /// * `Register` + A mutable Reference to the altered register
    ///
    /// ```text
    ///     0b0101_1010
    ///     0b0000_0100
    /// OR______________
    ///     0b0101_1110
    /// ```
    ///
    pub fn set_bit(&mut self, pos: u32) -> &mut Register {
        unsafe {
            write_volatile(
                &mut self.register as *mut u32,
                read_volatile(&mut self.register) | (1 << pos),
            );
        }
        self
    }

    /// Turns a given set of bits to '1'
    ///
    /// Builds a "0b..1111.." pattern first.
    ///
    /// # Arguments
    ///
    /// * `pos` - A u32 which represents the bit position to be altered (LSB)
    /// * `length` - A u32 which represents the number of bits to be set
    ///
    /// # Returns
    /// * `Register` + A mutable Reference to the altered register
    ///
    /// ```text
    ///     0b0101_1010
    ///     0b0001_1110
    /// OR______________
    ///     0b0101_1110
    /// ```
    ///
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

    /// Replaces a set of bits with the given pattern.
    ///
    /// Clears out the matching positions first.
    ///
    /// # Arguments
    ///
    /// * `pos` - A u32 which represents the bit position to be altered (LSB)
    /// * `length` - A u32 which represents the number of bits to be altered
    /// * `new_value` - A u32 which represents the new bit pattern
    ///
    /// # Returns
    /// * `Nothing`
    ///
    /// ```text
    ///     0b0101_1010
    ///     0b0001_0110
    /// OR______________
    ///     0b0101_0110
    /// ```
    ///
    pub fn replace_bits(&mut self, pos: u32, new_value: u32, length: u32) {
        self.clear_bits(pos, length);
        unsafe {
            write_volatile(
                &mut self.register as *mut u32,
                read_volatile(&mut self.register) | (new_value << pos),
            );
        }
    }

    /// Toggles a bit to its according opposite
    ///
    /// # Arguments
    ///
    /// * `pos` - A u32 which represents the bit position to be altered (LSB)
    /// * `length` - A u32 which represents the number of bits to be altered
    ///
    /// # Returns
    /// * `Register` + A mutable Reference to the altered register
    ///
    /// ```text
    ///     0b0101_1010
    ///     0b0001_1110
    /// XOR____________
    ///     0b0100_0100
    /// ```
    ///
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

    /// Clears one single bit
    ///
    /// # Arguments
    ///
    /// * `pos` - A u32 which represents the bit position to be cleared (LSB)
    ///
    /// # Returns
    /// * `Nothing`
    ///
    /// # Example
    ///
    /// let bits = 0b00101_1100;
    ///
    /// clear_bit(4);
    ///
    /// ```text
    ///     0b0101_1100
    ///     0b0001_0000
    /// NAND____________
    ///     0b0100_1100
    /// ```
    ///
    pub fn clear_bit(&mut self, pos: u32) {
        unsafe {
            write_volatile(
                &mut self.register as *mut u32,
                read_volatile(&mut self.register) & !(1 << pos),
            );
        }
    }

    ///
    /// Clears a block of bits
    ///
    /// # Arguments
    ///
    /// * `pos` - A u32 which represents the bit position to be altered (LSB)
    /// * `length` - A u32 which represents the number of bits to be altered
    ///
    /// # Returns
    /// * `None`
    ///
    /// # Example
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
    /// ```text
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

    /// Converts a amount as number into a block of bits matching the amount
    ///
    /// # Arguments
    ///
    /// * `length` - A u32 which represents the number of bits
    ///
    /// # Returns
    /// * `u32` - Format will be the number of bits a block like 4 : 0b1111
    ///
    /// # Example
    /// let length = 3;
    ///
    /// ```text
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
