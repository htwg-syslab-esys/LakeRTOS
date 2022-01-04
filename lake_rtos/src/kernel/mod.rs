use self::processes::Processes;

pub mod exceptions;
pub mod processes;

const PROCESS_BASE: u32 = 0x2000_8000;

pub static mut PROCESS_OFFSET_TABLE: Option<Processes> = None;

extern "C" {
    pub fn __context_switch(to_psp: u32, from_psp_addr: u32) -> u32;
    pub fn __breakpoint();
}
