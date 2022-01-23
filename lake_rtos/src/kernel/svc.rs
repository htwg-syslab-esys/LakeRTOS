//! Supervisor Call (System Calls)
//!

use super::__syscall;

/// Maximum length of text being written to the console. Last 
/// character will be overwritten to be null-terminated.
#[cfg(feature = "semihosting")]
const SEMIHOSTING_WRITE_LENGTH: usize = 64;

/// Systemcalls requests.
#[allow(dead_code)]
#[repr(C)]
pub enum SvcRequest {
    /// Writes null-terminated array of characters to console.
    #[cfg(feature = "semihosting")]
    SemihostingWrite0(*const u8),
    /// Writes character to console.
    #[cfg(feature = "semihosting")]
    SemihostingWriteC(*const u8),
    /// Reads character from console.
    #[cfg(feature = "semihosting")]
    SemihostingReadC,
    /// Yields process. Returns to scheduler.
    Yield,
}

/// A system call will write the result as an [SvcResult] variant.
#[repr(C)]
pub enum SvcResult {
    None,
    Char(u8),
}

/// The [SvcOrder] is a helper struct for system calls. The order itself is
/// allocated on the calling process stack. The SVCall exceptions then uses
/// a pointer to evaluate the request and possible write any result back to
/// the order.
#[repr(C)]
pub struct SvcOrder {
    pub request: SvcRequest,
    pub response: SvcResult,
}

/// Will trigger a system call. After the system call was executed, the
/// result will be in [SvcResult].
pub fn syscall(request: SvcRequest) -> SvcResult {
    let order = SvcOrder {
        request,
        response: SvcResult::None,
    };

    unsafe {
        __syscall(&order);
    };
    order.response
}

/// Convenient method for printing text on the console. Be aware that the
/// length of the text is restricted by [SEMIHOSTING_WRITE_LENGTH].
#[cfg(feature = "semihosting")]
pub fn sprint(text: &str) {
    let mut whole = [0; SEMIHOSTING_WRITE_LENGTH];
    for (index, empty_char) in whole.iter_mut().enumerate() {
        if index == SEMIHOSTING_WRITE_LENGTH - 1 {
            *empty_char = '\0' as u8;
        }
        match text.as_bytes().get(index) {
            Some(char) => *empty_char = *char,
            None => {
                *empty_char = '\0' as u8;
                break;
            }
        }
    }
    syscall(SvcRequest::SemihostingWrite0(whole.as_ptr() as *const u8));
}
