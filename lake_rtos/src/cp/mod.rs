//! # Core Peripherals
pub mod stk;

use core::mem::replace;

use self::stk::SystemTimer;

const SYSTICK_TIMER: u32 = 0xE000_E010;

/// Boolean flag for singleton pattern.
static mut TAKEN: bool = false;

/// Contains the core peripherals. Unlike device peripherals there is no bus interface.
pub struct CorePeripherals {
    stk: Option<SystemTimer>,
}

impl CorePeripherals {
    pub fn take() -> Option<CorePeripherals> {
        if unsafe { TAKEN } {
            None
        } else {
            Some(unsafe { CorePeripherals::steal() })
        }
    }

    unsafe fn steal() -> Self {
        TAKEN = true;

        CorePeripherals {
            stk: Some(SystemTimer::init()),
        }
    }

    /// Singleton pattern
    pub fn take_system_timer(&mut self) -> Option<SystemTimer> {
        if let Some(_) = self.stk {
            replace(&mut self.stk, None)
        } else {
            None
        }
    }
}
