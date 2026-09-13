use crate::memory::address::VirtAddr;

pub const PAGE_SIZE: usize = 4096;
pub const ENTRIES_PER_TABLE: usize = 512;

#[derive(Clone, Copy)]
pub struct Page {
    number: usize,
}

impl Page {
    pub fn start_address(self) -> VirtAddr {
        VirtAddr(self.number * 4096)
    }
    
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageTableEntry(u64);
impl PageTableEntry {
    pub const PRESENT: u64 = 1 << 0;
    pub const READABLE: u64 = 1 << 1;
    pub const WRITABLE: u64 = 1 << 2;
    pub const EXECUTABLE: u64 = 1 << 3;
    pub const USER: u64 = 1 << 4;
    pub const GLOBAL: u64 = 1 << 5;
    pub const ACCESSED: u64 = 1 << 6;
    pub const DIRTY: u64 = 1 << 7;

    #[inline(always)]
    pub const fn empty() -> Self {
        Self(0)
    }
    #[inline(always)]
    pub const fn new(ppn: u64, flags: u64) -> Self {
        // ppn = Physical Page Number
        Self((ppn << 10) | flags)
    }


}

#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; ENTRIES_PER_TABLE],
}
impl PageTable {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            entries: [PageTableEntry::empty(); ENTRIES_PER_TABLE],
        }
    }
}

pub const SATP_MODE_SV39: u64 = 8 << 60;

/// Activates an Sv39 root page table.
///
/// # Safety
/// `root` must be aligned to 4096 bytes and its address must be
/// the physical address accessible by the current execution context.
#[inline(always)]
pub unsafe fn setup_satp(root: *const PageTable) {
    let root_address = root as *const PageTable as u64;
    let root_ppn = root_address >> 12;

    // MODE = Sv39, ASID = 0, PPN = root page table physical page number.
    let satp_value = SATP_MODE_SV39 | root_ppn;

    unsafe {
        core::arch::asm!(
        "csrw satp, {0}",
        "sfence.vma",
        in(reg) satp_value,
        options(nostack)
    );
    }
}