use slab_allocator::SlabAlloc;

fn main() {
    let mut allocator = SlabAlloc::new();
    
    println!("test d'alloc");
    let ptr1 = allocator.alloc().unwrap();
    let ptr2 = allocator.alloc().unwrap();
    
    unsafe {
        ptr1.write(42);
        ptr2.write(24);
    }
    
    let (used, total) = allocator.get_stats();
    println!("Stats: {}/{} chunks used", used, total);
    
    println!("libération de la première allocation");
    allocator.free(ptr1).unwrap();
    let ptr3 = allocator.alloc().unwrap();
    
    unsafe {
        ptr3.write(33);
        println!("Values : {}, {}", ptr2.read(), ptr3.read());
    }
    
    let (used_after, total_after) = allocator.get_stats();
    println!("Stats : {}/{} chunks used", used_after, total_after);
}