use slab_allocator::SlabAllocator;
use core::alloc::Layout;

fn main() {
    let mut allocator = SlabAllocator::new();
    allocator.init();
    
    println!("---Test allocation---");
    
    let layout_petit = Layout::from_size_align(24, 8).unwrap();
    let ptr1 = allocator.alloc(layout_petit).unwrap();
    unsafe { ptr1.write(42u8); }
    println!("test 1 allocation réussie");
    
    let layout_moyen = Layout::from_size_align(48, 16).unwrap();
    let ptr2 = allocator.alloc(layout_moyen).unwrap();
    let ptr3 = allocator.alloc(layout_moyen).unwrap();
    unsafe {
        ptr2.write(24u8);
        ptr3.write(33u8);
    }
    println!("test 2 allocation réussie (2x moyen)");
    

    allocator.free(ptr1).unwrap();
    let ptr4 = allocator.alloc(layout_petit).unwrap();
    unsafe { ptr4.write(55u8); }
    println!("Test 3  allocation réussie après libération");
    
    // verif des valeurs stockées
    unsafe {
        println!("les valeurs stockées sont:");
        println!("ptr2: {}", *ptr2);
        println!("ptr3: {}", *ptr3);
        println!("ptr4: {}", *ptr4);
    }
    
    // stats
    let stats = allocator.get_stats();
    println!("\nStats de l'allocateur:");
    println!("chunks libres (petit): {}", stats.free_small);
    println!("chunks libres (moyen): {}", stats.free_medium);
    println!("chunks libres (grand): {}", stats.free_large);
    println!("total chunks: {}", stats.total_chunks);
    
    // test erreur avec allocation trop grande
    let layout_trop_grand = Layout::from_size_align(1024, 8).unwrap();
    match allocator.alloc(layout_trop_grand) {
        Ok(_) => println!("Erreur: devrait échouer pour taille trop grande"),
        Err(_) => println!("Test 4 OK: Allocation trop grande correctement rejetée"),
    }
}