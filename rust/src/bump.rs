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
