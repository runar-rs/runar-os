use core::{f32::consts::E, num::NonZero};

use crate::arch::riscv64::context::CpuContext;

pub struct ProcessTable {
    entries: [Option<(ProcessId, ProcessTableEntry)>; 64]
}
impl ProcessTable {
    #[inline(always)]
    pub fn get_entries(&self) -> &[Option<(ProcessId, ProcessTableEntry)>; 64] {
        &self.entries
    }

    #[inline(always)]
    pub fn get_mut_entries(&mut self) -> &mut [Option<(ProcessId, ProcessTableEntry)>; 64] {
        &mut self.entries
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessId {
    id: NonZero<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessState {
    /// The process is new and not yet fully initialized.
    New,
    Ready,
    Running,
    Blocked,
    Sleeping,
    Zombie,
    Exited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    Success,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStateError {
    AlreadyBlocked,
    Exited,
    InvalidTransition,
    NoBlockedOnDefined,
}

pub struct ProcessTableEntry {
    /// Process ID
    pid: ProcessId,
    /// PID of the parent process
    parent_pid: Option<ProcessId>,
    /// CPU context of the process
    context: CpuContext,

    page_table: usize,
    stack_top: usize,
    stack_size: usize,

    /// Priority of the process
    priority: u8,
    /// Current CPU Time
    cpu_time: usize,

    state: ProcessState,
    blocked_on: Option<WaitQueueId>,
    exit_code: ExitCode,
}
// Non mutable getters
impl ProcessTableEntry {
    #[inline(always)]
    pub fn get_pid(&self) -> &ProcessId {
        &self.pid
    }

    #[inline(always)]
    pub fn get_parent_pid(&self) -> &Option<ProcessId> {
        &self.parent_pid
    }

    #[inline(always)]
    pub fn get_context(&self) -> &CpuContext {
        &self.context
    }

    #[inline(always)]
    pub fn get_page_table(&self) -> &usize {
        &self.page_table
    }

    #[inline(always)]
    pub fn get_stack_top(&self) -> &usize {
        &self.stack_top
    }

    #[inline(always)]
    pub fn get_stack_size(&self) -> &usize {
        &self.stack_size
    }

    #[inline(always)]
    pub fn get_priority(&self) -> &u8 {
        &self.priority
    }

    #[inline(always)]
    pub fn get_cpu_time(&self) -> &usize {
        &self.cpu_time
    }
}
impl ProcessTableEntry {
    /// Tries setting the process state to blocked.
    pub fn try_set_blocked(&mut self) -> Result<(), ProcessStateError> {
        if let Some(_) = self.blocked_on {
            self.state = ProcessState::Blocked;
            Ok(())
        }
        else {
            // No blocked_on set.
            Err(ProcessStateError::NoBlockedOnDefined)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WaitQueueId {
    id: NonZero<usize>,
}


pub struct WaitQueue {}