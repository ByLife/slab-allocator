use slab_allocator::{MonSlab, Taille};

fn main() {
    let mut slab = MonSlab::new();
    
    let pos1 = slab.alloue(String::from("petit test")).unwrap();
    let pos2 = slab.alloue(String::from("test un peu plus long pour pool moyen")).unwrap();
    let pos3 = slab.alloue(String::from("test".repeat(100))).unwrap();
    
    slab.debug_info();
    
    slab.libere(pos1.0, pos1.1);
    let pos4 = slab.alloue(String::from("nouveau petit test")).unwrap();
    
    slab.debug_info();

    slab.alloue(String::from("test".repeat(1000)));
}