pub(crate) mod heap;

#[derive(Default)]
pub struct MemoryManager {
    heap: crate::mm::heap::Heap,
}
