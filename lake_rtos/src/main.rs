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
        leds.all_off();
    }
};

/// pid1
fn user_task_led_vertical() -> ! {
    LED_DEMO_CLOSURE(|led| {
        led.on(North).on(South);
    })
}

/// pid2
fn user_task_led_diagonally_right() -> ! {
    LED_DEMO_CLOSURE(|led| {
        led.on(NorthWest).on(SouthEast);
    })
}

/// pid3
fn user_task_led_horizontal() -> ! {
    LED_DEMO_CLOSURE(|led| {
        led.on(West).on(East);
    })
}

/// pid4
fn user_task_led_diagonally_left() -> ! {
    LED_DEMO_CLOSURE(|led| {
        led.on(NorthEast).on(SouthWest);
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

    let mut cp = CorePeripherals::take().unwrap();
    let system_timer = cp.take_system_timer().unwrap();

    unsafe {
        LEDS = Some(leds);
    };

    let mut p = Scheduler::init(system_timer).unwrap();
    p.create_process(user_task_led_vertical).unwrap();
    p.create_process(user_task_led_diagonally_right).unwrap();
    p.create_process(user_task_led_horizontal).unwrap();
    p.create_process(user_task_led_diagonally_left).unwrap();
    p.start_scheduling()
}
