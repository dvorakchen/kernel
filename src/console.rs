//! console.rs
//!
//! 使用 UART 串口通信向终端输出和读取字符

use core::fmt::{self, Write};

const UART_BASE: usize = 0x1000_0000;
const UART_RBR: *const u8 = (UART_BASE + 0x00) as *const u8;
const UART_THR: *mut u8 = (UART_BASE + 0x00) as *mut u8; // Transmitter Holding Register（发送寄存器）
const UART_LSR: *const u8 = (UART_BASE + 0x05) as *const u8; // Line Status Register（状态寄存器）
const LSR_DATA_READY: u8 = 1 << 0;
const LSR_TX_READY: u8 = 1 << 5; // bit 5 = 1 表示可以安全写入LSR_TX_READY: u8 = 1 << 5; // bit 5 = 1 表示可以安全写入

pub struct Stdin;

impl Stdin {
    pub fn read() -> u8 {
        while unsafe { core::ptr::read_volatile(UART_LSR) } & LSR_DATA_READY == 0 {
            core::hint::spin_loop();
        }
        // 从 RBR 读取（地址 0x00 的读操作）
        unsafe { core::ptr::read_volatile(UART_RBR) }
    }
    pub fn read_line(buf: &mut [u8]) -> usize {
        let mut i = 0;

        while i < buf.len() {
            match Self::read() {
                b'\n' | b'\r' => {
                    buf[i] = b'\n';
                    Stdout::write_char('\n');
                    return i;
                }
                b'\x08' | b'\x7f' => {
                    if i > 0 {
                        i -= 1;
                        Stdout::write_char('\x08');
                        Stdout::write_char(' ');
                        Stdout::write_char('\x08');
                    }
                }
                c => {
                    buf[i] = c;
                    i += 1;
                    Stdout::write_char(c as char);
                }
            }
        }
        i
    }
}

pub struct Stdout;

impl Stdout {
    pub fn clean() {}

    pub fn write_char(c: char) {
        let mut buf = [0u8; 4];
        let bytes = c.encode_utf8(&mut buf).as_bytes();

        bytes.iter().for_each(|&b| {
            Self::write_byte(b);
        });
    }

    fn write_byte(byte: u8) {
        // 等待发送缓冲区空（LSR bit5 = 1）
        while unsafe { core::ptr::read_volatile(UART_LSR) } & LSR_TX_READY == 0 {
            core::hint::spin_loop();
        }
        // 写入 THR（必须用 volatile 防止编译器优化掉）
        unsafe { core::ptr::write_volatile(UART_THR, byte) };
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        for &b in bytes {
            Self::write_byte(b);
        }

        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    write!(Stdout, "{}", args).expect("print failed");
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($args:tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($args)+)?))
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt: literal $(, $($args:tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($args)+)?))
    };
}
