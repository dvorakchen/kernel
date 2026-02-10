#![no_std]
#![no_main]

global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub extern "C" fn main(hart_id: usize, _dtb_pa: usize) {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }

    kernel::sbi_println!("你好 Kernel");
    kernel::sbi_println!("sbss address: 0x{:x}", sbss as *const () as usize);
    kernel::sbi_println!("ebss address: 0x{:x}", ebss as *const () as usize);
    kernel::sbi_println!("hart id: 0x{:x}", hart_id);

    let ret = hart_get_status(hart_id);
    kernel::sbi_println!("hart status error: {}", ret.error);
    kernel::sbi_println!("hart status value: {}", ret.value);

    kernel::sbi_println!("try to get hart 1 status");
    let ret = hart_get_status(1);
    kernel::sbi_println!("hart status error: {}", ret.error);
    kernel::sbi_println!("hart status value: {}", ret.value);

    //kernel::device::parse_dtb(dtb_pa);

    //let c = read_char();
    //kernel::sbi_println!("read char: {}", c);

    let line = kernel::console::sbi::read_line();
    let physical = Physical::new(line.len(), line.as_ptr() as usize, 0);
    let _sbiret = sbi_rt::console_write(physical);

    wfi();
}

use core::{arch::global_asm, panic::PanicInfo};
use kernel::console::sbi::read_char;
use riscv::asm::wfi;
use sbi_rt::{Physical, hart_get_status};

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}
