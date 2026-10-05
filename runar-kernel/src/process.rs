use core::{num::NonZero, ops::{Index, IndexMut}};

use crate::{ arch::{NUM_HARTS, riscv64::{HartNum, context::CpuContext, get_mapped_hart_num}}, process::{process_table::{ProcessId, ProcessTable, WaitQueueId}, scheduler::Scheduler}};
use crate::sync::SpinMutex;

pub mod process_table;
pub mod scheduler;

pub struct HartProcessMap {
    current_process: [Option<ProcessId>; NUM_HARTS],
}

impl HartProcessMap {
    pub const fn new_empty() -> Self {
        Self {
            current_process: [None; NUM_HARTS]
        }
    }
    /// Gets a non-mutable borrow of the current process of the hart, that is executing this
    /// function.
    pub fn get_current_harts_process(&self) -> &Option<ProcessId> {
        &self.current_process[usize::from(get_mapped_hart_num())]
    }

    /// Gets a mutable borrow of the current process of the hart, that is executing this function.
    pub fn get_mut_current_harts_process(&mut self) -> &mut Option<ProcessId> {
        &mut self.current_process[usize::from(get_mapped_hart_num())]
    }
}

impl Index<HartNum> for HartProcessMap {
    type Output = Option<ProcessId>;

    fn index(&self, index: HartNum) -> &Self::Output {
        &self.current_process[usize::from(index)]
    }
}

pub struct ProcessManager<'a> {
    processes: ProcessTable<'a>,
    scheduler: Scheduler,
    /// The current processes for each of the harts.
    current_process: HartProcessMap,
}

impl ProcessManager<'static> {
    /// Creates an empty process manager suitable for static kernel storage.
    pub const fn new() -> Self {
        Self {
            processes: ProcessTable::new(),
            scheduler: Scheduler::new(),
            current_process: HartProcessMap::new_empty(),
        }
    }
}

/// The kernel-wide process manager. Acquire its lock before accessing manager state.
///
///
pub static PROCESS_MANAGER: SpinMutex<ProcessManager<'static>> = SpinMutex::new(ProcessManager::new());

impl<'a> ProcessManager<'a> {
    /// Blocks the current process, until the given queue is triggered.
    pub fn block_current(&mut self, queue: WaitQueueId) {
        for process_option in self.processes.get_mut_entries() {
            if let Some((pid, process_page)) = process_option {
                if let Some(current_process) = self.current_process.get_current_harts_process() {
                    if current_process == pid {
                        match process_page.try_set_blocked_on(Some(queue)) {
                            Ok(_) => {
                                // Set the process to blocked
                                if let Err(err) = process_page.try_set_blocked() {
                                    // Setting the ProcessState to blocked has failed
                                    match err {
                                        process_table::ProcessStateError::AlreadyBlocked => {
                                            // If the process was already blocked, a wait queue must be set. If `block_current()` is called
                                            // an error should be raised by trying to overwrite the wait queue. Therefore, this block should
                                            // never be reached.
                                            panic!("PID: {}: Invalid Process State \n The process was blocked without a wait queue set.", *pid)
                                        },
                                        process_table::ProcessStateError::Exited => todo!(),
                                        process_table::ProcessStateError::InvalidTransition => todo!(),
                                        process_table::ProcessStateError::NoWaitQueueSet => todo!(),
                                    }
                                }
                            },
                            Err(_err) => {
                                // Process already blocked.
                            }
                        }
                        // Return transfer to scheduler.
                    }
                }
                else {
                    panic!("Current Process doesn't exist")
                }
            }
        }
    }

    /// Yields the current process.
    pub fn yield_current(&mut self) {
        if let Some(current_harts_pid) = self.current_process.get_current_harts_process() {
            for process_option in self.processes.get_mut_entries() {
                // Search the current process.
                if let Some((pid, process_page)) = process_option {
                    if pid == current_harts_pid {
                        // TODO try_set_ready()
                        match process_page.try_set_ready() {
                            Ok(_) => {},
                            Err(err) => {
                                // Something went wrong
                            }
                        }
                    }
                }
            }
        }
        else {
            // Shouldn't be reached, as this method should only be called when current process is set.
        }
    }
}