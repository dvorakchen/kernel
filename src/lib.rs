#![no_std]

use riscv::asm::wfi;

use crate::{arch::Arch, device::DeviceTree, mm::MemoryManager};

mod arch;
pub mod console;
pub mod device;
pub mod mm;
mod system;
mod trap;
mod utils;

unsafe extern "C" {
    fn skernel();
    fn ekernel();
    fn user_trap_entry();
    fn kernel_trap_entry();
}

/// 内核
pub struct Kernel {
    pub arch: Arch,
    pub device_tree: DeviceTree,
    pub(crate) mm: mm::MemoryManager,
}

impl Kernel {
    /// 初始化内核
    pub fn new(dtb_pa: usize) -> Self {
        crate::trap::Trap::using_kernel_trap_handler();

        let dt = device::DeviceTree::new(dtb_pa);
        let arch = Arch::new(&dt);

        Self {
            arch,
            mm: MemoryManager::new(dt.memory),
            device_tree: dt,
        }
    }

    /// 启动内核
    pub fn run(self) -> ! {
        crate::println!("[KERNEL] running");
        crate::trap::Trap::using_kernel_trap_handler();
        // crate::trap::Trap::using_user_trap_handler();
        loop {
            wfi();
        }
    }
}
