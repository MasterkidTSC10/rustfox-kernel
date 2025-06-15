#![allow(dead_code)]

extern crate alloc;

use core::mem::MaybeUninit;
use linked_list_allocator::LockedHeap;

const HEAP_SIZE: usize = 1024 * 1024; // 1 MiB heap
static mut HEAP_SPACE: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Initialize the heap allocator.
///
/// Call this early in your `_start()` function.
pub fn init_heap() {
    unsafe {
        ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

pub fn init_memory() {
    // Setup memory, page tables, etc
}
