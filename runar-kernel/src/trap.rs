use crate::{arch::riscv64::context::CpuContext, process::{PROCESS_MANAGER, process_table::ExitCode}};

pub enum SyscallError {
    /// Casting from usize to the syscall enum failed.
    InvalidSyscall,
}

#[repr(usize)]
pub enum Syscall {
    /// Exit the current process
    Exit,
    
    /// Yield the current process.
    /// 
    /// Voluntarily returns execution to the operating system.
    Yield,
    
    /// Let the current process sleep
    Sleep,

    /// Spawn a new process.
    Spawn,

    /// Clone the current process.
    Clone,

    /// Notify a notify handle.
    Notify,
    /// Start the process watchdog timer.
    StartWDC,

    /// Stop the process watchdog timer.
    StopWDC,
    
    /// Trigger the process watchdog timer.
    /// 
    /// This resets the watchdog timer. If the process watchdog timer is stopped when this syscall
    /// is triggered, it will be restarted.
    TriggerWDC,

    /// Set the process watchdog timer timeout.
    SetWDCTimeout,
}

impl TryFrom<usize> for Syscall {
    type Error = SyscallError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            _ => Err(SyscallError::InvalidSyscall)
        }
    }
}
impl Into<usize> for Syscall {
    fn into(self) -> usize {
        self as usize
    }
}

/// Trap handler
/// 
/// # Safety
/// Safe if only invoked via stvec if a trap is triggered.
pub unsafe extern "C" fn trap_handler() -> ! {
    // Save the current Cpu Context
    let context = CpuContext::new();
    let satp: usize;
    let scause: usize;
    let stval: usize;
    unsafe {
        core::arch::asm!("cssr {}, satp", out(reg) satp);
        core::arch::asm!("csrr {}, scause", out(reg) scause);
        core::arch::asm!("csrr {}, stval", out(reg) stval);
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
                // Custom use
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

                // Allow unused variables as not all syscalls have yet been implemented
                #[allow(unused_variables)]
                let [a0, a1, a2, a3, a4, a5, a6, a7] = context.get_mut_a_reg();
                if let Ok(syscall) = Syscall::try_from(*a7) {
                    match syscall {
                        Syscall::Exit => {
                            let exit_code = ExitCode::from(*a7);
                            let mut process_manager = PROCESS_MANAGER.lock();
                            process_manager.exit_current(exit_code);
                        },
                        Syscall::Yield => {
                            let mut process_manager = PROCESS_MANAGER.lock();
                            process_manager.yield_current();
                        },
                        Syscall::Sleep => todo!(),

                        Syscall::TriggerWDC => {
                            let mut process_manager = PROCESS_MANAGER.lock();
                            process_manager.reset_current_wdc();
                        }
                        _ => todo!(),
                    }
                }
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
    // Safe, if this function is invoked if a trap is triggered.
    // Returns from the trap.
    unsafe {
        context.set();
        // Set satp
        core::arch::asm!("csrw satp, {0}", "sfence.vma x0, x0", in(reg) satp, options(nostack));
        // Return
        core::arch::asm!("sret", options(noreturn));
    };
}