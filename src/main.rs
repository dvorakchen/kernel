#![no_std]
#![no_main]

global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub extern "C" fn main(hart_id: usize, dtb_pa: usize) {
    kernel::println!("dtb_pa: {:#x}", dtb_pa);
    let mut kernel = Kernel::new(dtb_pa);
    kernel.run();
}

fn set_time_interrupt() {
    if sbi_rt::probe_extension(Timer).is_unavailable() {
        kernel::println!("[SBI] Timer Extension unavailable");
        return;
    }

    //riscv::register::time;
    let stime_value = time::read64() + 10_000_000;
    set_timer(stime_value);
    use riscv::register::{sie, sstatus};
    unsafe {
        sie::set_stimer();
        sstatus::set_sie();
    }
}

use core::{arch::global_asm, panic::PanicInfo};
use kernel::{Kernel, device};
use riscv::{
    asm::wfi,
    interrupt::Trap,
    register::{sie::set_stimer, time},
};
use sbi_rt::{Physical, Timer, hart_get_status, set_timer};

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    if let Some(location) = info.location() {
        kernel::println!("{}", location.file());
        kernel::println!("{}", location.line());
    }
    loop {}
}
