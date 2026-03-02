#![no_std]

use riscv::asm::wfi;

use crate::{arch::Arch, device::DeviceTree, mm::MemoryManager};

pub mod arch;
pub mod console;
pub mod device;
mod mm;
pub mod system;
pub mod trap;

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

    pub fn run(mut self) -> ! {
        loop {
            wfi();
        }
    }
}
