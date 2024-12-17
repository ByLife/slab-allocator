#[derive(Debug)]
pub enum StatutBloc {
    Libre,
    Occupe { donnees: String, taille_utilisee: usize }
}

#[derive(Debug, Copy, Clone)]
pub enum Taille {
    Petit,    
    Moyen,    
    Grand     
}

impl Taille {
    fn en_bytes(&self) -> usize {
        match self {
            Taille::Petit => 32,
            Taille::Moyen => 128,
            Taille::Grand => 512,
        }
    }
}

struct Pool {
    blocs: Vec<Vec<u8>>,
    statut: Vec<StatutBloc>,
    taille_bloc: Taille,
    nb_libre: usize,
}

impl Pool {
    fn new(taille: Taille) -> Pool {
        Pool {
            blocs: Vec::new(),
            statut: Vec::new(),
            taille_bloc: taille,
            nb_libre: 0,
        }
    }

    fn alloue(&mut self, data: String) -> Option<usize> {
        let taille = data.len();
        
        if taille > self.taille_bloc.en_bytes() {
            return None;
        }

        if let Some(pos) = self.statut.iter().position(|x| matches!(x, StatutBloc::Libre)) {
            let mut bloc = vec![0; self.taille_bloc.en_bytes()];
            bloc[..taille].copy_from_slice(data.as_bytes());
            self.blocs[pos] = bloc;
            self.statut[pos] = StatutBloc::Occupe { 
                taille_utilisee: taille,
                donnees: data
            };
            self.nb_libre -= 1;
            return Some(pos);
        }

        let mut nouveau = vec![0; self.taille_bloc.en_bytes()];
        nouveau[..taille].copy_from_slice(data.as_bytes());
        self.blocs.push(nouveau);
        self.statut.push(StatutBloc::Occupe { 
            taille_utilisee: taille,
            donnees: data
        });
        Some(self.blocs.len() - 1)
    }

    fn libere(&mut self, pos: usize) -> bool {
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

    fn info(&self) -> (usize, usize, usize) {
        let total = self.blocs.len();
        let utilise = total - self.nb_libre;
        let mut memoire_utilisee = 0;
        
        for statut in &self.statut {
            if let StatutBloc::Occupe { taille_utilisee, .. } = statut {
                memoire_utilisee += taille_utilisee;
            }
        }
        
        (total, utilise, memoire_utilisee)
    }
}

pub struct MonSlab {
    pool_petit: Pool,
    pool_moyen: Pool,
    pool_grand: Pool,
}

impl MonSlab {
    pub fn new() -> MonSlab {
        MonSlab {
            pool_petit: Pool::new(Taille::Petit),
            pool_moyen: Pool::new(Taille::Moyen),
            pool_grand: Pool::new(Taille::Grand),
        }
    }

    pub fn alloue(&mut self, data: String) -> Option<(Taille, usize)> {
        let len = data.len();
        
        if len <= Taille::Petit.en_bytes() {
            self.pool_petit.alloue(data).map(|idx| (Taille::Petit, idx))
        } else if len <= Taille::Moyen.en_bytes() {
            self.pool_moyen.alloue(data).map(|idx| (Taille::Moyen, idx))
        } else if len <= Taille::Grand.en_bytes() {
            self.pool_grand.alloue(data).map(|idx| (Taille::Grand, idx))
        } else {
            println!("erreur: données trop grandes pour tous les pools");
            None
        }
    }

    pub fn libere(&mut self, taille: Taille, pos: usize) -> bool {
        match taille {
            Taille::Petit => self.pool_petit.libere(pos),
            Taille::Moyen => self.pool_moyen.libere(pos),
            Taille::Grand => self.pool_grand.libere(pos),
        }
    }

    pub fn debug_info(&self) {
        println!("--- Info Debug ---");
        let (total_p, util_p, mem_p) = self.pool_petit.info();
        let (total_m, util_m, mem_m) = self.pool_moyen.info();
        let (total_g, util_g, mem_g) = self.pool_grand.info();
        
        println!("Pool Petit (32B):");
        println!("Blocs : {}/{} - Mémoire: {}B", util_p, total_p, mem_p);
        
        println!("Pool Moyen (128B):");
        println!(" Blocs: {}/{} - Mémoire: {}B", util_m, total_m, mem_m);
        
        println!("Pool Grand (512B):");
        println!(" Blocs: {}/{} - Mémoire: {}B", util_g, total_g, mem_g);
        
        println!("Total mémoire utilisée: {}B", mem_p + mem_m + mem_g);
    }
}
