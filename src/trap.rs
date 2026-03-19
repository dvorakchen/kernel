//! src/trap.rs
//!
//! 中断处理
//!

use core::arch::global_asm;

use riscv::register::{
    stvec::{self, Stvec, TrapMode},
    time,
};

global_asm!(include_str!("trap/kernel_trap64.s"));
global_asm!(include_str!("trap/user_trap64.s"));

/// S-mode 下的中断处理函数
#[unsafe(no_mangle)]
pub fn kernel_handle_trap(sepc: usize, scause: usize, stval: usize, sstatus: usize) {
    let _ = sepc;
    // crate::println!("sstatus: {:#x}", sstatus);
    // crate::println!("sepc: {:#x}", sepc);
    // crate::println!("scause: {:#x}", scause);
    // crate::println!("stval: {:#x}", stval);
    //
    // crate::println!("time: {}", time::read());
    let stime_value = time::read64() + 10_000_000;
    sbi_rt::set_timer(stime_value);
}

/// U-mode 下的中断处理函数
#[unsafe(no_mangle)]
pub fn user_handle_trap(sepc: usize, scause: usize, stval: usize, sstatus: usize) {
    Trap::using_kernel_trap_handler();

    crate::println!("sstatus: {:#x}", sstatus);
    crate::println!("sepc: {:#x}", sepc);
    crate::println!("scause: {:#x}", scause);
    crate::println!("stval: {:#x}", stval);

    crate::println!("time: {}", time::read());
    let stime_value = time::read64() + 10_000_000;
    sbi_rt::set_timer(stime_value);

    Trap::using_user_trap_handler();
}

pub(crate) struct Trap;

impl Trap {
    pub(crate) fn using_kernel_trap_handler() {
        let st = Stvec::new(
            crate::kernel_trap_entry as *const () as usize,
            TrapMode::Direct,
        );
        unsafe { stvec::write(st) };
        // crate::println!(
        //     "[TRAP] set kernel handler at {:#x}",
        //     crate::kernel_trap_entry as *const () as usize
        // );
    }

    pub(crate) fn using_user_trap_handler() {
        let st = Stvec::new(
            crate::user_trap_entry as *const () as usize,
            TrapMode::Direct,
        );
        unsafe { stvec::write(st) };
        crate::println!(
            "[TRAP] set user handler at {:#x}",
            crate::user_trap_entry as *const () as usize
        );
    }
}
