//! # Policies
//!
//! ## RoundRobin
//!
//! The figure below shows how the [RoundRobin][SchedulerPolicy::RoundRobin] policy is implemented. 
//! In this case between each user process task the scheduler task will be called to select the next
//! process to be scheduled.
//! 
//! ```text
//!  pid
//! curr  0      1      0      2      0      3      0
//! next  1      0      2      0      3      0      ...
//!
//!    0  *             *             *             *
//!    1         *       
//!    2                       *
//!    3                                     *
//! etc.
//!       m      t      m      t      m      t      m
//!       |______|______|______|______|______|______|_... -> time axis
//!
//!
//! PendSV trigger:
//! m = triggered manually
//! t = triggered from timer
//!
//! ```
use crate::{
    cp::stk::STK_RELOAD_MAX,
    kernel::{
        scheduler::{Scheduler, ALLOWED_PROCESSES},
        CONTEXT_SWITCH,
    },
};
use core::ptr;

/// Minimum switch rate in clock cycle, that ensures that the scheduler does not jump
/// back to early.
const SWITCH_RATE_CC_MIN: u32 = 0x50;

#[derive(Debug)]
pub enum SchedulerPolicy {
    /// RoundRobin with optional custom context switch rate in clock cycles.
    ///
    /// Must be between [SWITCH_RATE_CC_MIN] and [STK_RELOAD_MAX].
    /// Default is [SWITCH_RATE_CC_MIN].
    RoundRobin(Option<u32>),
}

#[derive(Debug)]
pub(super) struct Policy {
    scheduler: &'static mut Scheduler,
}

impl Policy {
    /// Excepts a pointer to the initialized Scheduler.
    pub fn init(scheduler: &'static mut Scheduler) -> Policy {
        let pid0 = scheduler.processes[0].as_mut().unwrap();
        unsafe {
            CONTEXT_SWITCH.set_from_addr(ptr::addr_of!(pid0.psp) as u32);
        }
        Policy { scheduler }
    }

    /// Does not return. Will execute the selected policy.
    pub fn schedule(self) -> ! {
        match self.scheduler.policy {
            SchedulerPolicy::RoundRobin(cc_switch_rate_custom) => {
                let mut reload_val = SWITCH_RATE_CC_MIN;
                if let Some(cc_switch_rate) = cc_switch_rate_custom {
                    if (SWITCH_RATE_CC_MIN..STK_RELOAD_MAX).contains(&cc_switch_rate) {
                        reload_val = cc_switch_rate;
                    }
                }

                let mut cycle = (1..ALLOWED_PROCESSES).cycle();

                self.scheduler
                    .system_timer
                    .set_reload(reload_val)
                    .clear_val()
                    .tickint(true)
                    .enable();

                loop {
                    if let Some(pid) = cycle.next() {
                        if let Ok(()) = self.scheduler.switch_to_pid(pid) {}
                    }
                }
            }
        }
    }
}
