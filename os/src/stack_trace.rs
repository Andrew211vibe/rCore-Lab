use core::{arch::asm, ptr};

pub unsafe fn print_stack_trace() {
    let mut fp: *const usize;
    asm!("mv {}, fp", out(reg) fp);

    println!("---Begin Stack Trace---");
    while fp != ptr::null() {
        let save_ra = *fp.sub(1);
        let save_fp = *fp.sub(2);

        println!("0x{:016x}, fp=0x{:016x}", save_ra, save_fp);

        fp = save_fp as *const usize;
    }
    println!("---End Stack Trace---");
}