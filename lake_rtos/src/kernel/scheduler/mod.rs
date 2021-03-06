//! # Scheduler
//!
//! Processes will be placed top of SRAM
//!
//! ```text
//!     Memory model
//!   |     ...      |  
//!   |  Peripheral  |
//!   |--------------| 0x4000_0000
//!   |     ...      |
//!   |--------------| 0x2000 9C40 (upper limit for our discovery board version)
//!   |              | <<<< msp <<<< | start main stack |
//!   |              |
//! | |              |
//! | |              | <<<< psp <<<< | Process stack 0 (pid0) | 0x2000_6000
//! | |     SRAM     |               | Process stack 1 (pid1) | 0x2000_5000
//! v*|              |               | Process stack 2 (pid2) | 0x2000_4000
//!   |              |               | Process stack 3 (pid3) | 0x2000_3000
//!   |              |               | ...
//!   |              |
//!   |              | <<<<<<<<<<<<< | static variables |
//!   |--------------| 0x2000_0000
//!   |--------------| 0x1FFF_FFFF
//!   |     Code     |
//!   |     ...      |
//!
//! *full descending stack
//!
//! psp = process stack pointer
//! msp = main stack pointer
//! ```
//!
//! A process has as much memory available, as defined in [PROCESS_MEMORY_SIZE].

pub mod policies;

use crate::{
    cp::stk::SystemTimer,
    kernel::{
        exceptions::trigger_PendSV,
        scheduler::policies::{Policy, SchedulerPolicy},
    },
    util::register::Register,
};
use core::ptr;

use super::cs::CONTEXT_SWITCH;

/// Maximum allowed processes
const ALLOWED_PROCESSES: usize = 5;
/// Starting address of processes (processes are stacked descending)
const PROCESS_BASE: u32 = 0x2000_6000;
/// The reserved memory for a process. This does not protect against memory overflow.
const PROCESS_MEMORY_SIZE: u32 = 0x1000;

/// This [Option] holds a reference to the [Scheduler].
pub(super) static mut SCHEDULER_REF: Option<&mut Scheduler> = None;
/// This allows for the singleton pattern.
static mut SCHEDULER_TAKEN: bool = false;

#[derive(Debug)]
pub enum SchedulerError {
    /// Process stack is completely occupied.
    ProcessStackFull,
    /// Process was not initialized.
    NotInitialized,
    /// Process is not available. (Index out of Bounds)
    NotAvailable,
    /// Process is already running
    AlreadyRunning,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
}

/// This is process 0 (pid0). It is not intended to be called directly, but is
/// initiated as a process by the [Scheduler] itself.
fn scheduler_task() -> ! {
    let policy = Policy::init(unsafe { SCHEDULER_REF.as_mut().unwrap() });
    policy.schedule()
}

/// The scheduler is responsible to create processes and initiate scheduling.
///
/// It holds the [PCB][ProcessControlBlock] of each process, as well as the selected
/// [SchedulerPolicy]. It needs the system timer, otherwise safe operations can not be
/// guaranteed.
#[derive(Debug)]
pub struct Scheduler {
    processes: [Option<ProcessControlBlock>; ALLOWED_PROCESSES],
    policy: SchedulerPolicy,
    current_pid: Option<usize>,
    system_timer: SystemTimer,
}

impl Scheduler {
    /// Only the first call will return an reference to Some([Scheduler])
    pub fn init(system_timer: SystemTimer, policy: SchedulerPolicy) -> Option<Scheduler> {
        if unsafe { SCHEDULER_TAKEN } {
            None
        } else {
            unsafe {
                SCHEDULER_TAKEN = true;
            }

            let mut scheduler = Scheduler {
                processes: [None; ALLOWED_PROCESSES],
                policy,
                current_pid: None,
                system_timer,
            };

            scheduler.create_process(scheduler_task).unwrap();

            Some(scheduler)
        }
    }

    /// Disables the system times.
    ///
    /// Furthermore, it also clears a possible pending flag for the SysTick exception triggered
    /// by the system timer. For this to be effective, this function needs to be called within
    /// an exception, as for example in the supervisor call.
    pub(super) fn disable_timed_context_switch(&mut self) {
        self.system_timer.disable();
        let icsr: &mut Register = unsafe { &mut *(0xE000_ED04 as *mut Register) };
        icsr.set_bit(25);
    }

    /// This function will start the scheduling of the created processes. First task will
    /// be [scheduler_task] also known as pid0.
    ///
    /// Additionally the pointer to the scheduler will be stored in a mutable static to be
    /// referenced in the [scheduler_task] to initiate [Policy].
    pub fn start_scheduling(&mut self) -> ! {
        unsafe { SCHEDULER_REF = Some(&mut *(self as *mut Scheduler)) };
        if let Ok(()) = self.prepare_switch_to_pid(0) {
            trigger_PendSV();
        }
        // unreachable
        loop {}
    }

