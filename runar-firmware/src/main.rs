#![no_std]
#![no_main]

core::arch::global_asm!(include_str!("entry.s"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main(hart_id: usize) -> ! {
    if hart_id == 0 {
        // Bootstrap Hart
    }
    loop {
        
    }
}



/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {

    }
}