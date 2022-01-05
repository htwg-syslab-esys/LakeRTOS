//! # Exceptions

use crate::COUNTDOWN_FLAG;

/// # SysTick exception
///
/// This function will be called when the SysTick exception is triggered.
#[no_mangle]
pub unsafe extern "C" fn SysTick() {
    COUNTDOWN_FLAG = true;
}
