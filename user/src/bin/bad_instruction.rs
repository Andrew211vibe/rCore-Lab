#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[no_mangle]
pub fn main() -> ! {
    unsafe {
        asm!("sret");
    }
    panic!("FAIL: T.T\n");
}