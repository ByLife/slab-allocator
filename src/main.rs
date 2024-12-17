fn main() {
    #[derive(Debug)]
    enum StatutBloc {
        Libre,
        Occupe { donnees: String }
    }
 
    struct MonSlab {
        blocs: Vec<[u8; 32]>,             
        statut: Vec<StatutBloc>,          
        nb_libre: usize,                   
    }
 
    impl MonSlab {
        fn new() -> MonSlab {
            MonSlab {
                blocs: Vec::new(),
                statut: Vec::new(),
                nb_libre: 0,
            }
        }
 
        fn alloue(&mut self, data: String) -> Option<usize> {
            if data.len() > 32 {
                println!("oula c trop gros");
                return None;
            }
 
            if let Some(pos) = self.statut.iter().position(|x| matches!(x, StatutBloc::Libre)) {
                self.blocs[pos][..data.len()].copy_from_slice(data.as_bytes());
                self.statut[pos] = StatutBloc::Occupe { donnees: data };
                self.nb_libre -= 1;
                return Some(pos);
            }
 
            let mut nouveau = [0; 32];
            nouveau[..data.len()].copy_from_slice(data.as_bytes());
            self.blocs.push(nouveau);
            self.statut.push(StatutBloc::Occupe { donnees: data });
            Some(self.blocs.len() - 1)
        }
 
        fn libere(&mut self, pos: usize) -> bool {
            if pos >= self.statut.len() {
                println!("position nawak");
                return false;
            }
 
            if matches!(self.statut[pos], StatutBloc::Libre) {
                println!("deja libre");
                return false;
            }
 
            self.statut[pos] = StatutBloc::Libre;
            self.nb_libre += 1;
            true
        }
 
        fn debug_info(&self) {
            println!("---debug de ouf---");
            println!("total blocs: {}", self.blocs.len());
            println!("blocs libres: {}", self.nb_libre);
            for (i, statut) in self.statut.iter().enumerate() {
                println!("bloc {}: {:?}", i, statut);
            }
        }
    }
 
    let mut slab = MonSlab::new();
    
    let pos1 = slab.alloue(String::from("test1")).unwrap();
    let pos2 = slab.alloue(String::from("test2")).unwrap();
    
    slab.debug_info();
    
    slab.libere(pos1);
    let pos3 = slab.alloue(String::from("test3")).unwrap();
    
    slab.debug_info();
 
    slab.alloue(String::from("une string beaucoup trop longue pour rentrer dans un bloc"));
 }