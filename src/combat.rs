use crate::player::{Player, Enemy};
use crate::grid::Grid;

pub fn start_combat(player: &mut Player, mut enemy: Enemy, grid: &mut Grid) {
    println!("Combat entre {} et {} commence !", player.name, enemy.name);
    let mut turn = 1;

    loop {
        println!("\nTour {}: {} ({} HP) vs {} ({} HP)", turn, player.name, player.hp, enemy.name, enemy.hp);
        println!("1. Attaquer 2. Utiliser une potion 3. Fuir");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
        let choix = input.trim();

        match choix {
            "1" => {
                let attack = 10 + player.get_attack_bonus();
                enemy.hp -= attack;
                println!("\nVous infligez {} dégâts à l'ennemi !", attack);
                if enemy.hp <= 0 {
                    println!("Vous avez vaincu l'ennemi !");
                    break;
                }
                let defense = player.get_defense_bonus();
                let damage = (enemy.attack - defense).max(0);
                player.hp -= damage;
                println!("L'ennemi vous inflige {} dégâts !", damage);
                if player.is_dead() {
                    println!("Vous êtes mort !");
                    break;
                }
            }
            "2" => {
                player.use_potion();
            }
            "3" => {
                println!("Vous avez fui le combat !");
                grid.flee(); // Retourner à la position précédente
                break;
            }
            _ => println!("Choix invalide"),
        }
        turn += 1;
    }
}
