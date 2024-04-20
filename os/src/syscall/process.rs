use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus
    },
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information 
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The number of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

// impl TaskInfo {
//     pub fn new() -> Self {
//         TaskInfo {
//             status: TaskStatus::UnInit,
//             syscall_times: [0; MAX_SYSCALL_NUM],
//             time: get_time_ms(),
//         }
//     }

//     pub fn update_syscall_times(&mut self, syscall_id: usize) {
//         self.syscall_times[syscall_id] += 1;
//     }
// }

/// task exits and submit an exit code
pub fn sys_exit(xstate: i32) -> ! {
    trace!("[kernel] Application exited with code {}", xstate);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: Get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    // let us = get_time_us();
    // unsafe {
    //     *ts = TimeVal{
    //         sec: us / 1_000_000,
    //         usec: us % 1_000_000,
    //     };
    // }
    // 0
    -1
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    // if _ti.is_null() {
    //     return -1
    // }
    // let info = get_current_task_info();
    // unsafe {
    //     (*_ti).status = info.status;
    //     (*_ti).syscall_times = info.syscall_times;
    //     (*_ti).time = get_time_ms() - info.time;
    // }
    // 0
    -1
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}