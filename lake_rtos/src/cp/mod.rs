//! # Core Peripherals
mod stk;

use self::stk::SystemTimer;

const SYSTICK_TIMER: u32 = 0xE000_E010;

/// Boolean flag for singleton pattern.
static mut TAKEN: bool = false;

pub struct CorePeripherals {
    pub stk: SystemTimer,
}

impl CorePeripherals {
    pub fn take() -> Option<CorePeripherals> {
        if unsafe { TAKEN } {
            None
        } else {
            Some(unsafe { CorePeripherals::steal() })
        }
    }

    pub unsafe fn steal() -> Self {
        TAKEN = true;

        CorePeripherals {
            stk: SystemTimer::init(),
        }
    }
}
