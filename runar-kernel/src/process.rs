use core::num::{NonZero};

use crate::{arch::riscv64::context::CpuContext, process::{process_table::{ProcessId, ProcessTable, WaitQueueId}, scheduler::Scheduler}};

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
        for process_option in self.processes.get_mut_entries() {
            if let Some((pid, process_page)) = process_option {
                if let Some(current_process) = self.current_process {
                    if current_process == *pid {
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
}