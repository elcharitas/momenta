#![no_std]
#![no_main]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

unsafe extern "C" {
    static _stack_start: u8;
    static _sidata: u8;
    static _sdata: u8;
    static _edata: u8;
    static _sbss: u8;
    static _ebss: u8;
}

#[repr(C)]
struct VectorTable {
    stack: *const u8,
    reset: extern "C" fn() -> !,
}

unsafe impl Sync for VectorTable {}

#[unsafe(link_section = ".vector_table")]
#[used]
static VECTORS: VectorTable = VectorTable {
    stack: core::ptr::addr_of!(_stack_start),
    reset,
};

const HEAP_SIZE: usize = 32 * 1024;

#[repr(align(16))]
struct Heap([u8; HEAP_SIZE]);

static mut HEAP: Heap = Heap([0; HEAP_SIZE]);
static NEXT: AtomicUsize = AtomicUsize::new(0);

struct BumpAllocator;

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Taking the heap address does not create a reference.
        let base = unsafe { core::ptr::addr_of_mut!(HEAP.0).cast::<u8>() as usize };
        loop {
            let current = NEXT.load(Ordering::Relaxed);
            let Some(start) = base
                .checked_add(current)
                .and_then(|address| address.checked_add(layout.align() - 1))
                .map(|address| address & !(layout.align() - 1))
            else {
                return null_mut();
            };
            let Some(end) = start.checked_add(layout.size()) else {
                return null_mut();
            };
            let used = end - base;
            if used > HEAP_SIZE {
                return null_mut();
            }
            if NEXT
                .compare_exchange_weak(current, used, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                return start as *mut u8;
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn reset() -> ! {
    unsafe {
        let data_len = core::ptr::addr_of!(_edata) as usize - core::ptr::addr_of!(_sdata) as usize;
        core::ptr::copy_nonoverlapping(
            core::ptr::addr_of!(_sidata),
            core::ptr::addr_of!(_sdata) as *mut u8,
            data_len,
        );
        let bss_len = core::ptr::addr_of!(_ebss) as usize - core::ptr::addr_of!(_sbss) as usize;
        core::ptr::write_bytes(core::ptr::addr_of!(_sbss) as *mut u8, 0, bss_len);
    }
    core::hint::black_box(momenta_no_std_check::render().to_html());
    core::hint::black_box(momenta_no_std_check::route_matches());
    loop {
        core::hint::spin_loop();
    }
}
