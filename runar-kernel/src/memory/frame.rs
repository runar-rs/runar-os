use crate::memory::address::Sv32PhysAddr;

#[derive(Clone, Copy)]
pub struct PhysFrame {
    number: usize
}

impl PhysFrame {
    pub fn start_address(self) -> Sv32PhysAddr {
        Sv32PhysAddr(self.number * 4096)
    }
}