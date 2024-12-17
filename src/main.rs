fn main() {
    struct MonSlabNul {
        blocs: Vec<[u8; 16]>,    
        libre: Vec<bool>,    
    }
 
    impl MonSlabNul {
        fn new() -> MonSlabNul {
            MonSlabNul {
                blocs: Vec::new(),
                libre: Vec::new(),
            }
        }
 
        fn alloue(&mut self) -> Option<usize> {
            if let Some(pos) = self.libre.iter().position(|&x| x == true) {
                self.libre[pos] = false;
                return Some(pos);
            }
            self.blocs.push([0; 16]);
            self.libre.push(false);
            Some(self.blocs.len() - 1)
        }
 
        fn libere(&mut self, pos: usize) {
            if pos < self.libre.len() {
                self.libre[pos] = true;
            }
        }
    }
 
    // test rapide
    let mut slab = MonSlabNul::new();
    println!("je test mon slab");
    let bloc1 = slab.alloue();
    let bloc2 = slab.alloue();

    println!("mes blocs : {:?} et {:?}", bloc1, bloc2);

    slab.libere(0);
    let bloc3 = slab.alloue();
    println!("nouveau bloc apres libération :{:?}", bloc3);
 }