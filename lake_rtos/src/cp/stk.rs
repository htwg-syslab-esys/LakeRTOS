//! # SysTick timer (STK)
//!
//! [Programming Manual](https://www.st.com/content/ccc/resource/technical/document/programming_manual/6c/3a/cb/e7/e4/ea/44/9b/DM00046982.pdf/files/DM00046982.pdf/jcr:content/translations/en.DM00046982.pdf)
//! Section 4.5 - p.246
use super::SYSTICK_TIMER;
use core::ptr::{read_volatile, write_volatile};

/// System Timers registers
#[repr(C)]
struct Systick {
    /// Control and status register (RW)
    stk_ctrl: u32,
    /// Reload value register (RW)
    stk_load: u32,
    /// Current value register (RW)
    stk_val: u32,
    /// Calibration value register (RO)
    stk_calib: u32,
}

/// System Timer
///
/// Programming Manual Section 4.5
///
/// *Manual hints the correct sequence*
/// 1. Program reload value.
/// 2. Clear current value.
/// 3. Program Control and Status register.
pub struct SystemTimer {
    p: &'static mut Systick,
}

impl SystemTimer {
    pub fn init() -> SystemTimer {
        SystemTimer {
            p: unsafe { &mut *(SYSTICK_TIMER as *mut Systick) },
        }
    }

    /// Sets the reload value
    /// Reload value can be any value in the range 0x00000001-0x00FFFFFF.
    pub fn set_reload(self, load: u32) -> SystemTimer {
        if load <= 0x00FFFFFF {
            unsafe {
                write_volatile(
                    &mut self.p.stk_load as *mut u32,
                    read_volatile(&mut self.p.stk_load) | load,
                );
            }
        }
        self
    }

    /// Any write to the register will clear the field to 0 and sets the COUNTFLAG
    /// in STK_CTRL register to 0.
    pub fn clear_val(self) -> SystemTimer {
        unsafe {
            write_volatile(
                &mut self.p.stk_val as *mut u32,
                read_volatile(&mut self.p.stk_val) | 0b1,
            );
        }
        self
    }

    /// SysTick exception request enable
    pub fn tickint(self, enable: bool) -> SystemTimer {
        unsafe {
            write_volatile(
                &mut self.p.stk_ctrl as *mut u32,
                read_volatile(&mut self.p.stk_ctrl) | (enable as u32) << 1,
            );
        }
        self
    }

    /// Enables the counter
    pub fn enable(self) -> SystemTimer {
        unsafe {
            write_volatile(
                &mut self.p.stk_ctrl as *mut u32,
                read_volatile(&mut self.p.stk_ctrl) | 0b1,
            );
        }
        self
    }
}