    /// All process will be created relative to [PROCESS_BASE] and are distant by [PROCESS_MEMORY_SIZE].
    ///
    /// # Arguments
    ///
    /// * A process that is defined as a function with no parameters that does not return.
    ///
    /// # Returns
    ///
    /// * [Ok] creation of process was successful.
    /// * [Err] with an [SchedulerError].
    pub fn create_process(&mut self, init_fn: fn() -> !) -> Result<usize, SchedulerError> {
        if let Some((pid, empty_slot)) = self
            .processes
            .iter_mut()
            .enumerate()
            .find(|(_, process_frame)| process_frame.is_none())
        {
            let init_stack_frame = unsafe {
                &mut *((PROCESS_BASE - (pid as u32 * PROCESS_MEMORY_SIZE))
                    as *mut InitialStackFrame)
            };

            *init_stack_frame = InitialStackFrame {
                load_stack: LoadStackFrame::default(),
                exception_stack: ExceptionFrame::default(init_fn),
            };

            let auto_stack_addr = ptr::addr_of_mut!(init_stack_frame.exception_stack.r0);

            *empty_slot = Some(ProcessControlBlock::init(
                pid,
                auto_stack_addr as u32,
                ProcessState::Ready,
            ));

            Ok(pid)
        } else {
            Err(SchedulerError::ProcessStackFull)
        }
    }

    /// Prepares the [ContextSwitch][super::cs::ContextSwitch]. Returning [Ok] allows for enabling PendSV
    /// to switch to the prepared process.
    ///
    /// # Arguments
    ///
    /// * [usize] process id (pid)
    ///
    /// # Returns
    ///
    /// * [Ok] when context switch is prepared.
    /// * [SchedulerError] when preparing context switch failed.
    fn prepare_switch_to_pid(&mut self, pid: usize) -> Result<(), SchedulerError> {
        let next_process = match self.processes.get_mut(pid) {
            Some(process) => process,
            None => return Err(SchedulerError::NotAvailable),
        };

        let psp_next_addr = match next_process {
            Some(next_pcb) => match next_pcb.state {
                ProcessState::Ready => {
                    next_pcb.state = ProcessState::Running;
                    ptr::addr_of_mut!(next_pcb.psp) as u32
                }
                ProcessState::Running => return Err(SchedulerError::AlreadyRunning),
            },
            None => return Err(SchedulerError::NotInitialized),
        };

        unsafe {
            CONTEXT_SWITCH.set_next_addr(psp_next_addr);
        }

        if let Some(current_pid) = self.current_pid {
            if let Some(current_pcb) = self.processes.get_mut(current_pid).unwrap() {
                current_pcb.state = ProcessState::Ready;
            }
        }
        self.current_pid = Some(pid);

        Ok(())
    }
}

/// Every process has an [PCB][ProcessControlBlock].
///
/// It holds the saved process stack pointer (psp), as well as the program id (pid).
/// Furthermore it saves the [ProcessState].
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProcessControlBlock {
    psp: u32,
    pid: usize,
    state: ProcessState,
}

impl ProcessControlBlock {
    pub fn init(pid: usize, psp: u32, state: ProcessState) -> ProcessControlBlock {
        ProcessControlBlock { pid, psp, state }
    }
}

/// The first context switch to a new process will point to this initial stack frame.
///
/// There, the load_stack will be the first loaded manually and subsequently the built-in
/// automatic loading of the auto_stack will be done by the processor when existing an
/// exception.
#[repr(C)]
pub struct InitialStackFrame {
    load_stack: LoadStackFrame,
    exception_stack: ExceptionFrame,
}

/// Will be initially loaded when the first context switch occurs.
///
/// It needs to have an align_buffer to be placed correctly on top
/// of the 8 byte aligned [ExceptionFrame].
#[repr(C)]
pub struct LoadStackFrame {
    _align_buffer: [u32; 7],
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    lr: u32,
}

impl LoadStackFrame {
    fn default() -> LoadStackFrame {
        LoadStackFrame {
            _align_buffer: [0; 7],
            r4: 0x3,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            lr: 0xfffffffd,
        }
    }
}

/// It will be automatically read the first time a context switch occurs at the
/// corresponding process.
///
/// It needs to be 8 bytes aligned. Initially, the PC will hold the reference to
/// the function of the process task.
///
/// *NOTE: This will be only needed to be handled once, after that the processor will
/// automatically create an auto stack frame each time an exception occurs.*
#[repr(C, align(8))]
pub struct ExceptionFrame {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,
}

impl ExceptionFrame {
    fn default(init_fn: fn() -> !) -> ExceptionFrame {
        let init_fn = ptr::addr_of!(init_fn);
        ExceptionFrame {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r12: 0,
            lr: 0,
            pc: unsafe { init_fn.read_unaligned() as *const () as u32 },
            xpsr: 0x1000000,
        }
    }
}
