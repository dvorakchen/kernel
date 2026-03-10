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
    fn trap_entry();
    fn ekernel();
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
        loop {
            wfi();
        }
    }
}
