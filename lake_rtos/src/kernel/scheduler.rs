//! # Scheduler
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

use core::{iter::Cycle, ops::Range, ptr};

use super::{__context_switch, exceptions::trigger_PendSV, PROCESS_BASE, SCHEDULER};

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
    NotAvailable,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
}

#[derive(Debug)]
pub struct Scheduler {
    processes: [Option<ProcessControlBlock>; ALLOWED_PROCESSES],
    current_process_idx: Option<usize>,
    cycle: Cycle<Range<usize>>,
}

impl Scheduler {
    /// Only the first call will return an reference to Some([Processes])
    pub fn take() -> Option<&'static mut Scheduler> {
        unsafe {
            match SCHEDULER {
                Some(_) => None,
                None => {
                    SCHEDULER = Some(Scheduler {
                        processes: [None; ALLOWED_PROCESSES],
                        current_process_idx: None,
                        cycle: (0..ALLOWED_PROCESSES).cycle(),
                    });
                    Some(SCHEDULER.as_mut().unwrap())
                }
            }
        }
    }

    pub fn create_process(&mut self, init_fn: fn() -> !) -> Result<usize, ProcessesError> {
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
                auto_stack: AutoStackFrame::default(init_fn),
            };

            let auto_stack_addr = ptr::addr_of_mut!(init_stack_frame.auto_stack.r0);

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
    fn switch_to_pid(&mut self, pid: usize) -> Result<(), ProcessesError> {
        let next_process = match self.processes.get_mut(pid) {
            Some(process) => process,
            None => return Err(ProcessesError::NotAvailable),
        };

        let psp_next_addr = match next_process {
            Some(next_pcb) => {
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

    /// The current process frame is either [Some] [ProcessState::Initialized] [ProcessFrame] or [None] when no process was ever running.
    fn get_current_process(&mut self) -> Option<&mut ProcessControlBlock> {
        if let Some(current_process_idx) = self.current_process_idx {
            if let Some(ref mut current_pcb) = self.processes[current_process_idx] {
                return Some(current_pcb);
            }
        }
        None
    }

    pub fn start_scheduling(&mut self) -> ! {
        trigger_PendSV();
        loop {}
    }

    pub fn schedule(&mut self) {
        loop {
            if let Some(pid) = self.cycle.next() {
                if let Ok(()) = self.switch_to_pid(pid) {
                    return;
                }
            }
        }
    }
}

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
