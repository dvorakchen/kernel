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
//! ## 虚拟地址图例 (Sv39)
//!
//! 38------30|29------21|20------12|11-----------0
//!   VPN[2]     VPN[1]     VPN[0]     page offset (pgoff)
//!
//! ## 从虚拟地址(va)转换到物理地址(pa)的过程
//! 这里仅讨论 4KB 基础页 (不讨论大页)
//!
//! PAGESIZE = 4096
//! LEVELS = 3
//! PTESIZE = 8
//! i = LEVELS - 1 (即从 i=2 开始查找)
//!
//! :LOOP_START
//! // 1. 获取当前级别页表的物理基地址
//! let a = 当前的 ppn * PAGESIZE  // 第一次循环时，当前的 ppn 就是 satp.ppn
//!
//! // 2. 计算目标 PTE 在内存中的物理地址，并读取它的值
//! let pte_addr = a + va.vpn[i] * PTESIZE
//! let pte_val = read_memory(pte_addr) // 从内存中读出这个 64 位的结构
//!
//! // 3. 检查合法性
//! 如果 pte_val.v == 0 || (pte_val.r == 0 && pte_val.w == 1) {
//!     抛出 Page Fault 异常 (PTE 无效)
//! }
//!
//! // 4. 判断是否找到叶子节点 (真正的物理页映射)
//! 如果 pte_val.r == 1 || pte_val.x == 1 {
//!     // 找到叶子节点了！跳出循环，准备拼装最终物理地址
//!     goto :FINISH
//! }
//!
//! // 5. 如果还没找到，说明它是目录节点 (指向下一级页表)
//! i--
//! 如果 i < 0 {
//!     抛出 Page Fault 异常 (树走到底了还没找到叶子)
//! }
//! 当前的 ppn = pte_val.ppn // 更新 ppn 为下一级页表的物理页号
//! 回到 :LOOP_START
//!
//! :FINISH
//! // 6. 拼装最终的物理地址 (pa)
//! // 物理页号来自最后找到的那个叶子 PTE
//! pa.ppn = pte_val.ppn
//! // 页内偏移原封不动地抄虚拟地址的
//! pa.pgoff = va.pgoff
//!
//! pa = (pa.ppn * PAGESIZE) + pa.pgoff
//! 最终得到 pa 物理地址！

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

/// 页表项
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct PTE(usize);

impl From<usize> for PTE {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<PTE> for usize {
    fn from(value: PTE) -> Self {
        value.0
    }
}

impl PTE {
    pub fn is_valid(&self) -> bool {
        let flags = self.0 as u8;
        // According to the RISC-V spec, a PTE is invalid if V=0, or if R=0 and W=1.
        // A valid PTE must have V=1 and not be the invalid R=0, W=1 combination.
        let v_is_set = (flags & PTEFlags::V.bits()) != 0;
        let rw_is_invalid =
            (flags & (PTEFlags::R.bits() | PTEFlags::W.bits())) == PTEFlags::W.bits();
        v_is_set && !rw_is_invalid
    }

    pub fn ppn(&self) -> usize {
        (self.0 >> 10) & 0xFFF_FFFF_FFFF
    }
}

/// 页表
#[repr(transparent)]
pub struct PageTable([PTE; 512]);

/// 虚拟地址
#[repr(transparent)]
pub(crate) struct VirtualAddr(usize);

impl From<usize> for VirtualAddr {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<VirtualAddr> for usize {
    fn from(value: VirtualAddr) -> Self {
        value.0
    }
}

impl VirtualAddr {
    pub(crate) fn vpn(&self, index: usize) -> usize {
        if index > 2 {
            panic!("invalid VPN index: {}", index);
        }
        // Calculate the total shift amount: 12 bits for page offset + 9 bits per VPN level.
        let shift_amount = 12 + index * 9;
        (self.0 >> shift_amount) & 0x1FF
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum VMError {
    #[error("PTE invalid: {0}")]
    PTEInvalid(usize),
}

pub(crate) struct VM {
    root_ppn: usize,
}

impl VM {
    pub fn map(&mut self, va: VirtualAddr, pa: usize, flag: PTEFlags) -> Result<(), VMError> {
        // satp.ppn * 4096
        let mut a = self.root_ppn * 4096;

        for i in (0..3).rev() {
            let pte: PTE = (a + va.vpn(i) * 8).into();

            if !pte.is_valid() {
                return Err(VMError::PTEInvalid(pte.into()));
            }
            let ppn = pte.ppn();
            a = ppn * 4096;
        }

        // TODO: to pa
        Ok(())
    }

    pub fn enable_stap(&self) {}
}
