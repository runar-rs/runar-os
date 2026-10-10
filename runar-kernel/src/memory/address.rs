/// A physical memory address in Sv32 virtual memory systems.
/// 
/// | Bits  | Field       |
/// | ----- | ----------- |
/// | 33:22 | PPN[1]      |
/// | 21:12 | PPN[0]      | 
/// | 11:0  | Page offset |
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv32PhysAddr(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv39PhysAddr(pub usize);


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv48PhysAddr(pub usize);


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv57PhysAddr(pub usize);





/// A virtual memory address in Sv32 virtual memory systems.
/// 
/// | Bits  | Field       |
/// | ----- | ----------- |
/// | 31:22 | VPN[1]      |
/// | 21:12 | VPN[0]      |
/// | 11:0  | Page offset |
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv32VirtAddr(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv39VirtAddr(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv48VirtAddr(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv57VirtAddr(pub usize);
