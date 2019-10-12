extern crate cortex_m_rt; // must explicitly link this in

#[cfg(feature = "alloc")]
const HEAP_SIZE: usize = 1024; // TODO: determine this from symbols??? max estimated usage??? or just never allocate?? (this actually seems like a good idea btw)

#[cfg(all(feature = "alloc", feature = "alloc-cortex-m"))]
#[global_allocator]
#[link_section = ".uninit.allocator"]
static ALLOCATOR: alloc_cortex_m::CortexMHeap; // = alloc_cortex_m::CortexMHeap::empty();

#[cortex_m_rt::pre_init]
unsafe fn entry() {
    // NOTE: this executes before any static ram is set up!
    // (this function should exist even if empty, replacing the default symbol enables inlining/removal of the pre_init call)
    // (XXX: use the .uninit link section for any globals here)
    #[cfg(all(feature = "enable-alloc", feature = "alloc-cortex-m"))]
    unsafe {
        ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE);
    }
}
