//! src/trap.rs
//!
//! 中断处理
//!

use core::arch::global_asm;

use riscv::register::{
    stvec::{self, Stvec, TrapMode},
    time,
};
use sbi_rt::{Timer, set_timer};

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

pub struct Trap;

impl Trap {
    /// init 初始化 trap
    pub fn init() {
        let st = Stvec::new(trap_entry as *const () as usize, TrapMode::Direct);
        unsafe { stvec::write(st) };
        crate::println!(
            "[TRAP] Handler installed at {:#x}",
            trap_entry as *const () as usize
        );
    }
    pub fn set_time_interrupt() {
        if sbi_rt::probe_extension(Timer).is_unavailable() {
            crate::println!("[SBI] Timer Extension unavailable");
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
}
