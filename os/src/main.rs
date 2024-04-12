#![no_std]
#![no_main]
#![feature(panic_info_message)]
#[macro_use]
extern crate log;

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
pub mod batch;
pub mod lang_items;
pub mod logging;
pub mod sbi;
pub mod syscall;
pub mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

// 清空.bss段
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    // (sbss as usize..ebss as usize).for_each(|a| {
    //     unsafe { (a as *mut u8).write_volatile(0) }
    // });
    unsafe {
        core::slice::from_raw_parts_mut(
            sbss as usize as *mut u8, ebss as usize - sbss as usize
        ).fill(0);
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
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

    // sbi:: shutdown()
    trap::init();
    batch::init();
    batch::run_next_app();
}