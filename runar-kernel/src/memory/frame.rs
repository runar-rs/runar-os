use crate::memory::address::PhysAddr;

#[derive(Clone, Copy)]
pub struct PhysFrame {
    number: usize
}

impl PhysFrame {
    pub fn start_address(self) -> PhysAddr {
        PhysAddr(self.number * 4096)
    }
}