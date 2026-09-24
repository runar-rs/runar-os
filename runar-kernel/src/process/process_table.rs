use core::{f32::consts::E, fmt::Display, num::NonZero};

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

impl Display for ProcessId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.id)
    }
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

/// Errors that can occur when trying to change the process state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStateError {
    /// The process is already blocked
    AlreadyBlocked,
    /// The process has already exited
    Exited,
    InvalidTransition,
    /// Switch to `Blocked` was unsuccessful, because no wait queue was set
    NoWaitQueueSet,
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
    exit_code: Option<ExitCode>,
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

    #[inline(always)]
    pub fn get_exit_code(&self) -> &Option<ExitCode> {
        &self.exit_code
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
            Err(ProcessStateError::NoWaitQueueSet)
        }
    }

    /// Tries setting the wait queue id the process is waiting for.
    pub fn try_set_blocked_on(&mut self, wait_queue_id: Option<WaitQueueId>) -> Result<(),WaitQueueError> {
        if self.blocked_on == None {
            self.blocked_on = wait_queue_id;
            Ok(())
        }
        else {
            // A wait queue is already set. Overwriting it would result in a loss of the previous block
            Err(WaitQueueError::WaitQueueAlreadySet)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitQueueError {
    /// The wait 
    WaitQueueAlreadySet
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WaitQueueId {
    id: NonZero<usize>,
}


pub struct WaitQueue {}