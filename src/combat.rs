use crate::entities::entity::EntityTrait;
use crate::entities::monster::Monster;
use crate::entities::player::Player;
use crate::items::item::ItemType;
use crate::read_key;
use crate::ui::UI;

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
            format!(
                "Règles de combat : A attaquer, F fuir, P potion")
        } else {
            format!(
                "Règles de combat : A attaquer, P potion")
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
    ]);

    loop {
        let action = read_key();

        match action {
            'a' => {
                player.attack(monster);
                if monster.is_dead() {
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
                    ]);
                    return true;
                }
                monster.attack(player);
                if player.is_dead() {
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
                    ]);
                    return false;
                }
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
                ]);
            }
            'p' => {
                player.use_item(ItemType::HealingPotion);
                ui.update_items(player.get_items().clone());
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
                ]);
            }
            'f' => {
                if can_flee {
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
                    ]);
                    return false;
                } else {
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
                    ]);
                }
            }
            _ => ui.display_game_view_and_message(vec![
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
            ]),
        }
        turn += 1;
    }
}
