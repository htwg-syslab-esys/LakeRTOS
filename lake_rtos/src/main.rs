#![no_std]
#![no_main]

use dp::bus::{Serial, AHB1, GPIO, PERIPHERALS, RCC};
use led::LED;

extern crate lake_rtos_rt;

mod dp;
mod led;

#[no_mangle]
fn main() -> ! {
    let serial: Serial = unsafe { PERIPHERALS.take_serial() };

    let mut ahb1: AHB1 = serial.ahb1();
    ahb1.rcc(|rcc: &mut RCC| rcc.iopeen());

    let gpioe: &mut GPIO = serial.ahb2().gpioe();
    let mut leds: LED = led::LED::new(gpioe);

    leds.on(13);

    loop {}
}
