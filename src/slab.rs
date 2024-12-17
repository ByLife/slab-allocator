#[derive(Debug)]
pub enum StatutBloc {
    Libre,
    Occupe { donnees: String }
}

pub struct MonSlab {
    blocs: Vec<[u8; 32]>,             
    statut: Vec<StatutBloc>,          
    nb_libre: usize,                   
}

impl MonSlab {
    pub fn new() -> MonSlab {
        MonSlab {
            blocs: Vec::new(),
            statut: Vec::new(),
            nb_libre: 0,
        }
    }

    pub fn alloue(&mut self, data: String) -> Option<usize> {
        if data.len() > 32 {
            println!("erreur: données trop grandes");
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

    pub fn libere(&mut self, pos: usize) -> bool {
        if pos >= self.statut.len() {
            println!("erreur: position invalide");
            return false;
        }

        if matches!(self.statut[pos], StatutBloc::Libre) {
            println!("erreur: bloc déjà libre");
            return false;
        }

        self.statut[pos] = StatutBloc::Libre;
        self.nb_libre += 1;
        true
    }

    pub fn debug_info(&self) {
        println!("--- Info Debug ---");
        println!("total blocs: {}", self.blocs.len());
        println!("blocs libres: {}", self.nb_libre);
        for (i, statut) in self.statut.iter().enumerate() {
            println!("bloc {}: {:?}", i, statut);
        }
    }
}