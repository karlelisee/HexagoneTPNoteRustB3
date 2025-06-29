mod livre;
mod bibliotheque;

use bibliotheque::Bibliotheque;
use livre::Livre;
use std::io;

fn main() {
    let mut bibliotheque = Bibliotheque::new();

    loop {
        println!("\n--- Menu Bibliothèque ---");
        println!("1. Ajouter un livre");
        println!("2. Emprunter un livre");
        println!("3. Retourner un livre");
        println!("4. Afficher tous les livres");
        println!("5. Afficher les livres disponibles");
        println!("6. Quitter");
        println!("Choisissez une option : ");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de saisie");
        match choix.trim() {
            "1" => {
                let (titre, auteur, annee) = obtenir_details_livre();
                if bibliotheque.ajouter_livre(Livre::new(&titre, &auteur, annee)) {
                    println!("Livre ajouté avec succès.");
                } else {
                    println!("Erreur : Un livre avec ce titre existe déjà.");
                }
            },
            "2" => {
                println!("Titre du livre à emprunter : ");
                let titre = saisir_ligne();
                if bibliotheque.emprunter_livre(&titre) {
                    println!("Livre emprunté.");
                } else {
                    println!("Livre introuvable ou déjà emprunté.");
                }
            },
            "3" => {
                println!("Titre du livre à retourner : ");
                let titre = saisir_ligne();
                if bibliotheque.retourner_livre(&titre) {
                    println!("Livre retourné.");
                } else {
                    println!("Livre introuvable ou non emprunté.");
                }
            },
            "4" => bibliotheque.afficher_livres(),
            "5" => bibliotheque.afficher_livres_disponibles(),
            "6" => {
                println!("Quitter le programme.");
                break;
            },
            _ => println!("Option invalide."),
        }
    }
}

fn saisir_ligne() -> String {
    let mut ligne = String::new();
    io::stdin().read_line(&mut ligne).expect("Erreur de saisie");
    ligne.trim().to_string()
}

fn obtenir_details_livre() -> (String, String, u32) {
    println!("Titre : ");
    let titre = saisir_ligne();
    println!("Auteur : ");
    let auteur = saisir_ligne();
    println!("Année de publication : ");
    let annee: u32 = loop {
        let entree = saisir_ligne();
        match entree.parse::<u32>() {
            Ok(num) => break num,
            Err(_) => println!("Entrée invalide, veuillez entrer une année valide."),
        }
    };
    (titre, auteur, annee)
}
