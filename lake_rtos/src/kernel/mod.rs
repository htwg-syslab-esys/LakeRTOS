//! # Kernel
//!

#[cfg(feature = "semihosting")]
pub use self::svc::sprint;
pub use self::svc::{syscall, SvcRequest, SvcResult};

pub mod scheduler;

mod cs;
mod exceptions;
mod svc;

extern "C" {
    /// The context switch only works when called within an interrupt. e.g. [exceptions::PendSV]
    fn __context_switch(psp_next_addr: u32, psp_from_addr: u32);
    /// Sets a breakpoint in the running program.
    fn __breakpoint();
    /// Triggers the supervisor call.
    ///
    /// # Argument
    ///
    /// * A pointer to an supervisor call order.
    fn __syscall(order: *const svc::SvcOrder);
    /// # ARM Semihosting SYS_WRITE0
    ///
    /// Writes a null-terminated string to the debug channel.
    ///
    /// # Argument
    ///
    /// * A pointer to an null-terminated u8 array.
    fn __sys_write0(text: *const u8);
    /// # ARM Semihosting SYS_WRITEC
    ///
    /// Writes a character byte, pointed to by R1, to the debug channel.
    ///
    /// # Argument
    ///
    /// * A pointer to an character
    fn __sys_writec(char: *const u8);
    /// # ARM Semihosting SYS_READC
    ///
    /// Reads a byte from the console.
    ///
    /// # Return
    ///
    /// * The byte read from the console.
    fn __sys_readc() -> u8;
    /// Returns value from register 0.
    fn __get_r0() -> *mut u32;
}
