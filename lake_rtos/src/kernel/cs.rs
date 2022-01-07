//! # Context Switch
//!

pub struct ContextSwitch {
    pub psp_from_addr: u32,
    pub psp_next_addr: u32,
}

impl ContextSwitch {
    pub fn prepare_context_switch(&mut self) -> (u32, u32) {
        let jump_to = (self.psp_next_addr, self.psp_from_addr);
        core::mem::swap(&mut self.psp_next_addr, &mut self.psp_from_addr);
        jump_to
    }
}
