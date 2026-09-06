
pub struct BumpAllocator {
    end: usize,
    next: core::sync::atomic::AtomicUsize,
}
impl BumpAllocator {
    fn align_up(&self, align: usize) -> usize {
        (self.next.fetch_add(align, core::sync::atomic::Ordering::Relaxed) - 1) & !(align - 1)
    }
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            end,
            next: core::sync::atomic::AtomicUsize::new(start)
        }
    }
}

unsafe impl core::alloc::GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let start = self.align_up(layout.align());
        let end = match start.checked_add(layout.size()) {
            Some(end) => end,
            None => return core::ptr::null_mut(),
        };

        if end > self.end {
            return core::ptr::null_mut();
        }
        // Update self.next
        match self.next.compare_exchange_weak(self.next.load(core::sync::atomic::Ordering::Relaxed), end, core::sync::atomic::Ordering::Relaxed, core::sync::atomic::Ordering::Relaxed) {
            Ok(_) => return start as *mut u8,
            Err(_actual) => return core::ptr::null_mut(),
        };
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        
    }
}


struct ListNode {
    size: usize,
    next: usize,
    /// Validates ListNode
    /// 
    /// Valid if `check == size ^ next`
    check: usize,
}
impl ListNode {
    /// Create a new ListNode
    #[inline(always)]
    pub fn new(size: usize, next: usize) -> Self {
        Self {
            size,
            next, 
            check: size ^ next
        }
    }

    /// Recalculates the check value to mark the node as valid.
    #[inline(always)]
    pub fn update_check(&mut self) {
        self.check = self.size ^ self.next;
    }

    /// Check if the node is valid
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.check == self.size ^ self.next
    }
}

pub struct FreeListAllocator {
    first_node: usize,
    start: usize,
    end: usize,
}

unsafe impl core::alloc::GlobalAlloc for FreeListAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut current_node_addr: usize = self.first_node;
        let mut current_best_node_addr: usize = 0;
        let mut current_best_padding: usize = usize::MAX;
        if current_node_addr != 0 {
            loop {
                let current_node: &mut ListNode;
                // Safe because current_node_addr points to a valid list node.
                unsafe {
                    current_node = &mut *(current_node_addr as *mut ListNode);
                }
                if !current_node.is_valid() {
                    panic!();
                }
                let aligned_addr = (current_node_addr + layout.align() - 1) & !(layout.align() - 1);
                let padding = aligned_addr - current_node_addr;
                let required = padding + layout.size();
                if required <= current_node.size {
                    // The block fits
                    if padding < current_best_padding {
                        current_best_node_addr = current_node_addr;
                        current_best_padding = padding;
                        if padding == 0 {
                            // Best possible padding. Would be a waste of resources to search further
                            break;
                        }
                    }
                }
                if current_node.next != 0 {
                    current_node_addr = current_node.next;
                }
                else {
                    break;
                }
            }
            if current_best_node_addr != 0 {
                let best_node: &mut ListNode;
                unsafe {
                    best_node = &mut*(current_best_node_addr as *mut ListNode);
                }
                let aligned_addr = (current_best_node_addr + layout.align() - 1) & !(layout.align() - 1);
                // Create new block after allocated data
                if aligned_addr + layout.size() != current_best_node_addr + best_node.size {
                    unsafe {
                        let new_node = (aligned_addr + layout.size()) as *mut ListNode;
                        new_node.write(ListNode::new(current_best_node_addr + best_node.size - aligned_addr , best_node.next));
                    }
                }
                // Shorten current block if padding was necessary
                if aligned_addr > current_best_node_addr + 3 {
                    best_node.size = aligned_addr - current_best_node_addr;
                    best_node.update_check();
                }

                aligned_addr as *mut u8
            }
            else {
                core::ptr::null_mut()
            }
        }
        else {
            core::ptr::null_mut()
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut current_node_addr = self.first_node;
        if current_node_addr != 0 {
            loop {
                // Get current node size
                let current_node: &mut ListNode;
                // Safe because current_node_addr points to a valid list node
                unsafe {
                    current_node = &mut *(current_node_addr as *mut ListNode);
                }
                if !current_node.is_valid() {
                    panic!();
                }

                let next_node_addr = current_node.next;

                if next_node_addr > ptr.addr() || (next_node_addr == 0) {
                    // Found the two segments between which the deallocated segment is positioned.
                    let current_node_size = current_node.size;
                    
                    if current_node_addr + current_node_size == ptr.addr() {
                        // | current | freed | (occupied) | next |
                        current_node.size += layout.size();
                        if ptr.addr() + layout.size() == next_node_addr {
                            // | current | freed | next |
                            let next_node: &ListNode;
                            // safe because next_node_addr points to a valid list node.
                            // ptr.addr() + layout.size() are at all times greater than 0.
                            unsafe {
                                next_node = &*(next_node_addr as *const ListNode);
                            }
                            if !next_node.is_valid() {
                                panic!()
                            }
                            current_node.next = next_node.next;
                            // Update check
                            current_node.update_check();
                        }
                    }
                    else if ptr.addr() + layout.size() == next_node_addr {
                        // | current | occupied | freed | next |
                        // The freed segment is directly in front of the next free segment.
                        let next_node: &ListNode;
                        // Safe because next_node_addr points to a valid list node.
                        // ptr.addr() + layout.size() are at all times greater than 0.
                        unsafe {
                            next_node = &*(next_node_addr as *const ListNode);
                        }
                        let new_node = ptr as *mut ListNode;
                        unsafe {
                            new_node.write(ListNode::new(layout.size() + next_node.size, next_node.next));
                        }
                    }
                    else {
                        // | current | occupied | freed | occupied | next |
                        let new_node = ptr as *mut ListNode;
                        current_node.next = ptr.addr();
                        unsafe {
                            new_node.write(ListNode::new(layout.size(), next_node_addr));
                        }
                    }
                    return;
                }
                current_node_addr = next_node_addr;
            }
        
        }
        else {
            //First node is not set.
            let new_node = ptr as *mut ListNode;
            unsafe {
                new_node.write(ListNode::new(layout.size(), 0));
            }
        }
    }
}