/// A physical memory address.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub usize);

/// A virtual memory address.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub usize);