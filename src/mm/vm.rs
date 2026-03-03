//! # 虚拟内存 Sv39
//!
//! > riscv-privileged, Charpter 12.4
//!
//! SXLEN=64
//!
//! ## 异常
//! 从没有 X 权限的页中获取指令执行会发起一个 `fetch page-fault` 异常
//! 从没有 R 权限的页中加载数据会发起一个 `load page-fault` 异常
//! 从没有 W 权限的页中写入数据会发起一个 `store page-fault` 异常
//!
//!

use bitflags::bitflags;

pub(crate) struct VirtualMemory {}

bitflags! {
    /// 页表项的 0 到 7 位
    ///
    /// 如果 R,W,X 位都为 0，表示这个 PTE 指向下一级页表，
    /// 否则是叶节点 PTE
    #[derive(Clone, Copy, Debug)]
    pub(crate) struct PTEFlags: u8 {
        /// V 位，表示这个 PTE 是否有效
        /// 如果是 0，表示无效，PTE 的其他位都无效
        const V = 1 << 0;
        /// R 位，表示该页是否可写，可写的页也必须是可读的
        const R = 1 << 1;
        /// W 位，表示该页是否可读
        const W = 1 << 2;
        /// X 位，表示该页是否可执行
        const X = 1 << 3;
        /// U 位，表示 U-mode 是否可以访问该页
        const U = 1 << 4;
        /// G 位，表示是否全局映射
        const G = 1 << 5;
        /// A 位，表示在最后一次 A 位置 0 后，
        /// 该虚拟页表是否被读过、写过、取指过
        const A = 1 << 6;
        /// D 位，表示在最后一次 D 位置 0 后，
        /// 该虚拟页表是否被写过
        const D = 1 << 7;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct PTE(u64);

impl PTE {
    pub fn is_valid(&self) -> bool {
        (self.0 as u8 & PTEFlags::V.bits()) == PTEFlags::V.bits()
    }
}
