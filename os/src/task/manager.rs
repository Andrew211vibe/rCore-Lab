use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;

/// A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    /// Create an empty TaskManager
    pub fn new() -> Self {
        Self { ready_queue: VecDeque::new(), }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        let pass = task.inner_exclusive_access().pass;
        let len = self.ready_queue.len();
        for idx in 0..len {
            let tmp = self
                .ready_queue
                .get(idx)
                .unwrap();
            let val = tmp
                .inner_exclusive_access()
                .pass;
            if pass < val {
                // println!("new task pass: {}, inserted before idx {}", pass, idx);
                self.ready_queue.insert(idx, task);
                return;
            }
        }
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        let mut target = self.ready_queue.pop_front();
        if let Some(ref mut tcb) = target {
            tcb.inner_exclusive_access().update_stride();
        }
        target
    }
}

lazy_static! {
    /// TASK_MANAGE instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> = 
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    trace!("kernel: Taskmanager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}