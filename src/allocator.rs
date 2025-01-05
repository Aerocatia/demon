//! Simple allocator implementation that directly maps to HeapAlloc, etc.

use core::alloc::{GlobalAlloc, Layout};
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Memory::{GetProcessHeap, HeapAlloc, HeapFree, HeapReAlloc, HEAP_ZERO_MEMORY};

pub struct Allocator;

#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        allocate(layout.size(), false)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        deallocate(ptr);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        allocate(layout.size(), true)
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        reallocate(ptr, new_size)
    }
}

unsafe fn get_heap() -> HANDLE {
    let heap = GetProcessHeap();
    assert!(!heap.is_null(), "Heap is NULL!");
    heap
}

unsafe fn allocate(size: usize, zeroed: bool) -> *mut u8 {
    HeapAlloc(get_heap(), if zeroed { HEAP_ZERO_MEMORY } else { 0 }, size) as *mut _
}

unsafe fn deallocate(what: *mut u8) {
    if what.is_null() {
        return
    }
    HeapFree(get_heap(), 0, what as *const _);
}

unsafe fn reallocate(what: *mut u8, new_size: usize) -> *mut u8 {
    HeapReAlloc(get_heap(), 0, what as *const _, new_size) as *mut _
}