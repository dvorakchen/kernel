#![no_std]
#![no_main]

global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub extern "C" fn main(_hart_id: usize, dtb_pa: usize) {
    clear_bss();
    clear_temp_pte();

    // kernel::println!("dtb_pa: {:#x}", dtb_pa);
    // let kernel = Kernel::new(dtb_pa);
    // kernel.run();
}

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }

    let space = unsafe {
        let sbss_addr = sbss as *const () as usize;
        let sbss = sbss_addr as *mut u8;
        let ebss = ebss as *const () as usize;
        slice::from_raw_parts_mut(sbss, ebss - sbss_addr)
    };
    space.fill(0);
}

fn clear_temp_pte() {
    unsafe extern "C" {
        static mut boot_page_table: [u64; 512];
    }

    unsafe {
        boot_page_table[2] = 0;
        asm!("SFENCE.VMA");
    }
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

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
    slice,
};
use kernel::Kernel;
use riscv::register::time;
use sbi_rt::{Timer, set_timer};

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    if let Some(location) = info.location() {
        kernel::println!("{}", location.file());
        kernel::println!("{}", location.line());
    }
    loop {}
}
