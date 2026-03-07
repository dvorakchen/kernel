use buddy_system_allocator::*;

const INIT_KERNEL_HEAP_SIZE: usize = 4096;

static INIT_KERNEL_HEAP_SPACE: [u8; INIT_KERNEL_HEAP_SIZE] = [0; INIT_KERNEL_HEAP_SIZE];

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeapWithRescue<32> = LockedHeapWithRescue::new(|_heap, layout| {
    crate::println!("[HEAP_ALLOCATOR] failed, layout: {:?}", layout);
});

/// 内核堆分配器
///
/// 现有了对分配器后才能使用 `String`, `Vec`，'Box' 等
pub(crate) struct HeapAllocator;

impl Default for HeapAllocator {
    fn default() -> Self {
        let start = INIT_KERNEL_HEAP_SPACE.as_ptr() as usize;

        crate::println!("[HEAP_ALLOCATOR] init space start: {:#x}", start);
        crate::println!(
            "[HEAP_ALLOCATOR] init space size: {:#x}",
            INIT_KERNEL_HEAP_SIZE
        );

        unsafe {
            HEAP_ALLOCATOR.lock().init(start, INIT_KERNEL_HEAP_SIZE);
        }

        HeapAllocator
    }
}
