use core::{f32::consts::E, fmt::Display, num::NonZero};

use crate::{arch::riscv64::context::CpuContext, memory::page::Sv32PageTable};

pub struct ProcessTable<'a> {
    entries: [Option<(ProcessId, ProcessTableEntry<'a>)>; 64],
    next: Option<&'a ProcessTable<'a>>,
}
impl<'a> ProcessTable<'a> {
    /// Creates an empty process table.
    pub const fn new() -> Self {
        Self {
            entries: [const { None }; 64],
            next: None,
        }
    }

    #[inline(always)]
    pub fn get_entries(&self) -> &[Option<(ProcessId, ProcessTableEntry<'a>)>; 64] {
        &self.entries
    }

    #[inline(always)]
    pub fn get_mut_entries(&mut self) -> &mut [Option<(ProcessId, ProcessTableEntry<'a>)>; 64] {
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

/// Refers to a Page Table.
pub enum PageTableRef<'a> {
    /// Page table in Sv32 systems
    Sv32(&'a Sv32PageTable),
    /// Page table in Sv39 systems
    Sv39,
    /// Page table in Sv48 systems
    Sv48,
    /// Page table in Sv57 systems
    Sv57,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessState {
    /// The process is new and not yet fully initialized.
    New,
    /// The process is currently not running, but is ready to be run.
    Ready,
    /// The process is running
    Running,
    /// The process is blocked
    Blocked,
    /// The process is sleeping
    Sleeping,
    /// The process hasn't triggered the watchdog timer.
    Zombie,
    /// The process has exited.
    Exited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode<'a> {
    /// The process exited successfully.
    Success,
    /// The process exited unsuccessfully.
    Error(ErrorCode<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode<'a> {
    /// The process tried accessing memory without the required rights.
    SegmentationFault(&'a usize),
}

/// Errors that can occur when trying to change the process state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStateError {
    /// The process is already blocked
    AlreadyBlocked,
    /// The process has already exited
    Exited,
    /// Invalid transition without a specific error message.
    InvalidTransition,
    /// Switch to `Blocked` was unsuccessful, because no wait queue was set
    NoWaitQueueSet,
}

/// Priority of the process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessPriority(u8);

/// Entry in the process table. Each process is represented by its individual [ProcessTableEntry]
pub struct ProcessTableEntry<'a> {
    /// Process ID
    pid: ProcessId,
    /// PID of the parent process
    parent_pid: Option<ProcessId>,
    /// CPU context of the process.
    /// 
    /// The CPU context contains the state
    context: CpuContext<'a>,

    /// Pointer to the root page table of the process.
    page_table: PageTableRef<'a>,
    /// Pointer to the top of the stack.
    stack_top: usize,
    /// Size of the stack.
    stack_size: usize,

    /// Priority of the process.
    priority: ProcessPriority,
    /// Current CPU Time.
    /// 
    /// Computational time, the process
    cpu_time: usize,

    /// The state of the process
    state: ProcessState,
    /// The wait queue
    blocked_on: Option<WaitQueueId>,
    /// Exit code of the process.
    /// 
    /// Some if the process has exited.
    /// 
    /// None if the process is still running.
    exit_code: Option<ExitCode<'a>>,
}
// Non mutable getters
impl<'a> ProcessTableEntry<'a> {
    /// Get the Process ID of the process.
    #[inline(always)]
    pub fn get_pid(&self) -> &ProcessId {
        &self.pid
    }

    /// Get the Process ID of the parent of the process.
    #[inline(always)]
    pub fn get_parent_pid(&self) -> &Option<ProcessId> {
        &self.parent_pid
    }

    /// Get the CPU context
    #[inline(always)]
    pub fn get_context(&self) -> &CpuContext<'a> {
        &self.context
    }

    /// Get a reference to the page table.
    #[inline(always)]
    pub fn get_page_table(&self) -> &PageTableRef<'a> {
        &self.page_table
    }

    /// Get the pointer to the top of the stack of the process.
    #[inline(always)]
    pub fn get_stack_top(&self) -> &usize {
        &self.stack_top
    }

    /// Get the size of the stack of the process
    #[inline(always)]
    pub fn get_stack_size(&self) -> &usize {
        &self.stack_size
    }

    /// Get the priority of the process
    #[inline(always)]
    pub fn get_priority(&self) -> &ProcessPriority {
        &self.priority
    }

    /// Get the current cpu time of the process.
    #[inline(always)]
    pub fn get_cpu_time(&self) -> &usize {
        &self.cpu_time
    }

    /// Get the exit code of the process.
    #[inline(always)]
    pub fn get_exit_code(&self) -> &Option<ExitCode<'a>> {
        &self.exit_code
    }
}

impl<'a> ProcessTableEntry<'a> {
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

    /// Tries setting the process state to ready.
    pub fn try_set_ready(&mut self) -> Result<(), ProcessStateError> {
        if self.state != ProcessState::Blocked && self.state != ProcessState::Exited { 
            self.state = ProcessState::Ready;
            Ok(())
        }
        else {
            Err(ProcessStateError::InvalidTransition)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitQueueError {
    /// The wait 
    WaitQueueAlreadySet
}

/// Zero-cost wrapper for wait queue ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WaitQueueId {
    id: NonZero<usize>,
}


pub struct WaitQueue {}