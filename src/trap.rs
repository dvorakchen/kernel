//! src/trap.rs
//!
//! 中断处理
//!

use core::arch::global_asm;

use riscv::register::{
    stvec::{self, Stvec, TrapMode},
    time,
};
use sbi_rt::set_timer;

use crate::Kernel;

global_asm!(include_str!("trap/kernel_trap64.s"));
global_asm!(include_str!("trap/user_trap64.s"));

/// S-mode 下的中断处理函数
#[unsafe(no_mangle)]
pub fn kernel_handle_trap(sepc: usize, scause: usize, stval: usize, sstatus: usize) {
    unsafe extern "C" {
        fn boot_stack_top();
    }

    let stack_top_addr = boot_stack_top as *const () as usize;
    let kernel_addr = unsafe {
        let kernel_addr = *(stack_top_addr as *const u128);
        crate::println!("[TRAP] kernel addr: {:#x}", kernel_addr);
        kernel_addr
    };
    let kernel = unsafe { &*(kernel_addr as *const Kernel) };
    let t = kernel.device_tree.cpu.timebase_freq;
    crate::println!("[TRAP] cpu time_freq: {}", t);

    crate::println!("sstatus: {:#x}", sstatus);
    crate::println!("sepc: {:#x}", sepc);
    crate::println!("scause: {:#x}", scause);
    crate::println!("stval: {:#x}", stval);
    crate::println!("boot_stack_top: {:#x}", stack_top_addr);

    crate::println!("time: {}", time::read());
    crate::println!("");
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
    /// 使用内核中断处理函数
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

    /// 设置时钟中断，每秒触发
    pub(crate) fn set_time_interrupt(timebase_freq: u64) {
        if sbi_rt::probe_extension(sbi_rt::Timer).is_unavailable() {
            crate::println!("[SBI] Timer Extension unavailable");
            return;
        }

        //riscv::register::time;
        let stime_value = time::read64() + timebase_freq;
        set_timer(stime_value);
        use riscv::register::{sie, sstatus};
        unsafe {
            sie::set_stimer();
            sstatus::set_sie();
        }
    }
}
