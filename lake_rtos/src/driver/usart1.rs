use crate::dp::gpio::GPIO;
use crate::dp::uart::UART;

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
pub fn print_str(msg: &str) {
    let usart1_tdr = 0x4001_3800 | 0x28;
    let usart1_isr = 0x4001_3800 | 0x1C;

    for c in msg.chars() {
        unsafe {
            core::ptr::write_volatile(usart1_tdr as *mut u32, c as u32);
            while !((core::ptr::read_volatile(usart1_isr as *const u32) & 0x80) != 0) {}
        }
    }
}
