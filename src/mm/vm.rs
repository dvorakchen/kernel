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
//!   VirtualPageN[2]     VirtualPageN[1]     VirtualPageN[0]     page offset (pgoff)
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
//!
//! ## 名词解释
//!
//! - ppn   物理页号 Physical Page Number,
//!         在 PET，va 中的 ppn 是截掉了低 12 位的，
//!         所以在和 offset 组合成物理地址的时候要先左移动 12 位，在加上 offset
//!         pv = (ppn << 12) + offset

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

impl PTEFlags {
    #[inline]
    pub(crate) fn PTE() -> Self {
        PTEFlags::V
    }
}

/// PTE 大小，8 byte
const PTE_SIZE: usize = 8;

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
    pub(crate) fn new(ppn: usize, flag: PTEFlags) -> Self {
        (ppn << 10 | flag.bits() as usize).into()
    }

    pub(crate) fn set(&mut self, pte: usize) {
        self.0 = pte;
    }

    pub(crate) fn ppn(&self) -> usize {
        (self.0 >> 10) & ((1 << 44) - 1)
    }

    pub fn v(&self) -> bool {
        self.0 & 0x01 == 1
    }

    pub fn is_valid(&self) -> bool {
        let flags = self.0 as u8;
        // According to the RISC-V spec, a PTE is invalid if V=0, or if R=0 and W=1.
        // A valid PTE must have V=1 and not be the invalid R=0, W=1 combination.
        let v_is_set = (flags & PTEFlags::V.bits()) != 0;
        let rw_is_invalid =
            (flags & (PTEFlags::R.bits() | PTEFlags::W.bits())) == PTEFlags::W.bits();
        v_is_set && !rw_is_invalid
    }

    pub(crate) fn next_page_table(&self) -> PageTable {
        (self.ppn() << 12).into()
    }
}

/// 页表
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub(crate) struct PageTable(usize);

