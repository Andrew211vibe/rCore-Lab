#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#[macro_use]
extern crate log;
extern crate alloc;

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
pub mod config;
mod heap_alloc;
pub mod lang_items;
mod loader;
pub mod logging;
pub mod sbi;
pub mod sync;
pub mod syscall;
pub mod task;
pub mod timer;
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

fn kernel_log_info() {
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
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    kernel_log_info();
    heap_alloc::init_heap();
    trap::init();
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}