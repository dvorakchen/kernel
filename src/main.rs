#![no_std]
#![no_main]

global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub extern "C" fn main(_hart_id: usize, dtb_pa: usize) {
    clear_bss();
    set_base_page_table();

    let dtb_pa = kernel::mm::phys_2_virt(dtb_pa);
    kernel::println!("dtb_pa: {:#x}", dtb_pa);
    let kernel = Kernel::new(dtb_pa).expect("[KERNEL] kernel error");
    kernel.run();
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

fn set_base_page_table() {
    unsafe extern "C" {
        static mut boot_page_table: [u64; 512];
    }

    unsafe {
        // 拆桥，抹除地位恒等映射
        boot_page_table[2] = 0;
        // 建桥，映射 MMIO 外设区域
        // 虚拟地址 0xFFFF_FFFF_0000_0000
        // 物理地址 0x0000_0000
        boot_page_table[508] = 0xE7;
        asm!("SFENCE.VMA");
    }
}

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
    slice,
};
use kernel::Kernel;
use riscv::asm::wfi;

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    if let Some(location) = info.location() {
        kernel::println!("[PANIC OCURRED]: ");
        kernel::println!("{}", location.file());
        kernel::println!("{}", location.line());
    }
    loop {
        wfi();
    }
}
