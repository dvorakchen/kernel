//! arch.rs
//!
//! 架构相关的信息
//!     - 64、32 位
//!

mod isa;
pub use isa::ISAExtensions;

use core::fmt::Display;

use crate::device::DeviceTree;

pub enum PointerWith {
    P32,
    P64,
}

pub struct Arch {
    pub pointer_width: PointerWith,
    pub sbi_specification_version: sbi_rt::Version,
    pub sbi_implementation_id: usize,
    pub sbi_implementation_version: usize,
    pub machine_vendor_id: usize,
    pub machine_architecture_id: usize,
    pub machine_implementation_id: usize,
    pub timebase_frequency: usize,
}

impl Arch {
    pub fn new(_dt: &DeviceTree) -> Self {
        Self {
            pointer_width: PointerWith::P64,
            sbi_specification_version: sbi_rt::get_spec_version(),
            sbi_implementation_id: sbi_rt::get_sbi_impl_id(),
            sbi_implementation_version: sbi_rt::get_sbi_impl_version(),
            machine_vendor_id: sbi_rt::get_mvendorid(),
            machine_architecture_id: sbi_rt::get_marchid(),
            machine_implementation_id: sbi_rt::get_mimpid(),
            timebase_frequency: 0,
        }
    }
}

impl Display for Arch {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            r#"SBI specification version: {}
SBI implementation id: {}
SBI implementation version: {}
machine vendor id: {}
machine architecture id: {},
machine implementation id: {}"#,
            self.sbi_specification_version,
            self.sbi_implementation_id,
            self.sbi_implementation_version,
            self.machine_vendor_id,
            self.machine_architecture_id,
            self.machine_implementation_id
        )
    }
}
