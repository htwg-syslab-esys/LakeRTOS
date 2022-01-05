//! # Processes
//!
//! Processes will be placed top of SRAM
//!
//! Memory layout
//!
//! ```text
//! |     ...      |  
//! |  Peripheral  |
//! |--------------| 0x4000_0000  
//! |--------------| 0x2000 9C40 (upper limit for our discovery board version)
//! |              | <<<<<<<<<<<<< | Process1 (psp1) | 0x2000_8000 - 0x2000_8FFF
//! |              |               | Process2 (psp2) | 0x2000_7000 - 0x2000_7FFF
//! |              |               | Process3 (psp3) | 0x2000_6000 - 0x2000_6FFF
//! |     SRAM     |               | Process4 (psp4) | 0x2000_5000 - 0x2000_5FFF
//! |              |
//! |              |
//! |              | <<<<<<<<<<<<< | Kernel stack (msp) |
//! |--------------| 0x2000_0000
//! |--------------| 0x1FFF_FFFF
//! |     Code     |
//! |     ...      |
//! ```
//!
//! A process has 4K memory available.

use core::ptr::{self};

use super::{__context_switch, PROCESS_BASE};

/// Maximum allowed processes
const ALLOWED_PROCESSES: usize = 4;
/// The reserved memory for a process. This does not protect against memory overflow.
const PROCESS_MEMORY_SIZE: u32 = 0x1000;

#[derive(Debug)]
pub enum ProcessesError {
    /// Process stack is completely occupied.
    ProcessStackFull,
    /// Process was not initialized.
    NotInitialized,
    /// Process is not available. (Index out of Bounds)
    ProcessNotAvailable,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessState {
    Uninitialized,
    Initialized(ProcessFrame),
}

#[derive(Debug)]
pub struct Processes {
    processes: [ProcessState; ALLOWED_PROCESSES],
    current_process_idx: Option<usize>,
}

impl Processes {
    pub fn init() -> Processes {
        Processes {
            processes: [ProcessState::Uninitialized; ALLOWED_PROCESSES],
            current_process_idx: None,
        }
    }

    pub fn create(&mut self, init_fn: fn() -> !) -> Result<usize, ProcessesError> {
        if let Some((pid, empty_slot)) = self
            .processes
            .iter_mut()
            .enumerate()
            .find(|(_, &mut process_frame)| process_frame == ProcessState::Uninitialized)
        {
            let init_stack_frame = unsafe {
                &mut *((PROCESS_BASE - (pid as u32 * PROCESS_MEMORY_SIZE))
                    as *mut InitialStackFrame)
            };

            *init_stack_frame = InitialStackFrame {
                load_stack: LoadStackFrame::default(),
                auto_stack: AutoStackFrame::default(init_fn),
            };

            let auto_stack_addr = ptr::addr_of_mut!(init_stack_frame.auto_stack.r0);

            *empty_slot =
                ProcessState::Initialized(ProcessFrame::init(pid, auto_stack_addr as u32));

            Ok(pid)
        } else {
            Err(ProcessesError::ProcessStackFull)
        }
    }

    /// Prepares the context switch and then actually calls [__context_switch].
    ///
    /// # Arguments
    ///
    /// * [usize] process id (pid)
    ///
    /// # Returns
    ///
    /// * [Ok] when context switch was successful.
    /// * [ProcessesError] when context switch failed.
    ///
    /// *Note*
    ///
    /// When switching from msp, that is current_process_idx is [None], the argument
    /// in [__context_switch] from_psp_addr can be 0, because it will not be used.
    pub unsafe fn switch_to_pid(&mut self, pid: usize) -> Result<(), ProcessesError> {
        if let Some(process) = self.processes.get(pid) {
            if let ProcessState::Initialized(mut next_process) = process {
                let psp_current_addr = match self.get_current_process() {
                    Some(current_process) => ptr::addr_of!(current_process.psp) as u32,
                    None => 0,
                };
                self.current_process_idx = Some(pid);

                __context_switch(ptr::addr_of_mut!(next_process.psp) as u32, psp_current_addr);

                Ok(())
            } else {
                return Err(ProcessesError::NotInitialized);
            }
        } else {
            return Err(ProcessesError::ProcessNotAvailable);
        }
    }


    /// The current process frame is either [Some] [ProcessState::Initialized] [ProcessFrame] or [None] when no process was ever running.
    fn get_current_process(&self) -> Option<&ProcessFrame> {
        if let Some(current_process_idx) = self.current_process_idx {
            if let ProcessState::Initialized(current_process) =
                self.processes.get(current_process_idx).unwrap()
            {
                return Some(current_process);
            }
        }
        None
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProcessFrame {
    psp: u32,
    pid: usize,
}

impl ProcessFrame {
    pub fn init(pid: usize, psp: u32) -> ProcessFrame {
        ProcessFrame { pid, psp }
    }
}

#[repr(C)]
pub struct InitialStackFrame {
    #[allow(dead_code)]
    load_stack: LoadStackFrame,
    auto_stack: AutoStackFrame,
}

#[repr(C, align(8))]
pub struct LoadStackFrame {
    /// Here initial value for control
    _buffer: [u32; 7],
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
            _buffer: [0; 7],
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

#[repr(C, align(8))]
pub struct AutoStackFrame {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,
}

impl AutoStackFrame {
    fn default(init_fn: fn() -> !) -> AutoStackFrame {
        let init_fn = ptr::addr_of!(init_fn);
        AutoStackFrame {
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
