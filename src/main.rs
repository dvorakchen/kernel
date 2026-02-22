#![no_std]
#![no_main]

global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub extern "C" fn main(hart_id: usize, dtb_pa: usize) {
    let dt = device::DeviceTree::new(dtb_pa);
    let kernel = Kernel::new(dt);

    //::kernel::println!("{}", kernel.arch);

    /*
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
    */
    //  kernel::println!("[Trap] start installing Trap");
    //kernel::trap::init();
    //kernel::println!("[Trap] installed");

    //    set_time_interrupt();
    wfi();
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
