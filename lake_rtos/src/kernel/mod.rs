//! # Kernel
//!

pub mod exceptions;
pub mod scheduler;

mod cs;

extern "C" {
    /// The context switch only works when called within an interrupt. e.g. [exceptions::PendSV]
    pub fn __context_switch(psp_next_addr: u32, psp_current_addr: u32);
    /// Sets a breakpoint in the running program.
    pub fn __breakpoint();
}
