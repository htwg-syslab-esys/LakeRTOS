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
use driver::leds::{
    CardinalPoints::{self, *},
    LEDs,
};
use kernel::threads::Processes;

/// LEDs hook for exceptions
static mut LEDS: Option<LEDs> = None;
/// Boolean flag for countdown timer
static mut COUNTDOWN_FINISHED: bool = false;

static mut PROCESS_OFFSET_TABLE: Option<Processes> = None;

const LED_DEMO_CLOSURE: fn(CardinalPoints) -> ! = |dir: CardinalPoints| loop {
    unsafe {
        if COUNTDOWN_FINISHED {
            match &mut LEDS {
                Some(leds) => leds.toggle(dir),
                None => {}
            }
            COUNTDOWN_FINISHED = false;
            if let Some(processes) = &mut PROCESS_OFFSET_TABLE {
                processes.switch_to_next_process();
            }
        }
    }
};

fn user_task_led_north() -> ! {
    LED_DEMO_CLOSURE(North)
}

fn user_task_led_east() -> ! {
    LED_DEMO_CLOSURE(East)
}

fn user_task_led_west() -> ! {
    LED_DEMO_CLOSURE(West)
}

fn user_task_led_south() -> ! {
    LED_DEMO_CLOSURE(South)
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
        processes.create_process(user_task_led_north).unwrap();
        processes.create_process(user_task_led_east).unwrap();
        processes.create_process(user_task_led_south).unwrap();
        processes.create_process(user_task_led_west).unwrap();

        processes.switch_to_next_process();
    }

    loop {}
}
