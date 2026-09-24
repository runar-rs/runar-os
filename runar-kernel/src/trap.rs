use crate::arch::riscv64::context::CpuContext;


pub extern "C" fn trap_handler() -> ! {
    // Save the current Cpu Context
    let context = CpuContext::new();

    let scause: usize;
    unsafe {
        core::arch::asm!("csrr {}, scause", out(reg) scause);
    }
    let interrupt = scause >> 31;
    let exception_code = scause & 0x8f_ff_ff_ff;
    if interrupt == 1 {
        // Interrupt
        match exception_code {
            1 => {
                // Supervisor software interrupt
            },
            5 => {
                // Supervisor timer interrupt
            },
            9 => {
                // Supervisor external interrupt
            },
            13 => {
                // Counter-overflow interrupt
            },
            0 | 2 | 3 | 6 ..= 8 | 10 ..= 12 | 14 | 15 => {
                panic!("Reserved Exception Code")
            },
            _ => {
                //Platform use
            }
        }
    }
    else {
        // Exception
        match exception_code {
            0 => {
                // Instruction address misaligned
            },
            1 => {
                // Instruction access fault
            },
            2 => {
                // Illegal instruction
            },
            3 => {
                // Breakpoint
            },
            4 => {
                // Load access misaligned
            },
            5 => {
                // Load access fault
            },
            6 => {
                // Store/AMO address misaligned
            },
            7 => {
                // Store/AMO access fault
            },
            8 => {
                // Environment call from U-mode
            },
            9 => {
                // Environment call from S-mode
            },
            12 => {
                // Instruction page fault
            },
            13 => {
                // Load page fault
            },
            15 => {
                // Store/AMO page fault
            },
            18 => {
                // Software check
            },
            19 => {
                // Hardware error
            }
            10 | 11 | 14 | 16 | 17 | 20 ..= 23 | 32 ..= 47 | 64.. => {
                panic!("Reserved Exception Code")
            },
            _=> {
                // Custom use
            }

        }
    }
    // Safe, if this function is only invoked if a trap is triggered.
    // Returns from the trap.
    unsafe {
        context.set();
        core::arch::asm!("sret", options(noreturn));
    };
}