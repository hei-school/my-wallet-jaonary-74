const prompt = require('prompt-sync')();

let portefeuille = [];
let menu = 0;

while (menu != 5) {
    console.log("Veuillez choisir une action");
    console.log("1) Ajouter un élément");
    console.log("2) Afficher les contenus du portefeuille");
    console.log("3) Retirer un élément du portefeuille");
    console.log("4) Vider le portefeuille");
    console.log("5) Quitter");

    menu = parseInt(prompt('Entrer votre choix: '));

    if (menu === 1) {
        let contenu = prompt('Veuillez ajouter un contenu :');
        if (contenu.length !== 0) {
            portefeuille.push(contenu);
            console.log(`Vous avez ajouté ${contenu} dans votre portefeuille` );
        }
    } else if (menu === 2) {
        let nbr = 1;
        for (let i = 0; i < portefeuille.length; i++) {
            console.log(nbr, portefeuille[i]);
            nbr++;
        }
    } else if (menu === 3) {
        console.log("Contenu du portefeuille :");
        let nbr = 1;
        for (let i = 0; i < portefeuille.length; i++) {
            console.log(nbr, portefeuille[i]);
            nbr++;
        }

        let rm = parseInt(prompt("Entrer le numero de l'element à retirer :"));
        if (rm > 0 && rm <= portefeuille.length) {
            portefeuille.splice(rm - 1, 1);
            console.log("Un élément a été retiré du portefeuille");
        }
    } else if (menu === 4) {
        portfeuille = [];
        console.log("Le portefeuille a été vidé");
    } else if (menu === 5) {
    console.log("Vous avez quitté le programme");
    } else {
        console.log("Veuillez choisir le bon menu");
    }
}