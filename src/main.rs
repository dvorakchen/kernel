#![no_std]
#![no_main]

global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub extern "C" fn main(_hart_id: usize, _dtb_pa: usize) {
    Trap::init();
    Trap::set_time_interrupt();

    loop {
        wfi();
        kernel::println!("[Kernel] after wfi");
    }
}

use core::{arch::global_asm, panic::PanicInfo};
use kernel::trap::Trap;
use riscv::asm::wfi;

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    if let Some(location) = info.location() {
        kernel::println!("{}", location.file());
        kernel::println!("{}", location.line());
    }
    loop {}
}
