//! # Context Switch
//!
//! This struct will be used to store from the [ProcessControlBlock][super::scheduler::ProcessControlBlock]
//! the psp address of the current aka from and next process.

/// Holds addresses required for context switch
pub struct ContextSwitch {
    pub psp_from_addr: u32,
    pub psp_next_addr: u32,
}

impl ContextSwitch {
    /// Retrieves the required psp addresses for loading and saving the new processes.
    ///
    /// Will swap psp addresses from and next so that the second context switch will return
    /// to the previous process. In this implementation it will always go back to pid0, aka
    /// our scheduler.
    pub fn get_addr_and_swap(&mut self) -> (u32, u32) {
        let jump_to = (self.psp_next_addr, self.psp_from_addr);
        core::mem::swap(&mut self.psp_next_addr, &mut self.psp_from_addr);
        jump_to
    }

    pub fn set_from_addr(&mut self, psp_from_addr: u32) {
        self.psp_from_addr = psp_from_addr;
    }

    pub fn set_next_addr(&mut self, psp_next_addr: u32) {
        self.psp_next_addr = psp_next_addr;
    }
}
