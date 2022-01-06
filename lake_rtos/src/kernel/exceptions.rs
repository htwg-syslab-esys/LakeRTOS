//! # Exceptions

use crate::util::register::Register;

use super::SCHEDULER;

/// # SysTick exception
///
/// This function will be called when the SysTick exception is triggered.
#[no_mangle]
pub unsafe extern "C" fn SysTick() {
    trigger_PendSV();
}

// Set PendSV to pending
// Interrupt control and state register (ICSR)  0xE000ED04
#[no_mangle]
#[allow(non_snake_case)]
pub fn trigger_PendSV() {
    let icsr: &mut Register = unsafe { &mut *(0xE000_ED04 as *mut Register) };
    icsr.set_bit(28);
}

#[no_mangle]
pub unsafe extern "C" fn PendSV() {
    let scheduler = SCHEDULER.as_mut().unwrap();
    scheduler.schedule();
}
