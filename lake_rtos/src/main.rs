//!
//! Main crate for Kernelimplementation. Depends on lake_rtos_rt
//! and the main function gets called after successful power on. 
//!
#![no_std]
#![no_main]

extern crate lake_rtos_rt;

///
/// Target function after hardware initialization, 
/// acts as the first kernel function.
/// 
#[no_mangle]
pub fn kmain() -> ! {
    loop {}
}