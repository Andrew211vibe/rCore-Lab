#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

// 用于退出程序，整合至sbi.rs
// const SYSCALL_EXIT: usize = 93;

// fn syscall(id: usize, args: [usize; 3]) -> isize {
//     let mut ret;
//     unsafe {
//         core::arch::asm!(
//             "ecall",
//             inlateout("x10") args[0] => ret,
//             in("x11") args[1],
//             in("x12") args[2],
//             in("x17") id,
//         );
//     }
//     ret
// }

// pub fn sys_exit(xstate: i32) -> isize {
//     syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
// }

// 用于控制台输出，整合至console.rs
// const SYSCALL_WRITE: usize = 64;

// pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
//     syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
// }

// struct Stdout;

// impl Write for Stdout {
//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         sys_write(1, s.as_bytes());
//         Ok(())
//     }
// }

// pub fn print(args: fmt::Arguments) {
//     Stdout.write_fmt(args).unwrap();
// }

// #[macro_export]
// macro_rules! print {
//     ($fmt: literal $(, &($arg: tt)+)?) => {
//         $crate::console::print(format_args!($fmt $(, $($arg)+)?));
//     }
// }

// #[macro_export]
// macro_rules! println {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
//     }
// }

global_asm!(include_str!("entry.asm"));

// 清空.bss段
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn sbss();
        fn ebss();
        fn erodata();
        fn srodata();
        fn edata();
        fn sdata();
        fn boot_stack_top();
        fn boot_stack_lower_bound();
    }
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize,
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize,
        erodata as usize,
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize,
        edata as usize,
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize,
        boot_stack_lower_bound as usize,
    );
    error!(
        "[kernel] .bss [{:#x}, {:#x})",
        sbss as usize,
        ebss as usize,
    );

    sbi::shutdown()
}

// #[no_mangle]
// extern "C" fn _start() {
    // loop{};

    // println!("Hello, World!");
    // sys_exit(9);
    
    // sbi::shutdown();
//}

// fn main() {
    // println!("Hello, world!");
// }
