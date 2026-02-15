#![no_std]
#![no_main]

global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub extern "C" fn main(hart_id: usize, _dtb_pa: usize) {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }

    let ret = sbi_rt::get_spec_version();
    kernel::println!("SBI Version: {}", ret);

    kernel::println!("你好 Kernel");
    kernel::println!("sbss address: 0x{:x}", sbss as *const () as usize);
    kernel::println!("ebss address: 0x{:x}", ebss as *const () as usize);
    kernel::println!("hart id: 0x{:x}", hart_id);

    let ret = hart_get_status(hart_id);
    kernel::println!("hart status error: {}", ret.error);
    kernel::println!("hart status value: {}", ret.value);

    kernel::println!("try to get hart 1 status");
    let ret = hart_get_status(1);
    kernel::println!("hart status error: {}", ret.error);
    kernel::println!("hart status value: {}", ret.value);

    //kernel::device::parse_dtb(dtb_pa);

    //let c = read_char();
    //kernel::println!("read char: {}", c);

    let mut buf = [0u8; 128];
    let len = kernel::console::Stdin::read_line(&mut buf);
    let line = &buf[..len];
    let physical = Physical::new(line.len(), line.as_ptr() as usize, 0);
    let _sbiret = sbi_rt::console_write(physical);

    wfi();
}

use core::{arch::global_asm, panic::PanicInfo};
use riscv::asm::wfi;
use sbi_rt::{Physical, hart_get_status};

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}
