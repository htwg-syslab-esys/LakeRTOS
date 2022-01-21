//! # Processes
//!
//! Processes will be placed top of SRAM
//!
//! Memory layout
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
//! A process has as much memory available, as defined in [PROCESS_MEMORY_SIZE].

use core::ptr;

use super::__context_switch;

/// Maximum allowed processes
const ALLOWED_PROCESSES: usize = 4;
/// Starting address of processes (processes are stacked descending)
const PROCESS_BASE: u32 = 0x2000_6000;
/// The reserved memory for a process. This does not protect against memory overflow.
const PROCESS_MEMORY_SIZE: u32 = 0x1000;

/// This [Option] of [Processes] is designed as singleton pattern.
pub static mut PROCESSES: Option<Processes> = None;

#[derive(Debug)]
pub enum ProcessesError {
    /// Process stack is completely occupied.
    ProcessStackFull,
    /// Process was not initialized.
    NotInitialized,
    /// Process is not available. (Index out of Bounds)
    NotAvailable,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
}

#[derive(Debug)]
pub struct Processes {
    processes: [Option<ProcessControlBlock>; ALLOWED_PROCESSES],
    current_process_idx: Option<usize>,
}

impl Processes {
    /// Only the first call will return an reference to Some([Processes])
    pub fn take() -> Option<&'static mut Processes> {
        unsafe {
            match PROCESSES {
                Some(_) => None,
                None => {
                    PROCESSES = Some(Processes {
                        processes: [None; ALLOWED_PROCESSES],
                        current_process_idx: None,
                    });
                    Some(PROCESSES.as_mut().unwrap())
                }
            }
        }
    }

    pub fn create_process(&mut self, init_fn: fn() -> !) -> Result<usize, ProcessesError> {
        if let Some((pid, empty_slot)) = self
            .processes
            .iter_mut()
            .enumerate()
            .find(|(_, &mut process_frame)| process_frame.is_none())
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
    pub fn switch_to_pid(&mut self, pid: usize) -> Result<(), ProcessesError> {
        let next_process = match self.processes.get(pid) {
            Some(process) => process,
            None => return Err(ProcessesError::NotAvailable),
        };

        let psp_next_addr = match next_process {
            Some(mut next_pcb) => {
                next_pcb.state = ProcessState::Running;
                ptr::addr_of_mut!(next_pcb.psp) as u32
            }
            None => return Err(ProcessesError::NotInitialized),
        };

        let psp_current_addr = match self.get_current_process() {
            Some(mut current_pcb) => {
                current_pcb.state = ProcessState::Ready;
                ptr::addr_of!(current_pcb.psp) as u32
            }
            None => 0,
        };

        self.current_process_idx = Some(pid);

        unsafe {
            __context_switch(psp_next_addr, psp_current_addr);
        }

        Ok(())
    }

    /// The current process frame is either [Some] [ProcessControlBlock] or [None] when no process was ever running.
    fn get_current_process(&mut self) -> Option<&mut ProcessControlBlock> {
        if let Some(current_process_idx) = self.current_process_idx {
            if let Some(ref mut current_pcb) = self.processes[current_process_idx] {
                return Some(current_pcb);
            }
        }
        None
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
