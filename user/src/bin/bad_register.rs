#![no_std]
#![no_main]

use core::arch::asm;

extern crate user_lib;

#[no_mangle]
pub fn main() -> ! {
    let mut sstatus: usize;
    unsafe {
        asm!("csrr {}, sstatus", out(reg) sstatus); 
    }
    panic!("(-_-)I get sstatus:{:x}\nFAIL T.T\n", sstatus);
}