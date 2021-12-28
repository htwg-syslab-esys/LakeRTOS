#![no_std]
#![no_main]

extern crate lake_rtos_rt;

mod cp;
mod dp;
mod led;

use dp::bus::{PERIPHERALS, Serial, AHB1, GPIO, RCC};
use led::LED;

static mut LED: Option<led::LED> = None;

#[no_mangle]
fn kmain() -> ! {
    let serial: Serial = unsafe { PERIPHERALS.take_serial() };

    let mut ahb1: AHB1 = serial.ahb1();
    ahb1.rcc(|rcc: &mut RCC| rcc.iopeen());

    let gpioe: &mut GPIO = serial.ahb2().gpioe();
    let mut leds: LED = led::LED::new(gpioe);

    leds.on(13);

    let st = cp::stk::SystemTimer::take();
    st.set_reload(0x3FFFF).enable();

    unsafe { LED = Some(leds) };

    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn SysTick() {
    match &mut LED {
        Some(leds) => leds.toggle(13),
        None => {}
    }
}
