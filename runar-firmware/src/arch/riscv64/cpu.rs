/// Blocks hart until interrupt is received.
#[inline]
pub fn wait_for_interrupt() {
    unsafe {
        core::arch::asm!("wfi");
    }
}