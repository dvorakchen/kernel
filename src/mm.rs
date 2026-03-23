//! # 内存管理
//!
//! Author: Dvorak
//! Create Date: 2026-03-02
//! Last Update Date: 2026-03-03
//!
//! - 内核堆管理
//! - 物理页帧管理
//! - 虚拟内存管理
//!
//! ## 内核堆管理
//!
//! 使用 Rust 语言实现的裸机代码没有**全局分配器**，无法使用想 `Vec`、`Box` 等数据结构
//! 所以必须给它一个全局分配器的实现。
//! 这里用的是 `buddy_system_allocator`，提供了全局分配器和页帧分配器，
//! 这里的页帧分配器依赖全局分配器，必须先有了全局分配器了才能使用页帧分配器
//!
//! 初始内核堆在内核的 `bss` 段内，初始堆大小为 4KB，刚好是一页大小
//! 初始化内核堆阶段没有开启页表，在开启页表后，内核的地址做恒等映射
//!
//! ## 物理页帧管理
//!
//! 把所有可用的物理页按照 4KB 对齐，分为许多 4KB 大小的帧
//! 当内核需要一个物理页时候，给它一个没有使用的物理页帧的开始地址
//!
//! 这里的物理页帧分配器使用的是 `buddy_system_allocator` 的 `LockedFrameAllocator`
//!
//! ## 虚拟内存管理
//!
//! 开启页表

use riscv::register::satp;

use crate::mm;

pub(crate) mod frame;
pub(crate) mod heap;
pub(crate) mod vm;

pub const PA2VA_OFFSET: usize = 0xFFFF_FFFF_0000_0000;

/// 将物理地址转为内核能通过页表访问的虚拟地址
///
/// 当开启了页表，内核在虚拟地址中被放到了高半区，
/// 所以内核所有的地址都需要以高半区的虚拟地址来访问
#[inline]
pub const fn phys_2_virt(pa: usize) -> usize {
    pa + PA2VA_OFFSET
}

/// 内存一页的大小
pub(crate) const PAGE_SIZE: usize = 4096;

/// 内存管理
///
/// 包括管理 内核堆、物理页帧、虚拟内存
pub(crate) struct MemoryManager {
    satp: usize,
    device: crate::device::Memory,
    pub heap: crate::mm::heap::HeapAllocator,
    pub frame: crate::mm::frame::FrameAllocator,
}

impl MemoryManager {
    pub fn new(device: crate::device::Memory) -> Self {
        let heap = crate::mm::heap::HeapAllocator::default();
        let mut frame = crate::mm::frame::FrameAllocator::default();

        Self::device_mem_2_frame(&device, &mut frame);

        let satp = satp::read().bits();
        let mm = Self {
            satp,
            heap,
            device,
            frame,
        };

        crate::println!("[MEMORY MANAGER] satp: {:#x}", satp);

        mm
    }

    /// 将空闲内存分配到帧分配器里
    ///
    /// 这个函数会在帧分配器初始化后马上使用，
    /// 此时除了内核以外的内存都是空闲内存
    fn device_mem_2_frame(
        device: &crate::device::Memory,
        frame_alloc: &mut mm::frame::FrameAllocator,
    ) {
        let start = crate::utils::align_top(crate::ekernel as *const () as usize, PAGE_SIZE);
        crate::println!(
            "[MEMORY MANAGER] free memory start address aligned: {:#x}",
            start
        );
        let size = device.size - (start - device.start);

        let end = crate::utils::align_bottom(start + size, PAGE_SIZE);
        crate::println!(
            "[MEMORY MANAGER] free memory end address aligned: {:#x}",
            end
        );
        crate::println!("[MEMORY MANAGER] free memory size: {:#x}", size);

        let page_count = size << 12;
        crate::println!("[MEMORY MANAGER] Memory start address: {:#x}", device.start);
        crate::println!("[MEMORY MANAGER] Memory end address: {:#x}", end);
        crate::println!("[MEMORY MANAGER] Memory size: {:#x}", device.size);
        crate::println!("[MEMORY MANAGER] Memory page size: {:#x}", PAGE_SIZE);
        crate::println!("[MEMORY MANAGER] Memory page count: {:#x}", page_count);

        frame_alloc.set_start(start);
        frame_alloc.add(0, size / frame::FRAME_SIZE);
    }
}
