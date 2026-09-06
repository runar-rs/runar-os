#[repr(C)]
pub struct MemoryRegion {
    pub start: usize,
    pub end: usize,
}

#[repr(C)]
pub struct BootInfo {
    pub memory: MemoryRegion
}