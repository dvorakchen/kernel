#![no_std]

extern crate alloc;

use anyhow::Result;
use riscv::asm::wfi;

use crate::{arch::Arch, console::Stdin, device::DeviceTree, mm::MemoryManager, task::TaskManager};

mod arch;
pub mod console;
pub mod device;
pub mod mm;
mod regs;
mod system;
mod task;
mod trap;
mod utils;

unsafe extern "C" {
    // fn skernel();
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
    pub(crate) stdin: Stdin,
    pub(crate) task_manager: TaskManager,
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
            stdin: Stdin,
            task_manager: TaskManager::new(),
        })
    }

    /// 启动内核
    pub fn run(self) -> ! {
        let kernel_addr = &self as *const Self as usize;
        let stack_top = boot_stack_top as *mut u128;
        unsafe {
            *stack_top = kernel_addr as u128;
        };

        unsafe {
            crate::println!("[KERNEL] kernel struct addr: {:#x}", *stack_top);
        };

        crate::println!(
            "[KERNEL] set time interrppt, timebase_freq: {}",
            self.device_tree.cpu.timebase_freq
        );
        trap::Trap::set_time_interrupt(self.device_tree.cpu.timebase_freq);
        crate::println!("[KERNEL] running");

        let sp = regs::read_sp();
        crate::println!("[KERNEL] register sp: {:#x}", sp);

        loop {
            wfi();
        }
    }
}
