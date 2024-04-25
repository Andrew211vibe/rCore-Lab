/// read syscall
const SYSCALL_READ: usize = 63;
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
/// sbrk syscall
const SYSCALL_SBRK: usize = 214;
/// munmap syscall
const SYSCALL_MUNMAP: usize = 215;
/// mmap syscall
const SYSCALL_MMAP: usize = 222;
/// setpriority syscall
const SYSCALL_SET_PRIORITY: usize = 140;
/// fork syscall
const SYSCALL_FORK: usize = 220;
/// exec syscall
const SYSCALL_EXEC: usize = 221;
/// waitpid syscall
const SYSCALL_WAITPID: usize = 260;
/// spawn syscall
const SYSCALL_SPAWN: usize = 400;
/// getpid syscall
const SYSCALL_GETPID: usize = 172;

mod fs;
pub mod process;

use fs::*;
use process::*;

use crate::task::update_syscall_times;

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_READ => {
            update_syscall_times(syscall_id);
            sys_read(args[0], args[1] as *const u8, args[2])
        },
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
        SYSCALL_MMAP => {
            update_syscall_times(syscall_id);
            sys_mmap(args[0], args[1], args[2])
        }
        SYSCALL_MUNMAP => {
            update_syscall_times(syscall_id);
            sys_munmap(args[0], args[1])
        }
        SYSCALL_SBRK => {
            update_syscall_times(syscall_id);
            sys_sbrk(args[0] as i32)
        },
        SYSCALL_SPAWN => {
            update_syscall_times(syscall_id);
            sys_spawn(args[0] as *const u8)
        },
        SYSCALL_SET_PRIORITY => {
            update_syscall_times(syscall_id);
            sys_set_priority(args[0] as isize)
        },
        SYSCALL_WAITPID => {
            update_syscall_times(syscall_id);
            sys_waitpid(args[0] as isize, args[1] as *mut i32)
        },
        SYSCALL_GETPID => {
            update_syscall_times(syscall_id);
            sys_getpid()
        },
        SYSCALL_FORK => {
            update_syscall_times(syscall_id);
            sys_fork()
        },
        SYSCALL_EXEC => {
            update_syscall_times(syscall_id);
            sys_exec(args[0] as *const u8)
        },
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}