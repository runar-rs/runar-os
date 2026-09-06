
#![no_std]
#![no_main]

use crate::bootinfo::BootInfo;

pub mod bootinfo;

pub mod memory;
/// Start of the kernel. Should be only called on the primary hart, hart 0.
#[unsafe(no_mangle)]
pub extern "C" fn _start(info: &BootInfo) -> ! {
    loop {
        
    }
}



/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {

    }
}