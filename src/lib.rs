#![no_std]

use crate::{arch::Arch, device::DeviceTree};

pub mod arch;
pub mod console;
pub mod device;
pub mod system;
pub mod trap;

pub struct Kernel {
    pub arch: Arch,
}

impl Kernel {
    pub fn new(dt: DeviceTree) -> Self {
        let arch = Arch::new(&dt);

        Self { arch }
    }
}
