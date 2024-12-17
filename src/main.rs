use slab_allocator::MonSlab;

fn main() {
    let mut slab = MonSlab::new();
    
    let pos1 = slab.alloue(String::from("test1")).unwrap();
    let pos2 = slab.alloue(String::from("test2")).unwrap();
    
    slab.debug_info();
    
    slab.libere(pos1);
    let pos3 = slab.alloue(String::from("test3")).unwrap();
    
    slab.debug_info();

    slab.alloue(String::from("une chaine beaucoup trop longue pour rentrer dans un bloc"));
}