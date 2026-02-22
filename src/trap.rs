//! src/trap.rs
//!
//! 中断处理
//!

use core::arch::global_asm;

use riscv::register::{
    stvec::{self, Stvec, TrapMode},
    time,
};

global_asm!(include_str!("trap/trap64.s"));

unsafe extern "C" {
    fn trap_entry();
}

#[unsafe(no_mangle)]
pub fn handle_trap(sepc: usize, scause: usize, stval: usize, sstatus: usize) {
    crate::println!("sstatus: {:#x}", sstatus);
    crate::println!("sepc: {:#x}", sepc);
    crate::println!("scause: {:#x}", scause);
    crate::println!("stval: {:#x}", stval);

    crate::println!("time: {}", time::read());
    let stime_value = time::read64() + 10_000_000;
    sbi_rt::set_timer(stime_value);
}

/// init 初始化 trap
pub fn init() {
    let st = Stvec::new(trap_entry as *const () as usize, TrapMode::Direct);
    unsafe { stvec::write(st) };
    crate::println!(
        "[TRAP] Handler installed at {:#x}",
        trap_entry as *const () as usize
    );
}
