#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use std::thread::sleep;

use user_lib::{get_time, yield_};

#[no_mangle]
fn main() -> i32 {
    let start = get_time();
    println!("current time_msec = {}", start);
    sleep(100);
    let end = get_time();
    println!(
        "time_msec = {} after sleeping 100 ticks, delta = {}ms!",
        end,
        end - start
    );
    println!("Test sleep1 passed!");
    0
}