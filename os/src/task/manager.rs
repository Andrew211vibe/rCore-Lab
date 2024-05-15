use super::TaskControlBlock;
use crate::config::BIG_STRIDE;
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
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        // let mut min_stride = 0;
        // let mut index = 0;
        // for idx in 0..self.ready_queue.len() {
        //     let cur_stride = self.ready_queue[idx].inner_exclusive_access().stride;
        //     if min_stride == 0 && idx == 0 {
        //         min_stride = cur_stride;
        //         continue;
        //     }
        //     println!("    {}-{}-cur:{}-min:{}", index, self.ready_queue[idx].getpid(), cur_stride, min_stride);
        //     if cur_stride < min_stride {
        //         min_stride = cur_stride;
        //         index = idx;
        //     }
        // }
        self.ready_queue.make_contiguous().sort_by_key(|item| {
            let cur_stride = item.inner_exclusive_access().stride;
            cur_stride
        });
        let min_stride = self.ready_queue.front()?.inner_exclusive_access().stride;
        self.ready_queue.iter_mut()
            .for_each(|item| {
            let mut inner = item.inner_exclusive_access();
            // println!("    pid:{}-stride:{}-pass:{}", item.getpid(), inner.stride, inner.pass);
            inner.stride -= min_stride;
        });
        // println!("pop task: pid-{} stride-{}", 
        //     self.ready_queue.front()?.getpid(), 
        //     self.ready_queue.front()?.inner_exclusive_access().stride);
        self.ready_queue.pop_front()
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