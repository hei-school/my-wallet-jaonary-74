use std::io;

fn main() {
    let mut portefeuille: Vec<String> = Vec::new();
    let mut menu = 0;

    loop {
        println!("Veuillez choisir une action");
        println!("1) Ajouter un élément");
        println!("2) Afficher les contenus du portefeuille");
        println!("3) Retirer un élément du portefeuille");
        println!("4) Vider le portefeuille");
        println!("5) Quitter");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur lors de la lecture de l'entrée");

        menu = input.trim().parse().expect("Veuillez entrer un nombre valide");

        if menu == 1 {
            println!("Veuillez ajouter un contenu :");
            let mut contenu = String::new();
            io::stdin()
                .read_line(&mut contenu)
                .expect("Erreur lors de la lecture de l'entrée");

            if !contenu.is_empty() {
                portefeuille.push(contenu.trim().to_string());
                println!("Vous avez ajouté {} dans votre portefeuille", contenu.trim());
            }
        } else if menu == 2 {
            for (i, contenu) in portefeuille.iter().enumerate() {
                println!("{} {}", i + 1, contenu);
            }
        } else if menu == 3 {
            println!("Contenu du portefeuille :");
            for (i, contenu) in portefeuille.iter().enumerate() {
                println!("{} {}", i + 1, contenu);
            }

            println!("Sélectionnez un élément à retirer :");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Erreur lors de la lecture de l'entrée");

            let rm: usize = input.trim().parse().expect("Veuillez entrer un nombre valide");

            if rm > 0 && rm <= portefeuille.len() {
                portefeuille.remove(rm - 1);
                println!("Un élément a été retiré du portefeuille");
            }
        } else if menu == 4 {
            portefeuille.clear();
            println!("Le portefeuille a été vidé");
        } else if menu == 5 {
            println!("Vous avez quitté le programme");
            break;
        } else {
            println!("Veuillez choisir le bon menu");
        }
    }
}