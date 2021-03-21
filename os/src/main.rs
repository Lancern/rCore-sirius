#![no_std]
#![no_main]

#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod sbi;

use core::panic::PanicInfo;

use crate::sbi::shutdown;

global_asm!(include_str!("entry.asm"));

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message().unwrap());
    shutdown();
}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort()")
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello rCore!");
    panic!("end of rust_main");
}
