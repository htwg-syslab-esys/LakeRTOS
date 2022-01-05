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
/// Static mutable reference hook for Processes (for demonstration purposes)
static mut PROCESSES: Option<&mut Processes> = None;

const LED_DEMO_CLOSURE: fn(usize, led: fn(&mut LEDs)) -> ! =
    |pid_next: usize, led: fn(&mut LEDs)| unsafe {
        loop {
            if COUNTDOWN_FLAG {
                if let Some(leds) = &mut LEDS {
                    led(leds)
                }
                COUNTDOWN_FLAG = false;
                if let Some(processes) = &mut PROCESSES {
                    processes.switch_to_pid(pid_next).unwrap();
                }
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

        if let Ok(process_option) = Processes::take() {
            if let Some(processes) = process_option {
                PROCESSES = Some(processes);
            }
        }
    };

    if let Some(p) = unsafe { &mut PROCESSES } {
        p.create(user_task_led_on).unwrap();
        p.create(user_task_led_off).unwrap();
        p.switch_to_pid(0).unwrap();
    }

    loop {}
}
