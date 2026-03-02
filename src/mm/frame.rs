use buddy_system_allocator::*;

/// 物理页帧分配器
///
/// 这个只是个分配器，它能使用多少的物理内存，由调用者决定
#[derive(Default)]
pub(crate) struct Frame {
    allocator: LockedFrameAllocator,
}

impl Frame {
    pub(crate) fn add(&mut self, start: usize, end: usize) {
        self.allocator.lock().add_frame(start, end);
    }

    pub(crate) fn alloc(&mut self) -> Option<usize> {
        self.allocator.lock().alloc(1)
    }

    pub(crate) fn dealloc(&mut self, frame_number: usize) {
        self.allocator.lock().dealloc(frame_number, 1);
    }
}
