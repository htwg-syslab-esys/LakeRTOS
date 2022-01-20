//! # LEDs
//!
//! Convenient wrapper for the LEDs. Supported by the discovery board.
//!
//! ## Configuration
//!
//! GPIO pin must be correctly configured for the LED to work
//!
//! ### *GPIO port mode register*
//! - 00: Input mode (reset state)
//! - **01: General purpose output mode**
//! - 10: Alternate function mode
//! - 11: Analog mode
//!
//! `General purpose output mode` is required.
//!
//! ### *GPIO port output type register*
//! - **0: Output push-pull (reset state)**
//! - 1: Output open-drain
//!
//! If the pins have not been altered after reset we would not need
//! to set the register. But it could have been changed, therefore we
//! set it to `output push-pull` to be safe.
//!
//! ### *GPIO port output data register*
//! - 0: LED is off
//! - 1: LED is on
//!
//! Reset value is 0
//!
//! ## More Information
//!
//! [Reference Manual](https://www.st.com/resource/en/reference_manual/dm00043574-stm32f303xb-c-d-e-stm32f303x6-8-stm32f328x8-stm32f358xc-stm32f398xe-advanced-arm-based-mcus-stmicroelectronics.pdf)
//! GPIO registers - Section 11.4
use core::slice::Iter;

use self::CardinalPoints::*;
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

impl CardinalPoints {
    pub fn iterator() -> Iter<'static, CardinalPoints> {
        static DIRECTIONS: [CardinalPoints; 8] = [
            NorthWest, North, NorthEast, East, SouthEast, South, SouthWest, West,
        ];
        DIRECTIONS.iter()
    }
}

/// Needs [GPIO] port E.
///
/// Consists of eight LEDs in an cardinal points layout. They can be accessed with enum type [CardinalPoints].
pub struct LEDs {
    gpio: &'static mut GPIO,
    initialized: [bool; 8],
}

#[allow(dead_code)]
impl LEDs {
    pub fn new(gpioe: &'static mut GPIO) -> LEDs {
        LEDs {
            gpio: gpioe,
            initialized: [false; 8],
        }
    }

    /// Checks if the passed LED is initialized. If not, the
    /// gpio initialization steps get done described in the main
    /// documentation of this module.
    pub fn check_init(&mut self, led: CardinalPoints) {
        if !self.initialized[led as usize - 8] {
            self.gpio.moder.replace_bits(led as u32 * 2, 0b01, 2);
            self.gpio.otyper.clear_bit(led as u32);
            self.initialized[led as usize - 8] = true;
        }
    }

    /// Turns the LED on. If necessary, initializes the led.
    pub fn on(&mut self, led: CardinalPoints) -> &mut LEDs {
        self.check_init(led);
        self.gpio.odr.set_bit(led as u32);
        self
    }

    /// Turns the LED off.
    pub fn off(&mut self, led: CardinalPoints) -> &mut LEDs {
        self.gpio.odr.clear_bit(led as u32);
        self
    }

    pub fn all_off(&mut self) -> &mut LEDs {
        CardinalPoints::iterator().for_each(|led| {
            self.off(*led);
        });
        self
    }

    /// Toggles the led. If necessary, initializes the led.
    pub fn toggle(&mut self, led: CardinalPoints) -> &mut LEDs {
        self.check_init(led);
        self.gpio.odr.flip_bits(led as u32, 1);
        self
    }
}