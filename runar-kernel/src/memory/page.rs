use crate::memory::address::Sv32VirtAddr;

pub const PAGE_SIZE: usize = 4096;
pub const ENTRIES_PER_TABLE: usize = 1024;

#[derive(Clone, Copy)]
pub struct Page {
    number: usize,
}

impl Page {
    pub fn start_address(self) -> Sv32VirtAddr {
        Sv32VirtAddr(self.number * 4096)
    }
    
}

/// A Sv32 page table entry.
/// 
/// # Fields
/// 
/// | Bits  | Field  |
/// | ----- | ------ |
/// | 31:20 | PPN[1] |
/// | 19:10 | PPN[0] |
/// | 9:8   | RSW    |
/// | 7     | D      |
/// | 6     | A      |
/// | 5     | G      |
/// | 4     | U      |
/// | 3     | X      |
/// | 2     | W      |
/// | 1     | R      |
/// | 0     | V      |
/// 
/// ## G
/// The G bit indicates a global mapping. Global mappings are those that exist in all address
/// spaces. For non-leaf PTEs, the global setting implies that all mappings in the subsequent
/// levels of the page table are global. Failing to mark a global mapping as global merely reduces
/// performance, whereas marking a non-global mapping as global is a software bug, that, after
/// switching to an address space with a different non-global mapping for that address range, can
/// unpredictable result in either mapping being used.
/// 
/// ## U
/// The U bit indicates whether the Page is accessible to user mode.
/// 
/// ## X
/// The X bit indicates whether the Page is executable.
/// 
/// ## W
/// The W bit indicates whether the Page is writeable.
/// 
/// ## R
/// The R bit indicates whether the Page is readable.
/// 
/// 
/// ## V
/// The V bit indicates whether the Page Table Entry is valid. If it is 0, all other bits in the
/// Page Table Entry are don't cares and may be used freely by software.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Sv32PageTableEntry(u32);
impl Sv32PageTableEntry {
    pub const PRESENT: u32 = 1 << 0;
    pub const READABLE: u32 = 1 << 1;
    pub const WRITABLE: u32 = 1 << 2;
    pub const EXECUTABLE: u32 = 1 << 3;
    pub const USER: u32 = 1 << 4;
    pub const GLOBAL: u32 = 1 << 5;
    pub const ACCESSED: u32 = 1 << 6;
    pub const DIRTY: u32 = 1 << 7;

    #[inline(always)]
    pub const fn empty() -> Self {
        Self(0)
    }
    #[inline(always)]
    pub const fn new(ppn: u32, flags: u32) -> Self {
        // ppn = Physical Page Number
        Self((ppn << 10) | flags)
    }


}

#[repr(C, align(4096))]
pub struct Sv32PageTable {
    pub entries: [Sv32PageTableEntry; ENTRIES_PER_TABLE],
}
impl Sv32PageTable {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            entries: [Sv32PageTableEntry::empty(); ENTRIES_PER_TABLE],
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
pub unsafe fn setup_satp(root: *const Sv32PageTable) {
    let root_address = root as *const Sv32PageTable as u64;
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