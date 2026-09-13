/// Specifies what kind a memory region is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryRegionKind {
    Usable = 0x00,
    Reserved = 0x01,
    Firmware = 0x02,
    Kernel = 0x03,
}

#[derive(Clone, Copy, Debug)]
pub struct MemoryRegion {
    start: usize,
    end: usize,
    kind: MemoryRegionKind,
}

impl MemoryRegion {

    /// Create a new `MemoryRegion`
    /// 
    /// # Panics
    /// Debug Assertions:
    /// - If start is greater than end
    #[inline(always)]
    pub fn new(start: usize, end: usize, kind: MemoryRegionKind) -> Self {
        #[cfg(debug_assertions)]
        {
            // Additional checks for debug builds
            assert!(start <= end);
        }
        Self {
            start,
            end,
            kind
        }
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.end - self.start
    }

    #[inline(always)]
    pub fn is_usable(&self) -> bool {
        self.kind == MemoryRegionKind::Usable
    }
}