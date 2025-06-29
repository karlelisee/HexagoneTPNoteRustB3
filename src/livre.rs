#[derive(Clone)]
pub struct Livre {
    pub titre: String,
    pub auteur: String,
    pub annee: u32,
    pub disponible: bool,
}

impl Livre {
    pub fn new(titre: &str, auteur: &str, annee: u32) -> Self {
        Self {
            titre: titre.to_string(),
            auteur: auteur.to_string(),
            annee,
            disponible: true,
        }
    }
}
