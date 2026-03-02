use buddy_system_allocator::*;

const INIT_KERNEL_HEAP_SIZE: usize = 4096;

static INIT_KERNEL_HEAP_SPACE: [u8; INIT_KERNEL_HEAP_SIZE] = [0; INIT_KERNEL_HEAP_SIZE];

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeapWithRescue<32> = LockedHeapWithRescue::new(|heap, layout| {
    crate::println!("[HEAP_ALLOCATOR] failed, layout: {:?}", layout);
});

#[derive(Default)]
pub(crate) struct Heap;
