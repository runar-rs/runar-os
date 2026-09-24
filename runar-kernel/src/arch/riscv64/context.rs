#[repr(C)]
pub struct CpuContext<'a> {
    _marker: core::marker::PhantomData<&'a ()>,
    /// x1: Return address
    ra: usize,
    /// x2: Stack pointer
    sp: usize,
    /// x3: Global pointer
    gp: usize,
    /// x4: Thread pointer
    tp: usize,
    /// x5: Temp register 0
    t0: usize,
    /// x6: Temp register 1
    t1: usize,
    /// x7: Temp register 2
    t2: usize,
    /// x8: Saved register 0/frame pointer
    s0_fp: usize,
    /// x9: Saved register 1
    s1: usize,
    /// x10: function argument register 0
    a0: usize,
    /// x11
    a1: usize,
    /// x12
    a2: usize,
    /// x13
    a3: usize,
    /// x14
    a4: usize,
    /// x15
    a5: usize,
    /// x16
    a6: usize,
    /// x17
    a7: usize,
    /// x18
    s2: usize,
    /// x19
    s3: usize,
    /// x20
    s4: usize,
    /// x21
    s5: usize,
    /// x22
    s6: usize,
    /// x23
    s7: usize,
    /// x24
    s8: usize,
    /// x25
    s9: usize,
    /// x26
    s10: usize,
    /// x27
    s11: usize,
    /// x28
    t3: usize,
    /// x29
    t4: usize,
    /// x30
    t5: usize,
    /// x31
    t6: usize,
    /// sepc
    return_addr: usize,
}

impl<'a> CpuContext<'a> {
    /// Creates a new `CpuContext`. Must be the first function called after entering a trap, to save
    /// the state of the cpu in order to be able to return
    #[inline(always)]
    pub fn new() -> &'a Self {
        let context: &'a Self;
        let sp: usize;
        unsafe {
            core::arch::asm!(
                "addi sp, sp, -256",
                "sd ra, 0(sp)",
                "sd sp, 8(sp)",
                "sd gp, 16(sp)",
                "sd tp, 24(sp)",
                "sd t0, 32(sp)",
                "sd t1, 40(sp)",
                "sd t2, 48(sp)",
                "sd s0, 56(sp)",
                "sd s1, 64(sp)",
                "sd a0, 72(sp)",
                "sd a1, 80(sp)",
                "sd a2, 88(sp)",
                "sd a3, 96(sp)",
                "sd a4, 104(sp)",
                "sd a5, 112(sp)",
                "sd a6, 120(sp)",
                "sd a7, 128(sp)",
                "sd s2, 136(sp)",
                "sd s3, 144(sp)",
                "sd s4, 152(sp)",
                "sd s5, 160(sp)",
                "sd s6, 168(sp)",
                "sd s7, 176(sp)",
                "sd s8, 184(sp)",
                "sd s9, 192(sp)",
                "sd s10, 200(sp)",
                "sd s11, 208(sp)",
                "sd t3, 216(sp)",
                "sd t4, 224(sp)",
                "sd t5, 232(sp)",
                "sd t6, 240(sp)",
                "csrr t0, sepc",
                "sd t0, 248(sp)"
            );
            core::arch::asm!("mv {}, sp", out(reg) sp);
            context = &*(sp as *const Self);
        }
        context
    }
    pub unsafe fn set(&self) {
        unsafe {
            // Set Stack Pointer
            core::arch::asm!("mv sp, {}", in(reg) self.sp);
            // Set the other registers
            core::arch::asm!(
                "ld t0, 248(sp)",
                "csrw sepc, t0",
                "ld t6, 240(sp)",
                "ld t5, 232(sp)",
                "ld t4, 224(sp)",
                "ld t3, 216(sp)",
                "ld s11, 208(sp)",
                "ld s10, 200(sp)",
                "ld s9, 192(sp)",
                "ld s8, 184(sp)",
                "ld s7, 176(sp)",
                "ld s6, 168(sp)",
                "ld s5, 160(sp)",
                "ld s4, 152(sp)",
                "ld s3, 144(sp)",
                "ld s2, 136(sp)",
                "ld a7, 128(sp)",
                "ld a6, 120(sp)",
                "ld a5, 112(sp)",
                "ld a4, 104(sp)",
                "ld a3, 96(sp)",
                "ld a2, 88(sp)",
                "ld a1, 80(sp)",
                "ld a0, 72(sp)",
                "ld s1, 64(sp)",
                "ld s0, 56(sp)",
                "ld t2, 48(sp)",
                "ld t1, 40(sp)",
                "ld t0, 32(sp)",
                "ld tp, 24(sp)",
                "ld gp, 16(sp)",
                "ld ra, 0(sp)",
                "addi sp, sp, 256",
            )
        }
    }
}