use core::fmt::{Arguments, Write};
use crate::sbi::console_putchar;

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.bytes() {
            console_putchar(ch);
        }
        Ok(())
    }
}

pub fn print_args(args: Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print_args(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print_args(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
