#![no_std]
#![no_main]

use crate::arch::riscv64::cpu::wait_for_interrupt;

pub mod arch;


core::arch::global_asm!(include_str!("entry.s"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main(hart_id: usize) -> ! {
    if hart_id == 0 {
        // Bootstrap Hart
    }
    loop {
        // Secondary Harts. Wait for bootstrap hart to setup.
        wait_for_interrupt();
    }
}



/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {

    }
}