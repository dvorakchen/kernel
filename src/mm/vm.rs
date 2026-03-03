//! 虚拟内存 Sv39
//!

use bitflags::bitflags;

pub(crate) struct VirtualMemory {}

bitflags! {
    /// 页表项的 0 到 7 位
    #[derive(Clone, Copy, Debug)]
    pub(crate) struct Flags: u8 {
        /// V 位，表示这个 PTE 是否有效
        /// 如果是 0，表示无效，PTE 的其他位都无效
        const V = 1 << 0;
        ///
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}
