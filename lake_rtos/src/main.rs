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
use kernel::{
    scheduler::{policies::SchedulerPolicy::RoundRobin, Scheduler},
    syscall,
    SvcRequest::*,
    SvcResult::*,
};

#[cfg(feature = "semihosting")]
use kernel::sprint;

/// pid1
fn user_task_pid_1() -> ! {
    let mut counter: u32 = 0;

    loop {
        counter += 1;
        #[cfg(feature = "semihosting")]
        {
            let display = [
                'p' as u8,
                'i' as u8,
                'd' as u8,
                ' ' as u8,
                '1' as u8,
                ' ' as u8,
                ((counter / 1000) % 10 + 48) as u8,
                ((counter / 100) % 10 + 48) as u8,
                ((counter / 10) % 10 + 48) as u8,
                ((counter / 1) % 10 + 48) as u8,
                '\n' as u8,
                '\0' as u8,
            ];

            syscall(SemihostingWrite0(display.as_ptr() as *const u8));
        }
        syscall(Yield);
    }
}

/// pid2
fn user_task_pid_2() -> ! {
    let bus: BusInterface = DevicePeripherals::take();

    let mut ahb1: AHB1 = bus.ahb1();
    ahb1.rcc(|rcc: &mut RCC| rcc.iopeen());

    let gpioe: &mut GPIO = bus.ahb2().gpioe();
    let mut leds: LEDs = LEDs::new(gpioe);

    loop {
        #[cfg(feature = "semihosting")]
        {
            let user_input = syscall(SemihostingReadC);
            if let Char(dir) = user_input {
                match dir.to_ascii_lowercase() as char {
                    // Hitting enter is just another input character. Here we skip it.
                    '\n' => continue,
                    'n' => {
                        sprint("pid 2 LED North on\n");
                        leds.on(North)
                    }
                    'w' => {
                        sprint("pid 2 LED West on\n");
                        leds.on(West)
                    }
                    'e' => {
                        sprint("pid 2 LED East on\n");
                        leds.on(East)
                    }
                    's' => {
                        sprint("pid 2 LED South on\n");
                        leds.on(South)
                    }
                    _ => {
                        sprint("pid 2 LED all off\n");
                        leds.all_off()
                    }
                };
            }
        }
        syscall(Yield);
    }
}

/// Kernel main
#[no_mangle]
fn kmain() -> ! {
    let mut cp = CorePeripherals::take().unwrap();
    let system_timer = cp.take_system_timer().unwrap();

    let mut p = Scheduler::init(system_timer, RoundRobin(Some(0x1F40))).unwrap();
    p.create_process(user_task_pid_1).unwrap();
    p.create_process(user_task_pid_2).unwrap();
    p.start_scheduling()
}
