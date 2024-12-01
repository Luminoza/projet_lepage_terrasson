/**
 * Module Combat
 * Utile pour gérer les combats du jeu
 * 
 * Auteur :  Nathan LEPAGE & Antonin TERRASSON
 */

/**
 * Importation des modules
 */
use crate::entities::entity::EntityTrait;
use crate::entities::monster::Monster;
use crate::entities::player::Player;
use crate::items::item::ItemType;
use crate::ui::UI;
use crate::utils::read_key;

/**
 * Fonction pour démarrer un combat entre un joueur et un monstre
 */
pub fn start_combat(
    can_flee: bool,
    player: &mut Player,
    monster: &mut Monster,
    ui: &mut UI,
) -> bool {
    let mut turn = 1;

    if let Err(_) = display_combat_start(ui, can_flee, player, monster) {
        return false;
    }

    loop {
        let action = match read_key() {
            Ok(key) => key,
            Err(_) => {
                display_error(ui);
                continue;
            }
        };

        if handle_action(action, can_flee, player, monster, ui, turn) {
            return true;
        }

        turn += 1;
    }
}

fn display_combat_start(
    ui: &mut UI,
    can_flee: bool,
    player: &Player,
    monster: &Monster,
) -> Result<(), Box<dyn std::error::Error>> {
    ui.display_game_view_and_message(vec![
        "".to_string(),
        "--------------------- ❌ Combat ❌ ---------------------".to_string(),
        if can_flee {
            format!(
                "{} à déclanché un combat contre {}",
                player.get_name(),
                monster.get_name()
            )
        } else {
            format!(
                "{} à déclanché un combat contre {}",
                monster.get_name(),
                player.get_name()
            )
        },
        if !can_flee {
            format!("Vous ne pouvez pas fuir")
        } else {
            String::new()
        },
        if can_flee {
            format!("Règles de combat : A attaquer, F fuir, P potion")
        } else {
            format!("Règles de combat : A attaquer, P potion")
        },
        "".to_string(),
        format!(
            "Tour 1: {} {}: {} Hp vs {} {}: {} Hp",
            player.get_icon(),
            player.get_name(),
            player.get_health(),
            monster.get_icon(),
            monster.get_name(),
            monster.get_health()
        ),
        "".to_string(),
    ])
}

fn display_error(ui: &mut UI) {
    ui.display_game_view_and_message(vec![
        "".to_string(),
        "--------------------- ❌ Combat ❌ ---------------------".to_string(),
        "Erreur de lecture de la touche. Veuillez réessayer.".to_string(),
    ]).unwrap();
}

fn handle_action(
    action: char,
    can_flee: bool,
    player: &mut Player,
    monster: &mut Monster,
    ui: &mut UI,
    turn: usize,
) -> bool {
    match action {
        'a' => handle_attack(can_flee, player, monster, ui, turn),
        'p' => handle_potion(can_flee, player, monster, ui, turn),
        'f' => handle_flee(can_flee, player, monster, ui, turn),
        _ => handle_invalid_choice(can_flee, player, monster, ui, turn),
    }
}

fn handle_attack(
    can_flee: bool,
    player: &mut Player,
    monster: &mut Monster,
    ui: &mut UI,
    turn: usize,
) -> bool {
    player.attack(monster);
    if monster.is_dead() {
        display_victory(ui, can_flee, player, monster, turn);
        return true;
    }
    monster.attack(player);
    if player.is_dead() {
        display_defeat(ui, can_flee, player, monster, turn);
        return true;
    }
    display_turn(ui, can_flee, player, monster, turn);
    false
}

fn handle_potion(
    can_flee: bool,
    player: &mut Player,
    monster: &mut Monster,
    ui: &mut UI,
    turn: usize,
) -> bool {
    player.use_item(ItemType::HealingPotion);
    ui.update_items(player.get_items().clone());
    display_turn(ui, can_flee, player, monster, turn);
    false
}

fn handle_flee(
    can_flee: bool,
    player: &mut Player,
    monster: &mut Monster,
    ui: &mut UI,
    turn: usize,
) -> bool {
    if can_flee {
        display_flee(ui, can_flee, player, monster, turn);
        return true;
    } else {
        display_invalid_choice(ui, can_flee, player, monster, turn);
        false
    }
}

fn handle_invalid_choice(
    can_flee: bool,
    player: &mut Player,
    monster: &mut Monster,
    ui: &mut UI,
    turn: usize,
) -> bool {
    display_invalid_choice(ui, can_flee, player, monster, turn);
    false
}

