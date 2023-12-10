portfeuille = []
menu = 0

while menu != 5:
    print("Veuillez choisir une action")
    print("1) Ajouter un élément")
    print("2) Afficher les contenus du portefeuille")
    print("3) Retirer un élément du portefeuille")
    print("4) Vider le portefeuille")
    menu = int(input())

    if menu == 1:
        contenu = input("Veuillez ajouter un contenu : ")
        if len(contenu) != 0:
            portfeuille.append(contenu)
            print("Vous avez ajouté un contenu dans votre portefeuille")

    elif menu == 2:
        nbr = 1
        for contenu in portfeuille:
            print(nbr, contenu)
            nbr += 1

    elif menu == 3:
        print("Contenu du portefeuille :")
        nbr = 1
        for contenu in portfeuille:
            print(nbr, contenu)
            nbr += 1

        rm = int(input("Sélectionnez un élément à retirer : "))
        if 0 < rm < len(portfeuille):
            portfeuille.pop(rm - 1)
            print("Un élément a été retiré du portefeuille")

    elif menu == 4:
        portfeuille.clear()
        print("Le portefeuille a été vidé")

    elif menu == 5:
        print("Vous avez quitté le programme")

    else:
        print("Veuillez choisir le bon menu")