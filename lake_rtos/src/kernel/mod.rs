pub mod exceptions;
pub mod threads;

const PROCESS_BASE: u32 = 0x2000_8000;

extern "C" {
    pub fn __context_switch(to_sp: u32, from_sp: u32) -> u32;
    pub fn __breakpoint();
}
