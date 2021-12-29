//! # LEDs
//! 
//! Convenient wrapper for the LEDs. Supported by the discovery board. 
use core::ptr::{read_volatile, write_volatile};
use crate::dp::gpio::GPIO;

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum CardinalPoints {
    NorthWest = 8,
    North,     // 9
    NorthEast, // 10
    East,      // etc.
    SouthEast,
    South,
    SouthWest,
    West,
}

/// Need [GPIO] port E.
///
/// There are eight LEDs in an cardinal points layout. They can be accessed with enum type [CardinalPoints].
pub struct LEDs {
    gpioe: &'static mut GPIO,
    initialized: [bool; 8],
}

impl LEDs {
    pub fn new(gpioe: &'static mut GPIO) -> LEDs {
        LEDs {
            gpioe,
            initialized: [false; 8],
        }
    }

    /// If necessary, initializes the LED.
    pub fn check_init(&mut self, led: CardinalPoints) {
        if !self.initialized[led as usize - 8] {
            unsafe {
                write_volatile(
                    &mut self.gpioe.moder as *mut u32,
                    read_volatile(&mut self.gpioe.moder) | (0b01 as u32) << (led as usize * 2),
                );
                write_volatile(
                    &mut self.gpioe.otyper as *mut u32,
                    read_volatile(&mut self.gpioe.otyper) & !(1 as u32) << led as usize,
                );
            }
            self.initialized[led as usize - 8] = true;
        }
    }

    /// Turns the LED on. If necessary, initializes the led.
    pub fn on(&mut self, led: CardinalPoints) {
        self.check_init(led);
        unsafe {
            write_volatile(
                &mut self.gpioe.odr as *mut u32,
                read_volatile(&mut self.gpioe.odr) | (0b1 as u32) << led as usize,
            );
        }
    }

    /// Turns the LED off.
    #[allow(dead_code)]
    pub fn off(&mut self, led: CardinalPoints) {
        unsafe {
            write_volatile(
                &mut self.gpioe.odr as *mut u32,
                read_volatile(&mut self.gpioe.odr) ^ (0b1 as u32) << led as usize,
            );
        }
    }

    /// Toggles the led. If necessary, initializes the led.
    pub fn toggle(&mut self, led: CardinalPoints) {
        self.check_init(led);
        let odr = unsafe { read_volatile(&mut self.gpioe.odr) };
        let on_bit = odr & (1 << led as usize);
        unsafe {
            write_volatile(
                &mut self.gpioe.odr as *mut u32,
                odr ^ (on_bit | 0b1) << led as usize,
            );
        }
    }
}
