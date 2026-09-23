#[repr(C)]
pub struct CpuContext {
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

impl CpuContext {
    /// Creates a new `CpuContext`. Must be the first function called after entering a trap, to save
    /// the state of the cpu in order to be able to return
    #[inline(always)]
    pub fn new() -> Self {
        let mut context = Self {
            ra: 0,
            sp: 0,
            gp: 0,
            tp: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            s0_fp: 0,
            s1: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            return_addr: 0,
        };

        
        unsafe {
            core::arch::asm!(
                "mv {ra}, ra",
                "mv {sp}, sp",
                "mv {gp}, gp",
                "mv {tp}, tp",
                "mv {t0}, t0",
                "mv {t1}, t1",
                "mv {t2}, t2",
                "mv {s0}, s0",
                "mv {s1}, s1",
                "mv {a0}, a0",
                "mv {a1}, a1",
                "mv {a2}, a2",
                "mv {a3}, a3",
                "mv {a4}, a4",
                "mv {a5}, a5",
                "mv {a6}, a6",
                "mv {a7}, a7",
                "mv {s2}, s2",
                "mv {s3}, s3",
                "mv {s4}, s4",
                "mv {s5}, s5",
                "mv {s6}, s6",
                "mv {s7}, s7",
                "mv {s8}, s8",
                "mv {s9}, s9",
                "mv {s10}, s10",
                "mv {s11}, s11",
                "mv {t3}, t3",
                "mv {t4}, t4",
                "mv {t5}, t5",
                "mv {t6}, t6",
                "csrr {return_addr}, sepc",
                ra = out(reg) context.ra,
                sp = out(reg) context.sp,
                gp = out(reg) context.gp,
                tp = out(reg) context.tp,
                t0 = out(reg) context.t0,
                t1 = out(reg) context.t1,
                t2 = out(reg) context.t2,
                s0 = out(reg) context.s0_fp,
                s1 = out(reg) context.s1,
                a0 = out(reg) context.a0,
                a1 = out(reg) context.a1,
                a2 = out(reg) context.a2,
                a3 = out(reg) context.a3,
                a4 = out(reg) context.a4,
                a5 = out(reg) context.a5,
                a6 = out(reg) context.a6,
                a7 = out(reg) context.a7,
                s2 = out(reg) context.s2,
                s3 = out(reg) context.s3,
                s4 = out(reg) context.s4,
                s5 = out(reg) context.s5,
                s6 = out(reg) context.s6,
                s7 = out(reg) context.s7,
                s8 = out(reg) context.s8,
                s9 = out(reg) context.s9,
                s10 = out(reg) context.s10,
                s11 = out(reg) context.s11,
                t3 = out(reg) context.t3,
                t4 = out(reg) context.t4,
                t5 = out(reg) context.t5,
                t6 = out(reg) context.t6,
                return_addr = out(reg) context.return_addr,
            );
        }
        context
    }
}