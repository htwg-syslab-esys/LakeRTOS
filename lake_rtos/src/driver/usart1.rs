//! # USART1
//!
//! This module offers a basic implementation of the user interface. 
//! When creating the object, the device is initialized according to the baud rate.
//! GPIO pin 9 is also will be configured according to the necessary specifications.
//! This class offers as a special feature an implementation of iostream, so that both string and integer values ​​
//! can be transmitted directly.
//! 
use crate::dp::gpio::GPIO;
use crate::dp::uart::UART;

const USART1_TDR: u32 = 0x4001_3828;
const USART1_ISR: u32 = 0x4001_381C;
const CRLF: &str = "\r\n";

pub trait iostream {
    fn print(&self);
    fn println(&self);
}

///
/// Appends print methods to &str primitive. The println method 
/// sends a newline character and a carriage return after the
/// payload has been succesful transmitted.
///
impl iostream for &str {
    fn print(&self) {
        for c in self.chars() {
            transmit(c as u32);
        }
    }
    fn println(&self) {
        for c in self.chars() {
            transmit(c as u32);
        }
        for c in CRLF.chars() {
            transmit(c as u32);
        }
    }
}

///
/// Appends print methods to u32 primitive. A numerical string has to be 
/// broken up into each digit before sending the payload byte-per-byte. 
///
impl iostream for u32 {
    fn print(&self) {
        let mut buffer: [u8; 32] = unsafe { core::mem::zeroed() };
        let mut cnt: u8 = 0;
        let mut dec = *self;
        while dec > 0 {
            buffer[cnt as usize] = (dec % 10 + 0x30) as u8;
            dec /= 10;
            cnt += 1;
        }
        for c in IntoIterator::into_iter(buffer).rev() {
            transmit(c as u32);
        }
    }
    fn println(&self) {
        let mut buffer: [u8; 32] = unsafe { core::mem::zeroed() };
        let mut cnt: u8 = 0;
        let mut dec = *self;
        while dec > 0 {
            buffer[cnt as usize] = (dec % 10 + 0x30) as u8;
            dec /= 10;
            cnt += 1;
        }
        for c in IntoIterator::into_iter(buffer).rev() {
            transmit(c as u32);
        }
        for c in CRLF.chars() {
            transmit(c as u32);
        }
    }
}

pub struct USART1 {
    gpio: &'static mut GPIO,
    uart: &'static mut UART,
    baudrate: u32,
}

impl USART1 {
    /// Returns a new USART Device
    ///
    /// # Arguments
    ///
    /// * `gpioa` - A static mutable reference to a gpio registerblock
    /// * `baudrate` - An u32 for desired baud rate
    ///
    /// # Returns
    /// * `USART1` - The USART1 Struct
    ///
    pub fn new(
        gpioa: &'static mut GPIO,
        baudrate: u32,
    ) -> USART1 {
        USART1 {
            gpio: gpioa,
            uart: UART::new(),
            baudrate,
        }
    }

    pub fn init(&mut self) -> &mut USART1 {
        // This operation turns pin 9 into alternate function mode
        self.gpio.moder.set_bit(19);
        // When using GPIO as transmit pin it has to be configured as Push-Pull output
        self.gpio.otyper.clear_bit(9);
        // The associated alternate function for pin- and usart-tx combination is af7
        self.gpio.afrh.replace_bits(4, 7, 4);

        // Basic configuration of usart. We calculate the baud rate according to the user's manual seection 29.5.4,
        // activate transmitter and receiver and finally activate the usart controller.
        // Note: these settings only can be set when the controller is disabled
        self.uart
            .brr
            .replace_bits(0, (8_000_000 / self.baudrate), 32);
        self.uart.cr1.set_bit(2);
        self.uart.cr1.set_bit(3);
        self.uart.cr1.set_bit(0);
        self
    }
}

///
/// Writes one byte into transmit register und polls interrupt flag until transmission in complete. Acts as 
/// as "blocking send".
///
/// # Arguments
///
/// * `c` - A bytee representing the char to be sent.
///
/// # Returns
/// * `Nothing`
///
fn transmit(c: u32) {
    unsafe {
        core::ptr::write_volatile(USART1_TDR as *mut u32, c);
        while !((core::ptr::read_volatile(USART1_ISR as *const u32) & 0x80) != 0) {}
    }
}
