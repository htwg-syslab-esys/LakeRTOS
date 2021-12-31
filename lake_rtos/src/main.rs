//! # LakeRTOS
//!
#![no_std]
#![no_main]

extern crate lake_rtos_rt;

mod cp;
mod dp;
mod driver;
mod util;

use cp::CorePeripherals;
use dp::{
    bus::{BusInterface, AHB1},
    gpio::GPIO,
    rcc::RCC,
    DevicePeripherals,
};
use driver::leds::{CardinalPoints::*, LEDs};

/// LEDs hook for exceptions
static mut LEDS: Option<LEDs> = None;

/// Kernel main
#[no_mangle]
fn kmain() -> ! {
    let bus: BusInterface = DevicePeripherals::take();

    let mut ahb1: AHB1 = bus.ahb1();
    ahb1.rcc(|rcc: &mut RCC| rcc.iopeen());

    let gpioe: &mut GPIO = bus.ahb2().gpioe();
    let mut leds: LEDs = LEDs::new(gpioe);

    leds.on(South);

    let cp = CorePeripherals::take().unwrap();
    cp.stk
        .set_reload(0x3FFFF)
        .clear_val()
        .tickint(true)
        .enable();

    unsafe { LEDS = Some(leds) };

    loop {}
}

/// # SysTick exception
///
/// This function will be called when the SysTick exception is triggered.
#[no_mangle]
pub unsafe extern "C" fn SysTick() {
    match &mut LEDS {
        Some(leds) => leds.toggle(South),
        None => {}
    }
}
