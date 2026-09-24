
#![no_std]
#![no_main]

use crate::{bootinfo::BootInfo, trap::trap_handler};

pub mod bootinfo;

pub mod arch;
pub mod memory;
pub mod process;
pub mod trap;

/// Start of the kernel. Should be only called on the primary hart, hart 0.
#[unsafe(no_mangle)]
pub extern "C" fn _start(info: &BootInfo) -> ! {
    // Register trap handler
    let handler = trap_handler as *const ();
    unsafe {
        core::arch::asm!("csrw stvec, {}", in(reg) handler);
    }

    


    memory::init();
    loop {
        
    }
}



/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {

    }
}