//! 直接使用 SBI 操作
//! RISC-V SBI 规范文档查阅 [riscv-sbi-v2.0](https://docs.riscv.org/reference/platform-software/sbi/v2.0/_attachments/riscv-sbi.pdf)
//!
//! 这里使用了 crate: [sbi_rt](https://crates.io/crates/sbi_rt) 封装的 API 来方便的操作 SBI
//!
#![no_std]
#![no_main]

global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub extern "C" fn main(_hart_id: usize, _dtb_pa: usize) {
    unsafe extern "C" {
        static sbss: u8;
        static ebss: u8;
    }

    put_str("\x1b[2J\x1b[H");
    put_str("Hello SBI");
    put_str("\n");

    let mut buf = [0u8; 128];
    scan_line(&mut buf);
    wfi();
}

use core::{arch::global_asm, panic::PanicInfo};
use riscv::asm::wfi;
use sbi_rt::{Physical, SbiRet, hart_get_status};

fn put_str(s: &str) {
    let phy = Physical::new(s.len(), s.as_ptr() as usize, 0);
    sbi_rt::console_write(phy);
}

fn scan_line(buf: &mut [u8]) {
    let phy: Physical<&mut [u8]> = Physical::new(buf.len(), buf.as_ptr() as usize, 0);
    let ret = sbi_rt::console_read(phy);

    put_str("sbiret.error: ");
    put_num(ret.error);
    put_str("\n");

    put_str("sbiret.value: ");
    put_num(ret.value);
    put_str("\n")
}

fn get_char() {
    let phy: Physical<&mut [u8]> = Physical::new(buf.len(), buf.as_ptr() as usize, 0);
    let ret = sbi_rt::console_read(phy);
}

fn put_num(num: usize) {
    let mut num = num;
    if num == 0 {
        sbi_rt::console_write_byte(b'0');
        return;
    }

    // 1. 拆数字到缓冲区（逆序）
    let mut buf = [0u8; 10]; // u32 最多 10 位
    let mut i = 0;
    while num > 0 {
        buf[i] = (num % 10) as u8 + b'0';
        num /= 10;
        i += 1;
    }

    // 2. 逆序发送
    while i > 0 {
        sbi_rt::console_write_byte(buf[i - 1]);
        i -= 1;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}
