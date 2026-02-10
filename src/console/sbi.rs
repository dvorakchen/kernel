use core::{
    char,
    fmt::{self, Write},
};
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
    ($fmt: literal $(, $($args:tt)+)?) => {
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

pub fn read_char() -> char {
    let buf = [0u8; 1];

    loop {
        let phy: Physical<&mut [u8]> = Physical::new(buf.len(), buf.as_ptr() as usize, 0);
        let ret = sbi_rt::console_read(phy);

        if ret.error == 0 && ret.value > 0 {
            return if let Some(c) = char::from_u32(buf[0] as u32) {
                c
            } else {
                ' '
            };
        }
        core::hint::spin_loop();
    }
}

pub fn read_line() -> [u8; 128] {
    let mut buf = [0u8; 128];

    let mut index = 0;

    loop {
        let c = read_char();

        match c {
            '\r' | '\n' => {
                buf[index] = '\n' as u8;
                sbi_println!("");
                break;
            }
            _ => {
                buf[index] = c as u8;
                index += 1;
                sbi_print!("{}", c);
            }
        }
    }

    buf
}
