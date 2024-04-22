use crate::{
    config::MAX_SYSCALL_NUM, mm::{VirtAddr, PhysAddr}, task::{
        task_mmap, task_munmap, current_task_info, ppn_by_vpn, change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus
    }, timer::{get_time_ms, get_time_us}
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

impl TaskInfo {
    pub fn new() -> Self {
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: get_time_ms(),
        }
    }

    pub fn new_with_status(status: TaskStatus) -> Self {
        TaskInfo {
            status,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: get_time_ms(),
        }
    }

    pub fn update_syscall_times(&mut self, syscall_id: usize) {
        self.syscall_times[syscall_id] += 1;
    }
}

fn va_to_pa(va: VirtAddr) -> Option<PhysAddr> {
    let offset = va.page_offset();
    let ppn = ppn_by_vpn(va.floor());
    match ppn {
        Some(ppn) => Some(PhysAddr::from((ppn.0 << 12) | offset)),
        _ => {
            error!("sys_get_time() failed");
            None
        }
    }
}

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
    let va = VirtAddr(_ts as usize);
    let pa = va_to_pa(va);
    if let Some(pa) = pa {
        let us = get_time_us();
        let phys_ts = pa.0 as *mut TimeVal;
        unsafe {
            *phys_ts = TimeVal{
                sec: us / 1_000_000,
                usec: us % 1_000_000,
            };
        }
        0
    } else {
        error!("sys_get_time() failed");
        -1
    }
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    if _ti.is_null() {
        return -1
    }

    let info = current_task_info();
    let va = VirtAddr::from(_ti as usize);
    let pa = va_to_pa(va);
    if let Some(pa) = pa {
        let ti = pa.0 as *mut TaskInfo;
        unsafe {
            (*ti).status = info.status;
            (*ti).syscall_times = info.syscall_times;
            (*ti).time = get_time_ms() - info.time;
        }
        0
    } else {
        error!("sys_task_info() failed");
        -1
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    task_mmap(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    task_munmap(start, len)
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