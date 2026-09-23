use core::num::{NonZero};

use crate::process::{process_table::{ProcessId, ProcessTable, WaitQueueId}, scheduler::Scheduler};

pub mod process_table;
pub mod scheduler;


pub struct ProcessManager {
    processes: ProcessTable,
    scheduler: Scheduler,
    current_process: Option<ProcessId>,
}
impl ProcessManager {
    /// Blocks the current process, until the given queue is triggered.
    pub fn block_current(&mut self, queue: WaitQueueId) {
        for process_option in self.processes.get_entries() {
            if let Some((pid, process_page)) = process_option {
                if let Some(current_process) = self.current_process {
                    if current_process == *pid {
                        
                    }
                }
            }
        }
    }
}