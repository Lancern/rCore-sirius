#![allow(dead_code)]

unsafe fn sbi_call(which: isize, arg0: isize, arg1: isize, arg2: isize) -> isize {
    let ret;

    llvm_asm!("ecall"
        : "={x10}" (ret)
        : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
        : "memory"
        : "volatile"
    );

    ret
}

const SBI_SET_TIMER: isize = 0;
const SBI_CONSOLE_PUTCHAR: isize = 1;
const SBI_CONSOLE_GETCHAR: isize = 2;
const SBI_CLEAR_IPI: isize = 3;
const SBI_SEND_IPI: isize = 4;
const SBI_REMOTE_FENCE_I: isize = 5;
const SBI_REMOTE_SFENCE_VMA: isize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: isize = 7;
const SBI_SHUTDOWN: isize = 8;

pub fn console_putchar(c: u8) {
    unsafe {
        sbi_call(SBI_CONSOLE_PUTCHAR, c as isize, 0, 0);
    }
}

pub fn console_getchar() -> u8 {
    unsafe {
        sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0) as u8
    }
}

pub fn shutdown() -> ! {
    unsafe {
        sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    }
    unreachable!()
}
