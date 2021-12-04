//!
//! Contains the first entrypoint for startup procedure.
//! Only for runtime purposes, acts as a platform for the kernel
//! code build on top of it.
//!

#![no_std]

use core::panic::PanicInfo;
use core::ptr;

///
/// Mandatory resetfunction at adress 0x08000004.
/// Gets called after power on the cpu.
///
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {

    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    // reference to target function
    extern "Rust" {
        fn kmain() -> !;
    }
    
    kmain();
}

///
/// Manually create a section with points to the adress of 
/// the reset function.
///
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;


#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
