//! # Policies
//!
use super::{Scheduler, ALLOWED_PROCESSES, SCHEDULER};

use core::ptr;

#[derive(Debug)]
pub enum SchedulerPolicy {
    RoundRobin,
}

#[derive(Debug)]
pub struct Policy {
    scheduler: &'static mut Scheduler,
}

impl Policy {
    pub fn init() -> Policy {
        let scheduler = unsafe { SCHEDULER.as_mut().unwrap() };
        let pid0 = scheduler.processes.get_mut(0).unwrap().as_mut().unwrap();
        unsafe {
            super::super::CONTEXT_SWITCH.psp_from_addr = ptr::addr_of!(pid0.psp) as u32;
        }
        Policy { scheduler }
    }

    pub fn schedule(self) -> ! {
        match self.scheduler.policy {
            SchedulerPolicy::RoundRobin => {
                let mut cycle = (1..ALLOWED_PROCESSES).cycle();
                loop {
                    if let Some(pid) = cycle.next() {
                        if let Ok(()) = self.scheduler.switch_to_pid(pid) {}
                    }
                }
            }
        }
    }
}
