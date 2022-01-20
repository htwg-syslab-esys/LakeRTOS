//! # Exceptions

use crate::util::register::Register;

use super::{
    __context_switch, __get_r0, cs::CONTEXT_SWITCH, scheduler::SCHEDULER_REF, svc::SvcOrder,
    SvcRequest, SvcResult,
};

#[cfg(feature = "semihosting")]
use super::{__sys_readc, __sys_write0, __sys_writec};

/// # SysTick exception
///
/// This function will be called when the SysTick exception is triggered.
#[no_mangle]
pub unsafe extern "C" fn SysTick() {
    trigger_PendSV();
}

/// Set PendSV to pending.
///
/// Interrupt control and state register (ICSR)  0xE000ED04
#[allow(non_snake_case)]
pub fn trigger_PendSV() {
    let icsr: &mut Register = unsafe { &mut *(0xE000_ED04 as *mut Register) };
    icsr.set_bit(28);
}

/// # PendSV exception
///
/// This exception has the lowest priority and therefore will be executed last when
/// there are nested exceptions.
#[no_mangle]
pub unsafe extern "C" fn PendSV() {
    let (psp_next_addr, psp_from_addr) = CONTEXT_SWITCH.get_addr_and_swap();
    __context_switch(psp_next_addr, psp_from_addr);
}


/// # SVCall exception
/// 
#[no_mangle]
pub extern "C" fn SVCall() {
    let mut order: &mut SvcOrder = unsafe { &mut *(__get_r0() as *mut SvcOrder) };
    match order.request {
        #[cfg(feature = "semihosting")]
        SvcRequest::SemihostingWrite0(text) => {
            unsafe { __sys_write0(text) }
            order.response = SvcResult::None;
        }
        #[cfg(feature = "semihosting")]
        SvcRequest::SemihostingWriteC(char) => {
            unsafe { __sys_writec(char) }
            order.response = SvcResult::None;
        }
        #[cfg(feature = "semihosting")]
        SvcRequest::SemihostingReadC => unsafe {
            order.response = SvcResult::Char(__sys_readc());
        },
        SvcRequest::Yield => {
            let scheduler = unsafe { SCHEDULER_REF.as_mut().unwrap() };
            scheduler.disable_timed_context_switch();

            #[cfg(feature = "semihosting")]
            unsafe {
                __sys_write0("yield\n\0".as_bytes().as_ptr() as *const u8)
            };

            trigger_PendSV();
        }
    }
}
