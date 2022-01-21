//! # Kernel
//!

pub mod exceptions;
pub mod processes;

extern "C" {
    pub fn __context_switch(psp_next_addr: u32, psp_current_addr: u32);
    pub fn __breakpoint();
}
