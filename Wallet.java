import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class Wallet {
    public static void main(String[] args) {
        List<String> portfeuille = new ArrayList<>();
        int menu = 0;

        Scanner scanner = new Scanner(System.in);

        while (menu != 5) {
            System.out.println("Veuillez choisir une action");
            System.out.println("1) Ajouter un élément");
            System.out.println("2) Afficher les contenus du portefeuille");
            System.out.println("3) Retirer un élément du portefeuille");
            System.out.println("4) Vider le portefeuille");
            System.out.println("5) Quitter");

            menu = Integer.parseInt(scanner.nextLine());

            if (menu == 1) {
                System.out.println("Veuillez ajouter un contenu :");
                String contenu = scanner.nextLine();
                if (!contenu.isEmpty()) {
                    portfeuille.add(contenu);
                    System.out.println("Vous avez ajouté " + contenu + " dans votre portefeuille");
                }
            } else if (menu == 2) {
                int nbr = 1;
                for (String contenu : portfeuille) {
                    System.out.println(nbr + " " + contenu);
                    nbr++;
                }
            } else if (menu == 3) {
                System.out.println("Contenu du portefeuille :");
                int nbr = 1;
                for (String contenu : portfeuille) {
                    System.out.println(nbr + " " + contenu);
                    nbr++;
                }

                System.out.println("Sélectionnez un élément à retirer :");
                int rm = Integer.parseInt(scanner.nextLine());
                if (rm > 0 && rm <= portfeuille.size()) {
                    portfeuille.remove(rm - 1);
                    System.out.println("Un élément a été retiré du portefeuille");
                }
            } else if (menu == 4) {
                portfeuille.clear();
                System.out.println("Le portefeuille a été vidé");
            } else if (menu == 5) {
                System.out.println("Vous avez quitté le programme");
            } else {
                System.out.println("Veuillez choisir le bon menu");
            }
        }
    }
}