impl From<usize> for PageTable {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl PageTable {
    pub(crate) fn nth_mut(&self, index: usize) -> &mut PTE {
        let t = (self.0 + index * PTE_SIZE) as *mut PTE;
        unsafe { &mut *t }
    }
}

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

/// 物理地址
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub(crate) struct PhysicalAddr(usize);

impl From<usize> for PhysicalAddr {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<PhysicalAddr> for usize {
    fn from(value: PhysicalAddr) -> Self {
        value.0
    }
}

impl VirtualAddr {
    pub(crate) fn vpn(&self, index: usize) -> usize {
        if index > 2 {
            panic!("invalid VirtualPageN index: {}", index);
        }
        // Calculate the total shift amount: 12 bits for page offset + 9 bits per VirtualPageN level.
        let shift_amount = 12 + index * 9;
        (self.0 >> shift_amount) & 0x1FF
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum VirtualPageError {
    #[error("PTE invalid: {0}")]
    InvalidPTE(usize),
    #[error("va map pa invalid: {0} -> {1}")]
    InvalidMap(usize, usize),
    #[error("has not more free frame")]
    NotMoreFreeFrame,
    #[error("used map: {0} -> {1}")]
    UsedMap(usize, usize),
}

pub(crate) enum PageType {
    /// 4kb 页
    Small,
    /// 2mb
    Big,
    /// 1g
    Super,
}

/// 虚拟页表
pub(crate) struct VirtualPage {
    /// 根页表的物理地址，也就是 L2 页表
    root_pt_addr: usize,
    page_type: PageType,
}

impl VirtualPage {
    /// new 一个新的虚拟页表
    ///
    /// ## Argument:
    /// - root_pt_addr: 根页表的物理地址
    pub fn new(root_pt_addr: usize, page_type: PageType) -> Self {
        Self {
            root_pt_addr,
            page_type,
        }
    }
}

impl VirtualPage {
    /// 映射内核空间
    ///
    /// 恒等映射，大页
    ///
    /// # Arguments:
    /// - pa_start: 内核空间开始物理地址
    /// - pa_end: 内核空间结束物理地址
    ///
    /// 调用者自行保证 2m 对齐
    pub(crate) fn map_kernel(
        &mut self,
        pa_start: PhysicalAddr,
        pa_end: PhysicalAddr,
        frame_alloc: &mut super::frame::FrameAllocator,
    ) -> Result<(), VirtualPageError> {
        const PAGE_SIZE_2M: usize = 0x20_0000;
        let start_addr = pa_start.0;
        let end_addr = pa_end.0;

        let flags = PTEFlags::V
            | PTEFlags::R
            | PTEFlags::W
            | PTEFlags::X
            | PTEFlags::G
            | PTEFlags::A
            | PTEFlags::D;

        let mut cur = start_addr;
        while cur < end_addr {
            let va: VirtualAddr = cur.into();
            let pa: PhysicalAddr = cur.into();

            self.map_2m_page(va, pa, flags, frame_alloc)?;

            cur += PAGE_SIZE_2M;
        }

        Ok(())
    }

    fn map_2m_page(
        &self,
        va: VirtualAddr,
        pa: PhysicalAddr,
        flags: PTEFlags,
        frame_alloc: &mut super::frame::FrameAllocator,
    ) -> Result<(), VirtualPageError> {
        let vpn2 = va.vpn(2);
        let vpn1 = va.vpn(1);

        let root_pt: PageTable = (self.root_pt_addr).into();
        let l2_pte = root_pt.nth_mut(vpn2);
        //let mut l2_pte = { root_pt.nth(vpn2) };

        if !l2_pte.v() {
            let frame = frame_alloc
                .alloc()
                .ok_or(VirtualPageError::NotMoreFreeFrame)?;
            let l1_ppn = frame.ppn();
            l2_pte.set((l1_ppn << 10) | PTEFlags::V.bits() as usize);
            //root_pt.set_PTE(vpn2, l2_pte);
        }

        let l1_table = l2_pte.next_page_table();
        //let mut l1_pte = l1_table.nth(vpn1);
        let l1_pte = l1_table.nth_mut(vpn1);
        // 在 2m 大页中，ppn[0] 必须为 0
        // 先右移21位，再左移19位，以让低10位为0
        l1_pte.set((pa.0 >> 21 << 19) | flags.bits() as usize);
        //l1_table.set_PTE(vpn1, l1_pte);

        Ok(())
    }
    /*
        /// 将一个 虚拟地址 映射到一个 物理地址
        ///
        /// 实际上就是在页表里添加页表项
        pub fn map_small(
            &mut self,
            va: VirtualAddr,
            pa: PhysicalAddr,
            flag: PTEFlags,
            frame_alloc: &mut super::frame::FrameAllocator,
        ) -> Result<(), VirtualPageError> {
            if va.0 & 0xFFF != pa.0 & 0xFFF {
                return Err(VirtualPageError::InvalidMap(va.0, pa.0));
            }
            // satp.ppn * 4096
            let pt_addr = self.root_pt_addr;
            let mut page_table: PageTable = pt_addr.into();

            for i in (0..3).rev()
            /* 2, 1, 0 */
            {
                let vpn = va.vpn(i);
                let mut pte = page_table.nth(vpn);

                if !pte.is_valid() {
                    let frame = frame_alloc
                        .alloc()
                        .ok_or(VirtualPageError::NotMoreFreeFrame)?;
                    let pte_ppn: usize = (Into::<usize>::into(frame)) >> 12;

                    if i == 0 {
                        // 叶子节点
                        let pa: usize = pa.into();
                        pte = PTE::new(pa >> 12, flag);
                    } else {
                        // PTE 节点
                        pte = PTE::new(pte_ppn, PTEFlags::PTE());
                    }
                    page_table.set_PTE(vpn, pte);
                }

                if i != 0 {
                    page_table = pte.next_page_table();
                }
            }

            Ok(())
        }
    */
    pub fn enable_stap(&self) {}
}
