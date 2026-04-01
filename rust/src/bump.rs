use std::alloc::Layout;
use std::ptr::null_mut;
use std::mem;


pub struct BumpAllocator {
    capacity: usize,
    offset: usize,
    memory: *mut u8,
}

impl BumpAllocator {
    //Constructor
    pub fn new(size: usize) -> Self {
        //raw memory buffer allocated by rust.
        let mut buffer = Vec::<u8>::with_capacity(size);
        let memory = buffer.as_mut_ptr(); //extract pointer and halt rust's awareness of the allocation

        //have to forget buffer so it doesn't get dropped.
        mem::forget(buffer);

        Self {
            capacity: size,
            offset: 0,
            memory,
        }
    }

    pub fn allocate(&mut self, layout: Layout) -> *mut u8 {
        //Layout contains the values of the alignement for whatever datatype is requested.
        let size = layout.size();
        let alignment = layout.align();
        let aligned_offset = BumpAllocator::align_up(self.offset, alignment);
        let new_offset = aligned_offset + size;

        if new_offset > self.capacity {
            println!("Not enough space for requested data.\n");
            return null_mut();
        }

        let ptr = unsafe { 
            self.memory.add(aligned_offset)
        };

        self.offset = new_offset;

        ptr // return 
    }

    pub fn align_up(value: usize, alignment: usize) -> usize {
        (value + (alignment -1)) & !(alignment -1)                
    }

    pub fn reset(&mut self) {
        self.offset = 0;
    }

} 

impl Drop for BumpAllocator {
    //Destructor
    fn drop (&mut self) {
        unsafe {
            // restore metadata ownership back to rust for it to properly free the allocation of Vec
            let _ = Vec::from_raw_parts(self.memory, 0, self.capacity);
        }
    }
}

// System TEST.

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::Layout;

    // Unit Test.
    // TODO 1: Test for when allocator returns a not null pointer.
    #[test]
    fn test_simple_allocation_is_not_null() {
        let mut alloc = BumpAllocator::new(64);
        let ptr = alloc.allocate(Layout::new::<i32>());
        assert!(!ptr.is_null(), "Expecting a valid pointer, got null");
    }
    
    // TODO 2: Test that align_up function correctly computes the padding for different inputs.
    #[test]
    fn test_align_up_is_correct() {
        assert_eq!(BumpAllocator::align_up(0, 4), 0);
        assert_eq!(BumpAllocator::align_up(1, 4), 4);
        assert_eq!(BumpAllocator::align_up(3, 4), 4);
        assert_eq!(BumpAllocator::align_up(4, 4), 4);
        assert_eq!(BumpAllocator::align_up(5, 4), 8);
        assert_eq!(BumpAllocator::align_up(7, 8), 8);

    }

    // TODO 3: Test that allocation fails gracefully when capacity is reached.
    #[test]
    fn test_allocation_fails_over_capacity() {
        let mut alloc = BumpAllocator::new(4);
        let ptr = alloc.allocate(Layout::new::<i64>()); // i64 = 8 bytes, won't fit
        assert!(ptr.is_null(), "Expected null when allocation exceeds capacity");
    }

    // TODO 4: Test that resets function correctly resets allowing for reuse of the memory space.
        #[test]
    fn test_reset_reuses_memory() {
        let mut alloc = BumpAllocator::new(4);
        let ptr1 = alloc.allocate(Layout::new::<i32>());
        assert!(!ptr1.is_null());

        alloc.reset();
        assert_eq!(alloc.offset, 0);

        let ptr2 = alloc.allocate(Layout::new::<i32>());
        assert!(!ptr2.is_null());
        assert_eq!(ptr1, ptr2, "After reset, allocation must start from same address");
    }


    // TODO 5: Test that an allocation of zero does not cause the offest to increase (it must be the same).
    #[test]
    fn test_zero_size_allocation_does_not_advance_offset() {
        let mut alloc = BumpAllocator::new(64);
        let layout = Layout::new::<()>(); 
        alloc.allocate(layout);
        assert_eq!(alloc.offset, 0, "Zero-size alloc must not advance offset");
    }

    
}