//! # Exceptions

use crate::util::register::Register;

use super::{__context_switch, CONTEXT_SWITCH};

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
    let (psp_next_addr, psp_current_addr) = CONTEXT_SWITCH.prepare_context_switch();
    __context_switch(psp_next_addr, psp_current_addr);
}
