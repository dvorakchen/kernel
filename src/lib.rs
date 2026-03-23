#![no_std]

extern crate alloc;

use anyhow::Result;
use riscv::{asm::wfi, register::time};
use sbi_rt::{Timer, set_timer};

use crate::{arch::Arch, device::DeviceTree, mm::MemoryManager};

mod arch;
pub mod console;
pub mod device;
pub mod mm;
mod regs;
mod system;
mod trap;
mod utils;

unsafe extern "C" {
    fn skernel();
    fn ekernel();
    fn user_trap_entry();
    fn kernel_trap_entry();
    fn boot_stack_top();
}

/// 内核
pub struct Kernel {
    pub arch: Arch,
    pub device_tree: DeviceTree,
    pub(crate) mm: mm::MemoryManager,
}

impl Kernel {
    /// 初始化内核
    pub fn new(dtb_pa: usize) -> Result<Self> {
        // 使用内核中断
        // 解析设备树信息

        crate::trap::Trap::using_kernel_trap_handler();

        let dt = device::DeviceTree::new(dtb_pa)?;
        let arch = Arch::new(&dt);

        Ok(Self {
            arch,
            mm: MemoryManager::new(dt.memory),
            device_tree: dt,
        })
    }

    /// 启动内核
    pub fn run(self) -> ! {
        crate::println!("[KERNEL] set time interrppt");
        self.set_time_interrupt();
        crate::println!("[KERNEL] running");

        let sp = regs::read_sp();
        crate::println!("[KERNEL] register sp: {:#x}", sp);

        loop {
            wfi();
        }
    }

    /// 设置时钟中断，每秒触发
    fn set_time_interrupt(&self) {
        if sbi_rt::probe_extension(Timer).is_unavailable() {
            crate::println!("[SBI] Timer Extension unavailable");
            return;
        }

        //riscv::register::time;
        let stime_value = time::read64() + self.device_tree.cpu.timebase_freq;
        set_timer(stime_value);
        use riscv::register::{sie, sstatus};
        unsafe {
            sie::set_stimer();
            sstatus::set_sie();
        }
    }
}
