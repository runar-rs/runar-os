use crate::memory::address::VirtAddr;

#[derive(Clone, Copy)]
pub struct Page {
    number: usize,
}

impl Page {
    pub fn start_address(self) -> VirtAddr {
        VirtAddr(self.number * 4096)
    }
    
}