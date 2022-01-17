//! # Kernel
//!

pub mod exceptions;
pub mod scheduler;

mod cs;

/// This static mutable of [ContextSwitch][cs::ContextSwitch] is useful because we can call this
/// within our exceptions and therefore make use of it easier in our assembler code.
static mut CONTEXT_SWITCH: cs::ContextSwitch = cs::ContextSwitch {
    psp_from_addr: 0,
    psp_next_addr: 0,
};

extern "C" {
    /// The context switch only works when called within an interrupt. e.g. [exceptions::PendSV]
    pub fn __context_switch(psp_next_addr: u32, psp_current_addr: u32);
    /// Sets a breakpoint in the running program.
    pub fn __breakpoint();
}
