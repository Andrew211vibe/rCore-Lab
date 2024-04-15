use crate::{TaskInfo, SignalAction};
use super::{Stat, TimeVal};

pub const SYSCALL_READ: usize = 63;
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_YIELD: usize = 124;
pub const SYSCALL_GETTIMEOFDAY: usize = 169;
pub const SYSCALL_TASK_INFO: usize = 410;

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

/// 功能：将内存中缓冲区中的数据写入文件。
/// 参数：`fd` 表示待写入文件的文件描述符；
///      `buf` 表示内存中缓冲区的起始地址；
///      `len` 表示内存中缓冲区的长度。
/// 返回值：返回成功写入的长度。
/// syscall ID：64
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

/// 功能：将文件中的数据读入内存缓冲区。
/// 参数：`fd` 表示待读入文件的文件描述符；
///      `buf` 表示内存中缓冲区的起始地址；
/// 返回值：返回成功读取的长度。
/// syscall ID：63
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(SYSCALL_READ, [fd, buffer.as_mut_ptr() as usize, buffer.len()])
}

/// 功能：退出应用程序并将返回值告知批处理系统。
/// 参数：`xstate` 表示应用程序的返回值。
/// 返回值：该系统调用不应该返回。
/// syscall ID：93
pub fn sys_exit(xstate: i32) -> ! {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0]);
    panic!("sys_exit never returns!");
}

/// 功能：应用主动交出 CPU 所有权并切换到其他应用。
/// 返回值：总是返回 0。
/// syscall ID：124
pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time(time: &mut TimeVal, tz: usize) -> isize {
    syscall(SYSCALL_GETTIMEOFDAY, [time as *const _ as usize, tz, 0])
}

pub fn sys_task_info(info: &mut TaskInfo) -> isize {
    syscall(SYSCALL_TASK_INFO, [info as *const _ as usize, 0, 0])
}