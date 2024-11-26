use crossterm::event;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::entities::entity::EntityTrait;
use crate::entities::monster::Monster;
use crate::entities::player::Player;
use crate::items::item::ItemType;
use crate::ui::{self, UI};

pub fn start_combat(player: &mut Player, monster: &mut Monster, ui: &mut UI) -> bool {
    ui.display_combat_start(
        0,
        player.get_icon(),
        &player.get_name(),
        player.get_health(),
        monster.get_icon(),
        &monster.get_name(),
        monster.get_health(),
    );
    let mut turn = 1;

    loop {
        enable_raw_mode().unwrap();
        let action = match event::read().unwrap() {
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            }) => c,
            _ => continue,
        };
        if let Err(e) = disable_raw_mode() {
            eprintln!("Erreur lors de la désactivation du mode brut: {}", e);
        }

        match action {
            'a' => {
                player.attack(monster);
                if monster.is_dead() {
                    ui::display_victory_in_combat_message();
                    return true;
                }
                monster.attack(player);
                if player.is_dead() {
                    ui::display_death_message();
                    return false;
                }
                ui.display_combat_turn(
                    turn,
                    player.get_icon(),
                    &player.get_name(),
                    player.get_health(),
                    monster.get_icon(),
                    &monster.get_name(),
                    monster.get_health(),
                );
            }
            'p' => {
                player.use_item(ItemType::HealingPotion);
                ui.update_items(player.get_items().clone());
                ui.display_combat_turn(
                    turn,
                    player.get_icon(),
                    &player.get_name(),
                    player.get_health(),
                    monster.get_icon(),
                    &monster.get_name(),
                    monster.get_health(),
                );
            }
            'f' => {
                ui::display_flee_message();
                return false;
            }
            _ => ui::display_invalid_choice_message(),
        }
        turn += 1;
    }
}
