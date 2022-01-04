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
use kernel::{processes::Processes, PROCESS_OFFSET_TABLE};

/// LEDs hook for exceptions
static mut LEDS: Option<LEDs> = None;
/// Boolean flag for countdown timer
static mut COUNTDOWN_FINISHED: bool = false;

const LED_DEMO_CLOSURE: fn(usize) -> ! = |pid_next: usize| loop {
    unsafe {
        if COUNTDOWN_FINISHED {
            match &mut LEDS {
                Some(leds) => leds.toggle(North),
                None => {}
            }
            COUNTDOWN_FINISHED = false;
            if let Some(processes) = &mut PROCESS_OFFSET_TABLE {
                processes.switch_to_pid(pid_next);
            }
        }
    }
};

fn user_task_led_north() -> ! {
    LED_DEMO_CLOSURE(1)
}

fn user_task_led_east() -> ! {
    LED_DEMO_CLOSURE(0)
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
        PROCESS_OFFSET_TABLE = Some(Processes::init());
    };

    if let Some(processes) = unsafe { &mut PROCESS_OFFSET_TABLE } {
        processes.create(user_task_led_north).unwrap();
        processes.create(user_task_led_east).unwrap();
        processes.switch_to_pid(0);
    }

    loop {}
}
