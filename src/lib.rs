#![no_std]

use riscv::asm::wfi;

use crate::{arch::Arch, device::DeviceTree};

pub mod arch;
pub mod console;
pub mod device;
pub mod system;
pub mod trap;

pub struct Kernel {
    pub arch: Arch,
    pub device_tree: DeviceTree,
}

impl Kernel {
    pub fn new(dtb_pa: usize) -> Self {
        let dt = device::DeviceTree::new(dtb_pa);
        let arch = Arch::new(&dt);

        Self {
            arch,
            device_tree: dt,
        }
    }

    pub fn run(mut self) -> ! {
        loop {
            wfi();
        }
    }
}
