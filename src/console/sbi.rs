use core::fmt::{self, Write};
use sbi_rt::{Physical, console_write};

pub struct SbiStdout;

impl Write for SbiStdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let physical = Physical::new(s.len(), s.as_ptr() as usize, 0);
        let _sbiret = console_write(physical);

        Ok(())
    }
}

pub fn sbi_print(args: fmt::Arguments) {
    write!(SbiStdout, "{}", args).expect("sbi_print failed");
}

#[macro_export]
macro_rules! sbi_print {
    ($fmt: literal $(, $args:tt+)?) => {
        $crate::console::sbi::sbi_print(format_args!($fmt $(, $($args)+)?))
    };
}

#[macro_export]
macro_rules! sbi_println {
    () => ($crate::sbi_print!("\n"));
    ($fmt: literal $(, $($args:tt)+)?) => {
        $crate::console::sbi::sbi_print(format_args!(concat!($fmt, "\n") $(, $($args)+)?))
    };
}
