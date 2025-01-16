#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;
use core::alloc::Layout;
use slab_allocator::SlabAllocator;

mod memory;
mod writer;
use memory::BootInfoFrameAllocator;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    print!("---Test allocation---\n");
    
    let mut allocator = SlabAllocator::new();
    allocator.init();

    let layout_petit = Layout::from_size_align(24, 8).unwrap();
    let ptr1 = allocator.alloc(layout_petit).unwrap();
    unsafe { ptr1.write(42u8); }
    print!("test 1 allocation réussie\n");
    
    let layout_moyen = Layout::from_size_align(48, 16).unwrap();
    let ptr2 = allocator.alloc(layout_moyen).unwrap();
    let ptr3 = allocator.alloc(layout_moyen).unwrap();
    unsafe {
        ptr2.write(24u8);
        ptr3.write(33u8);
    }
    print!("test 2 allocation réussie (2x moyen)\n");

    allocator.free(ptr1).unwrap();
    let ptr4 = allocator.alloc(layout_petit).unwrap();
    unsafe { ptr4.write(55u8); }
    print!("Test 3 allocation réussie après libération\n");
    
    unsafe {
        print!("les valeurs stockées sont:\n");
        print!("ptr2: {}\n", *ptr2);
        print!("ptr3: {}\n", *ptr3);
        print!("ptr4: {}\n", *ptr4);
    }
    
    let stats = allocator.get_stats();
    print!("\nStats de l'allocateur:\n");
    print!("chunks libres (petit): {}\n", stats.free_small);
    print!("chunks libres (moyen): {}\n", stats.free_medium);
    print!("chunks libres (grand): {}\n", stats.free_large);
    print!("total chunks: {}\n", stats.total_chunks);
    
    let layout_trop_grand = Layout::from_size_align(1024, 8).unwrap();
    match allocator.alloc(layout_trop_grand) {
        Ok(_) => print!("Erreur: devrait échouer pour taille trop grande\n"),
        Err(_) => print!("Test 4 OK: Allocation trop grande correctement rejetée\n"),
    }

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);
    loop {}
}