//! # LakeRTOS
//!
#![no_std]
#![no_main]

extern crate lake_rtos_rt;

mod cp;
mod dp;
mod driver;
mod kernel;
mod util;

use cp::CorePeripherals;
use dp::{
    bus::{BusInterface, AHB1},
    gpio::GPIO,
    rcc::RCC,
    DevicePeripherals,
};
use driver::leds::{CardinalPoints::*, LEDs};
use kernel::scheduler::Scheduler;

/// LEDs hook for exceptions
static mut LEDS: Option<LEDs> = None;

const LED_DEMO_CLOSURE: fn(led: fn(&mut LEDs)) -> ! = |led| unsafe {
    let leds = LEDS.as_mut().unwrap();
    loop {
        led(leds);
    }
};

fn user_task_led_on() -> ! {
    LED_DEMO_CLOSURE(|led| {
        led.on(North).on(South);
        led.off(West).off(East);
    })
}

fn user_task_led_off() -> ! {
    LED_DEMO_CLOSURE(|led| {
        led.on(West).on(East);
        led.off(North).off(South);
    })
}

/// Kernel main
#[no_mangle]
fn kmain() -> ! {
    let bus: BusInterface = DevicePeripherals::take();

    let mut ahb1: AHB1 = bus.ahb1();
    ahb1.rcc(|rcc: &mut RCC| rcc.iopeen());

    let gpioe: &mut GPIO = bus.ahb2().gpioe();
    let leds: LEDs = LEDs::new(gpioe);

    let cp = CorePeripherals::take().unwrap();
    cp.stk
        .set_reload(0x3FFFF)
        .clear_val()
        .tickint(true)
        .enable();

    unsafe {
        LEDS = Some(leds);
    };

    let p = Scheduler::take().unwrap();
    p.create_process(user_task_led_on).unwrap();
    p.create_process(user_task_led_off).unwrap();
    p.start_scheduling()
}
