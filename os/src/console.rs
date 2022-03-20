/*core::fmt::Write trait 包含一个用来实现 println! 宏很好用的 write_fmt 方法，
为此我们准备为结构体 Stdout 实现 Write trait 。
在 Write trait 中， write_str 方法必须实现，因此我们需要为 Stdout 实现这一方法 =》
它并不难实现，只需遍历传入的 &str 中的每个字符并调用 console_putchar 就能将传入的整个字符串打印到屏幕上。
*/
use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

//Write相当于接口，要求实现write_str方法
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!($fmt $(, $($arg)+)?)
        );
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(concat!($fmt, "\n") $(, $($arg)+)?)
        )
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(  concat!("\x1b[31m[ERROR]",concat!($fmt, "\n\x1b[0m"))   $(, $($arg)+)?)
        );
    }
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(  concat!("\x1b[34m[INFO]",concat!($fmt, "\n\x1b[0m"))   $(, $($arg)+)?)
        );
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(  concat!("\x1b[32m[DEBUG]",concat!($fmt, "\n\x1b[0m"))   $(, $($arg)+)?)
        );
    }
}

#[macro_export]
macro_rules! warning {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(  concat!("\x1b[93m[WARN]",concat!($fmt, "\n\x1b[0m"))   $(, $($arg)+)?)
        );
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(  concat!("\x1b[90m[TRACE]",concat!($fmt, "\n\x1b[0m"))   $(, $($arg)+)?)
        );
    }
}