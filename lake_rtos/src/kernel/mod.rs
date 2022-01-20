//! # Kernel
//!

use self::processes::Processes;

pub mod exceptions;
pub mod processes;

/// This [Option] of [Processes] is designed as singleton pattern.
static mut PROCESSES: Option<Processes> = None;

extern "C" {
    pub fn __context_switch(psp_next_addr: u32, psp_current_addr: u32);
    pub fn __breakpoint();
}
