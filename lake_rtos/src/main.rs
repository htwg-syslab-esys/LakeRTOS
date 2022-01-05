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
use kernel::processes::Processes;

/// LEDs hook for exceptions
static mut LEDS: Option<LEDs> = None;
/// Boolean flag for countdown timer
static mut COUNTDOWN_FLAG: bool = false;
/// Static mutable as hook to a mutable reference of [Processes] (for demonstration purposes)
static mut PROCESSES_HOOK: Option<&mut Processes> = None;

const LED_DEMO_CLOSURE: fn(usize, led: fn(&mut LEDs)) -> ! =
    |pid_next: usize, led: fn(&mut LEDs)| unsafe {
        loop {
            if COUNTDOWN_FLAG {
                led(LEDS.as_mut().unwrap());

                COUNTDOWN_FLAG = false;

                let p = PROCESSES_HOOK.as_mut().unwrap();
                p.switch_to_pid(pid_next).unwrap();
            }
        }
    };

fn user_task_led_on() -> ! {
    LED_DEMO_CLOSURE(1, |led| led.on(North))
}

fn user_task_led_off() -> ! {
    LED_DEMO_CLOSURE(0, |led| led.off(North))
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

    let p = Processes::take().unwrap();
    p.create_process(user_task_led_on).unwrap();
    p.create_process(user_task_led_off).unwrap();

    unsafe {
        PROCESSES_HOOK = Some(p);
        PROCESSES_HOOK.as_mut().unwrap().switch_to_pid(0).unwrap();
    }

    loop {}
}
