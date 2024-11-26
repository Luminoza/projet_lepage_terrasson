use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event;

use crate::entities::entity::EntityTrait;

use crate::entities::player::Player;
use crate::entities::monster::Monster;
use crate::items::item::ItemType;

pub fn start_combat(player: &mut Player, monster: &mut Monster) -> bool {
    println!("\nCombat entre {} et {} commence !", player.get_name(), monster.get_name());
    let mut turn = 1;

    loop {
        println!("\nTour {}: {} ({} HP) vs {} ({} HP)\n", turn, player.get_name(), player.get_health(), monster.get_name(), monster.get_health());
        
        if player.get_items().len() > 0 {
            println!("\nInventaire : vous avez {} potions de soin", player.get_items().len());
            println!("\na : Attaquer, p : Utiliser une potion, f : Fuir");
        }
        println!("\na : Attaquer, f : Fuir");

        if let Err(e) = enable_raw_mode() {
            eprintln!("Erreur lors de l'activation du mode brut: {}", e);
            continue;
        }
        let action = match event::read() {
            Ok(event::Event::Key(event::KeyEvent { code: event::KeyCode::Char(c), .. })) => c,
            Ok(_) => continue,
            Err(e) => {
                eprintln!("Erreur de lecture de l'événement: {}", e);
                continue;
            }
        };
        if let Err(e) = disable_raw_mode() {
            eprintln!("Erreur lors de la désactivation du mode brut: {}", e);
        }

        match action {
            'a' => {
                player.attack(monster);
                println!("\nVous attaquez l'ennemi !");
                if monster.is_dead() {
                    println!("\nVous avez vaincu l'ennemi !");
                    return true;
                }
                monster.attack(player);
                println!("\nL'ennemi vous attaque !");
                if player.is_dead() {
                    println!("\nVous êtes mort !");
                    return false;
                }
            }
            'p' => {
                player.use_item(ItemType::HealingPotion);
            }
            'f' => {
                println!("\nVous avez fui le combat !");
                return false;
            }
            _ => println!("Choix invalide"),
        }
        turn += 1;
    }
}
