use crate::dp::gpio::GPIO;
use crate::dp::uart::UART;

const USART1_TDR: u32 = 0x4001_3828;
const USART1_ISR: u32 = 0x4001_381C;
const CRLF: &str = "\r\n";

pub trait stdIo {
    fn print(&self);
    fn println(&self);
}

impl stdIo for &str {
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

// redundancy gets removed soon
impl stdIo for u32 {
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
    pub fn new(
        gpioa: &'static mut GPIO,
        /* uart: &'static mut UART, */ baudrate: u32,
    ) -> USART1 {
        USART1 {
            gpio: gpioa,
            uart: UART::new(),
            baudrate,
        }
    }

    pub fn init(&mut self) -> &mut USART1 {
        //af
        self.gpio.moder.set_bit(19);
        // pushpull
        self.gpio.otyper.clear_bit(9);
        // af7
        self.gpio.afrh.replace_bits(4, 7, 4);

        self.uart
            .brr
            .replace_bits(0, (8_000_000 / self.baudrate), 32);
        self.uart.cr1.set_bit(2);
        self.uart.cr1.set_bit(3);
        self.uart.cr1.set_bit(0);
        self
    }
}

fn transmit(c: u32) {
    unsafe {
        core::ptr::write_volatile(USART1_TDR as *mut u32, c);
        while !((core::ptr::read_volatile(USART1_ISR as *const u32) & 0x80) != 0) {}
    }
}
