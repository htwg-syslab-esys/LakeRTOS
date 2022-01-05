use self::processes::Processes;

pub mod exceptions;
pub mod processes;

const PROCESS_BASE: u32 = 0x2000_8000;

pub static mut PROCESS_OFFSET_TABLE: Option<Processes> = None;

extern "C" {
    pub fn __context_switch(psp_next_addr: u32, psp_current_addr: u32);
    pub fn __breakpoint();
}
