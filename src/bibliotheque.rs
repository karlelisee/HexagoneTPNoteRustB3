use crate::livre::Livre;

pub struct Bibliotheque {
    pub livres: Vec<Livre>,
}

impl Bibliotheque {
    pub fn new() -> Self {
        Self { livres: Vec::new() }
    }

    pub fn ajouter_livre(&mut self, livre: Livre) -> bool {
        if self.livres.iter().any(|l| l.titre == livre.titre) {
            false
        } else {
            self.livres.push(livre);
            true
        }
    }

    pub fn emprunter_livre(&mut self, titre: &str) -> bool {
        if let Some(livre) = self.livres.iter_mut().find(|l| l.titre == titre && l.disponible) {
            livre.disponible = false;
            true
        } else {
            false
        }
    }

    pub fn retourner_livre(&mut self, titre: &str) -> bool {
        if let Some(livre) = self.livres.iter_mut().find(|l| l.titre == titre && !l.disponible) {
            livre.disponible = true;
            true
        } else {
            false
        }
    }

    pub fn afficher_livres(&self) {
        for livre in &self.livres {
            println!(
                "{} par {}, {} [{}]",
                livre.titre,
                livre.auteur,
                livre.annee,
                if livre.disponible { "Disponible" } else { "Emprunté" }
            );
        }
    }

    pub fn afficher_livres_disponibles(&self) {
        self.livres.iter()
            .filter(|l| l.disponible)
            .for_each(|livre| {
                println!("{} par {}, {}", livre.titre, livre.auteur, livre.annee);
            });
    }
}
