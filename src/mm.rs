pub(crate) mod frame;
pub(crate) mod heap;

/// 内存一页的大小
pub(crate) const PAGE_SIZE: usize = 4096;

/// 内存管理
///
/// 包括管理 内核堆、物理页帧、虚拟内存
pub(crate) struct MemoryManager {
    device: crate::device::Memory,
    pub heap: crate::mm::heap::Heap,
    pub frame: crate::mm::frame::Frame,
}

impl MemoryManager {
    pub fn new(device: crate::device::Memory) -> Self {
        let heap = crate::mm::heap::Heap::default();

        let frame = crate::mm::frame::Frame::default();

        let mut mm = Self {
            heap,
            device,
            frame,
        };

        mm.device_mem_2_heap();

        mm
    }

    /// 将内存分配到帧分配器里
    fn device_mem_2_heap(&mut self) {
        let free_memory_start = crate::ekernel as *const () as usize;
        crate::println!(
            "[MEMORY MANAGER] free memory start address: {:#x}",
            free_memory_start
        );
        let free_memory_start = crate::utils::align_top(free_memory_start, PAGE_SIZE);
        crate::println!(
            "[MEMORY MANAGER] free memory start address aligned: {:#x}",
            free_memory_start
        );

        let size = self.device.size / PAGE_SIZE;
        crate::println!(
            "[MEMORY MANAGER] Memory start address: {:#x}",
            self.device.start
        );
        crate::println!("[MEMORY MANAGER] Memory size: {:#x}", self.device.size);
        crate::println!("[MEMORY MANAGER] Memory page size: {:#x}", PAGE_SIZE);
        crate::println!("[MEMORY MANAGER] Memory page count: {:#x}", size);
        self.frame.add(0, size);
    }

    /// 启用虚拟页表
    pub fn enable_virtual_page(&mut self) {
        // 内核空间为恒等映射

        unimplemented!()
    }
}
