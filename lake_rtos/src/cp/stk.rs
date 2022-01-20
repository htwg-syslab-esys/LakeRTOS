//! # SysTick timer (STK)
//!
//! [Programming Manual](https://www.st.com/content/ccc/resource/technical/document/programming_manual/6c/3a/cb/e7/e4/ea/44/9b/DM00046982.pdf/files/DM00046982.pdf/jcr:content/translations/en.DM00046982.pdf)
//! Section 4.5 - p.246
use crate::util::register::Register;

use super::SYSTICK_TIMER;

/// Reload value maximum
pub const STK_RELOAD_MAX: u32 = 0x00FFFFFF;

/// System Timers registers
#[repr(C)]
#[derive(Debug)]
struct Systick {
    /// Control and status register (RW)
    stk_ctrl: Register,
    /// Reload value register (RW)
    stk_load: Register,
    /// Current value register (RW)
    stk_val: Register,
    /// Calibration value register (RO)
    stk_calib: Register,
}

/// System Timer
///
/// Programming Manual Section 4.5
///
/// *Manual hints the correct sequence*
/// 1. Program reload value.
/// 2. Clear current value.
/// 3. Program Control and Status register.
#[derive(Debug)]
pub struct SystemTimer {
    p: &'static mut Systick,
}

impl SystemTimer {
    pub(super) fn init() -> SystemTimer {
        SystemTimer {
            p: unsafe { &mut *(SYSTICK_TIMER as *mut Systick) },
        }
    }

    /// Sets the reload value
    ///
    /// Reload value can be any value in the range ```0x00000001-0x00FFFFFF```.
    /// On each clock cycle the value gets incremented by one.
    /// The effective amount of time may calculated like this:
    ///
    /// time_cycle = 1 / clock
    ///  
    /// effective_time = register_value * time_cycle
    ///
    /// *Example*
    /// time_cycle = 1 / 8000000 = 125 (ns)
    ///
    /// effective_time = 0x3AB * time_cycle = 939 * 125 (ns) = 117375 (ns)
    ///
    /// -> Result: Every 117,375 (usec) a interrupt gets fired.
    ///
    /// # Arguments
    ///
    /// * `load` - A u32 which represents the count of systicks until
    /// a interrupt gets fired.
    ///
    /// # Returns
    /// * `None`
    ///
    pub fn set_reload(&mut self, load: u32) -> &mut SystemTimer {
        if load <= STK_RELOAD_MAX {
            self.p.stk_load.replace_bits(0, load, 31);
        }
        self
    }

    /// Any write to the register will clear the field to 0 and sets the COUNTFLAG
    /// in STK_CTRL register to 0.
    pub fn clear_val(&mut self) -> &mut SystemTimer {
        self.p.stk_val.set_bit(0);
        self
    }

    /// SysTick exception request enable.
    /// Setting bit to *1* requests the SysTick Interrupt when the STK_LOAD Register
    /// reaches 0.
    pub fn tickint(&mut self, enable: bool) -> &mut SystemTimer {
        self.p.stk_ctrl.replace_bits(1, enable as u32, 1);
        self
    }

    /// Enables the counter by setting `Bit 0 ENABLE: Counter enable`
    pub fn enable(&mut self) -> &mut SystemTimer {
        self.p.stk_ctrl.set_bit(0);
        self
    }

    /// Disables the counter.
    #[allow(dead_code)]
    pub fn disable(&mut self) -> &mut SystemTimer {
        self.p.stk_ctrl.clear_bit(0);
        self
    }
}