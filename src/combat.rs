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

        enable_raw_mode().unwrap();
        let action = match event::read().unwrap() {
            event::Event::Key(event::KeyEvent { code: event::KeyCode::Char(c), .. }) => c,
            _ => continue,
        };
        disable_raw_mode().unwrap();

        match action {
            'a' => {
                player.attack( monster);
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
