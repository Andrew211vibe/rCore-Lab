/// write syscall
const SYSCALL_WRITE: usize = 64;
/// exit syscall
const SYSCALL_EXIT: usize = 93;
/// yield syscall
const SYSCALL_YIELD: usize = 124;
/// gettime syscall
const SYSCALL_GET_TIME: usize = 169;
/// taskinfo syscall
const SYSCALL_TASK_INFO: usize = 410;

mod fs;
pub mod process;

use fs::*;
use process::*;

use crate::task::update_syscall_times;

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => {
            update_syscall_times(syscall_id);
            sys_write(args[0], args[1] as *const u8, args[2])
        },
        SYSCALL_EXIT => {
            update_syscall_times(syscall_id);
            sys_exit(args[0] as i32)
        },
        SYSCALL_YIELD => {
            update_syscall_times(syscall_id);
            sys_yield()
        },
        SYSCALL_GET_TIME => {
            update_syscall_times(syscall_id);
            sys_get_time(args[0] as *mut TimeVal, args[1])
        },
        SYSCALL_TASK_INFO => {
            update_syscall_times(syscall_id);
            sys_task_info(args[0] as *mut TaskInfo)
        },
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}