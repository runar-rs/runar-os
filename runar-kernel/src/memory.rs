pub mod address;
pub mod alloc;
pub mod frame;
pub mod page;
pub mod region;

static mut ROOT_PAGE_TABLE: page::PageTable = page::PageTable::new();

/// Initialize the system memory.
pub fn init() {
    unsafe {
        page::setup_satp(&raw const ROOT_PAGE_TABLE);
    }
}