fn display_victory(
    ui: &mut UI,
    can_flee: bool,
    player: &Player,
    monster: &Monster,
    turn: usize,
) {
    ui.display_game_view_and_message(vec![
        "".to_string(),
        "--------------------- ❌ Combat ❌ ---------------------".to_string(),
        if can_flee {
            format!("Règles de combat : A attaquer, F fuir, P potion")
        } else {
            format!("Règles de combat : A attaquer, P potion")
        },
        format!(
            "Tour {}: {} {}: {} Hp vs {} {}: {} Hp",
            turn,
            player.get_icon(),
            player.get_name(),
            player.get_health(),
            monster.get_icon(),
            monster.get_name(),
            monster.get_health()
        ),
        "".to_string(),
        "Vous avez gagné le combat! 🎉".to_string(),
        "Appuyez sur une touche pour continuer".to_string(),
    ]).unwrap();
}

fn display_defeat(
    ui: &mut UI,
    can_flee: bool,
    player: &Player,
    monster: &Monster,
    turn: usize,
) {
    ui.display_game_view_and_message(vec![
        "".to_string(),
        "--------------------- ❌ Combat ❌ ---------------------".to_string(),
        if can_flee {
            format!("Règles de combat : A attaquer, F fuir, P potion")
        } else {
            format!("Règles de combat : A attaquer, P potion")
        },
        "".to_string(),
        format!(
            "Tour {}: {} {}: {} Hp vs {} {}: {} Hp",
            turn,
            player.get_icon(),
            player.get_name(),
            player.get_health(),
            monster.get_icon(),
            monster.get_name(),
            monster.get_health()
        ),
        "".to_string(),
        "Vous êtes mort 💀".to_string(),
    ]).unwrap();
}

fn display_turn(
    ui: &mut UI,
    can_flee: bool,
    player: &Player,
    monster: &Monster,
    turn: usize,
) {
    ui.display_game_view_and_message(vec![
        "".to_string(),
        "--------------------- ❌ Combat ❌ ---------------------".to_string(),
        if can_flee {
            format!("Règles de combat : A attaquer, F fuir, P potion")
        } else {
            format!("Règles de combat : A attaquer, P potion")
        },
        "".to_string(),
        format!(
            "Tour {}: {} {}: {} Hp vs {} {}: {} Hp",
            turn,
            player.get_icon(),
            player.get_name(),
            player.get_health(),
            monster.get_icon(),
            monster.get_name(),
            monster.get_health()
        ),
        "".to_string(),
        format!("{} attaque {} !", player.get_name(), monster.get_name()),
        format!("{} attaque {} !", monster.get_name(), player.get_name()),
    ]).unwrap();
}

fn display_flee(
    ui: &mut UI,
    can_flee: bool,
    player: &Player,
    monster: &Monster,
    turn: usize,
) {
    ui.display_game_view_and_message(vec![
        "".to_string(),
        "--------------------- ❌ Combat ❌ ---------------------".to_string(),
        if can_flee {
            format!("Règles de combat : A attaquer, F fuir, P potion")
        } else {
            format!("Règles de combat : A attaquer, P potion")
        },
        "".to_string(),
        format!(
            "Tour {}: {} {}: {} Hp vs {} {}: {} Hp",
            turn,
            player.get_icon(),
            player.get_name(),
            player.get_health(),
            monster.get_icon(),
            monster.get_name(),
            monster.get_health()
        ),
        "".to_string(),
        "Vous avez fui le combat !".to_string(),
        "Appuyez sur une touche pour continuer".to_string(),
    ]).unwrap();
}

fn display_invalid_choice(
    ui: &mut UI,
    can_flee: bool,
    player: &Player,
    monster: &Monster,
    turn: usize,
) {
    ui.display_game_view_and_message(vec![
        "".to_string(),
        "--------------------- ❌ Combat ❌ ---------------------".to_string(),
        if can_flee {
            format!("Règles de combat : A attaquer, F fuir, P potion")
        } else {
            format!("Règles de combat : A attaquer, P potion")
        },
        "".to_string(),
        format!(
            "Tour {}: {} {}: {} Hp vs {} {}: {} Hp",
            turn,
            player.get_icon(),
            player.get_name(),
            player.get_health(),
            monster.get_icon(),
            monster.get_name(),
            monster.get_health()
        ),
        "".to_string(),
        "Choix invalide !".to_string(),
    ]).unwrap();
}
