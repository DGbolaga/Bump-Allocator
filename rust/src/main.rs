
mod bump;

use::std::alloc::Layout;

fn main() {
    let mut custom_alloc = bump::BumpAllocator::new(32);

    let layout = Layout::array::<i32>(3).unwrap();
    let ptr = custom_alloc.allocate(layout) as *mut i32;

    if !ptr.is_null() {
        unsafe {
            *ptr.add(0) = 40;
            *ptr.add(1) = 50;
            *ptr.add(2) = 60;

            for  i in 0..3 {
                println!("{}", *ptr.add(i));
            }
        }
    }

    custom_alloc.reset();
}

