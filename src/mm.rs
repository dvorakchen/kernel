pub(crate) mod frame;
pub(crate) mod heap;

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

        let mut frame = crate::mm::frame::Frame::default();
        // 这里应该先分配物理内存到 frame 里面
        frame.add(0, 1);
        crate::println!("[MEMORY MANAGER] frame allco: {:?}", frame.alloc());
        crate::println!("[MEMORY MANAGER] frame allco: {:?}", frame.alloc());
        frame.dealloc(0);
        crate::println!("[MEMORY MANAGER] frame allco: {:?}", frame.alloc());
        Self {
            heap,
            device,
            frame,
        }
    }

    /// 启用虚拟页表
    pub fn enable_virtual_page(&mut self) {
        // 内核空间为恒等映射

        unimplemented!()
    }
}
