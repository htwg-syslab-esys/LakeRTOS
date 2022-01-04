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

use core::ptr;

use super::{__context_switch, PROCESS_BASE};

const ALLOWED_PROCESSES: usize = 4;

#[derive(Debug)]
pub enum ProcessesError {
    ProcessStackFull,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessState {
    Uninitialized,
    Initialized,
}

#[derive(Debug)]
#[repr(C)]
pub struct Processes {
    processes: [ProcessFrame; ALLOWED_PROCESSES],
    current_processes_index: usize,
}

impl Processes {
    pub fn init() -> Processes {
        Processes {
            processes: [ProcessFrame::uninit(); ALLOWED_PROCESSES],
            current_processes_index: 0,
        }
    }

    pub fn create(&mut self, init_fn: fn() -> !) -> Result<usize, ProcessesError> {
        if let Some((pid, empty_slot)) = self
            .processes
            .iter_mut()
            .enumerate()
            .find(|(_, process_frame)| process_frame.state == ProcessState::Uninitialized)
        {
            let init_stack_frame =
                unsafe { &mut *((PROCESS_BASE - (pid as u32 * 0x1000)) as *mut InitialStackFrame) };
            *init_stack_frame = InitialStackFrame {
                load_stack: LoadStackFrame::default(),
                auto_stack: AutoStackFrame::default(init_fn),
            };
            let auto_stack_addr = ptr::addr_of_mut!(init_stack_frame.auto_stack.r0);
            *empty_slot = ProcessFrame::init(pid, auto_stack_addr as u32);
            Ok(pid)
        } else {
            Err(ProcessesError::ProcessStackFull)
        }
    }

    pub fn switch_to_pid(&mut self, pid: usize) {
        unsafe {
            let current_psp_addr =
                ptr::addr_of_mut!(self.processes[self.current_processes_index].psp) as u32;

            self.current_processes_index = pid;

            __context_switch(self.processes[pid].psp, current_psp_addr);
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ProcessFrame {
    state: ProcessState,
    psp: u32,
    pid: usize,
}

impl ProcessFrame {
    pub fn init(pid: usize, psp: u32) -> ProcessFrame {
        ProcessFrame {
            state: ProcessState::Initialized,
            pid,
            psp,
        }
    }

    pub fn uninit() -> ProcessFrame {
        ProcessFrame {
            state: ProcessState::Uninitialized,
            pid: 0,
            psp: 0,
        }
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
            r5: 0xaa,
            r6: 0xbb,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0xff,
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
