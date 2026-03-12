#![no_std]

use riscv::asm::wfi;

use crate::{arch::Arch, device::DeviceTree, mm::MemoryManager};

mod arch;
pub mod console;
pub mod device;
mod mm;
mod system;
mod trap;
mod utils;

unsafe extern "C" {
    fn skernel();
    fn ekernel();
    fn user_trap_entry();
    fn kernel_trap_entry();
}

pub struct Kernel {
    pub arch: Arch,
    pub device_tree: DeviceTree,
    pub(crate) mm: mm::MemoryManager,
}

impl Kernel {
    pub fn new(dtb_pa: usize) -> Self {
        let dt = device::DeviceTree::new(dtb_pa);
        let arch = Arch::new(&dt);

        Self {
            arch,
            mm: MemoryManager::new(dt.memory),
            device_tree: dt,
        }
    }

    pub fn run(self) -> ! {
        crate::trap::Trap::using_user_trap_handler();
        loop {
            wfi();
        }
    }
}
