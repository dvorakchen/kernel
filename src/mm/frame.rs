use buddy_system_allocator::*;

/// 一个物理页帧的大小，4096 byte
pub(crate) const FRAME_SIZE: usize = 4096;

/// 物理页帧分配器
///
/// 这个只是个分配器，它能使用多少的物理内存，由调用者决定
#[derive(Default)]
pub(crate) struct FrameAllocator {
    start_addr: usize,
    allocator: LockedFrameAllocator,
}

impl FrameAllocator {
    /// 获取物理页帧的起始地址
    ///
    /// 由于物理页帧号是一个代表是**第几个**页帧的数字
    /// 需要乘以页帧大小再加上物理页帧的开始地址才能得出正确的物理地址
    ///
    /// > addr = （frame_number * FRAME_SIZE) + start_addr;
    ///
    /// 这样，如果只想让物理页帧管理器管理内核结束后的内存，只需要将 `start_addr` 设置为 `ekernel`
    pub(crate) fn set_start(&mut self, start_addr: usize) {
        self.start_addr = start_addr;
    }

    /// 添加可使用的帧
    ///
    /// 添加的帧包含头不包含尾
    ///
    /// # Arguments:
    /// - start: 开始的帧号
    /// - end: 结束的帧号（不包含尾）
    ///
    /// # Example:
    ///
    /// ```
    /// let mut frame = crate::mm::frame::FrameAllocator::default();
    /// frame.add(0, 3);
    ///
    /// // 添加了帧 0 到 3，但不包含 3，所以实际有效是 0, 1, 2
    /// assert!(frame.alloc().is_some());
    /// assert!(frame.alloc().is_some());
    /// assert!(frame.alloc().is_some());
    /// assert!(frame.alloc().is_none());
    /// ```
    pub(crate) fn add(&mut self, start: usize, end: usize) {
        self.allocator.lock().add_frame(start, end);
    }

    /// 获取一个空闲的物理页帧
    pub(crate) fn alloc(&mut self) -> Option<Frame> {
        self.allocator.lock().alloc(1).map(|frame_number| {
            crate::println!("[FRAME] alloc frame number: {:#x}", frame_number);
            (self.start_addr + frame_number * FRAME_SIZE).into()
        })
    }

    /// 回收一个物理页帧
    pub(crate) fn dealloc(&mut self, frame: Frame) {
        let frame: usize = frame.into();
        let frame_number = (frame - self.start_addr) / FRAME_SIZE;
        crate::println!("[FRAME] dealloc frame number: {:#x}", frame_number);
        self.allocator.lock().dealloc(frame_number, 1);
    }
}

/// 这是一个物理帧
///
/// 代表一个物理页的开始，一个物理页帧 4KB 大小
/// 是一个直接可用的物理地址
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub(crate) struct Frame(usize);

impl From<usize> for Frame {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Frame> for usize {
    fn from(value: Frame) -> Self {
        value.0
    }
}
