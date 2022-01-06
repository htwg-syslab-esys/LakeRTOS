//! # Kernel
//!

use self::scheduler::Scheduler;

pub mod exceptions;
pub mod scheduler;

/// Starting address of processes (processes are stacked descending)
const PROCESS_BASE: u32 = 0x2000_8000;

/// This [Option] of [Processes] is designed as singleton pattern.
static mut SCHEDULER: Option<Scheduler> = None;

extern "C" {
    pub fn __context_switch(psp_next_addr: u32, psp_current_addr: u32);
    pub fn __breakpoint();
}
