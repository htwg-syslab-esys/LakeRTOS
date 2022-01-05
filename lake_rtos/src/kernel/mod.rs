//! # Kernel
//!

pub mod exceptions;
pub mod processes;

/// Starting address of processes (processes are stacked descending)
const PROCESS_BASE: u32 = 0x2000_8000;

extern "C" {
    pub fn __context_switch(psp_next_addr: u32, psp_current_addr: u32);
    pub fn __breakpoint();
}
