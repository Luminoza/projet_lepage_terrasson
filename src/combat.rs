use crossterm::terminal::enable_raw_mode;

use crate::entities::entity::EntityTrait;
use crate::entities::monster::Monster;
use crate::entities::player::Player;
use crate::items::item::ItemType;
use crate::read_active_key;
use crate::ui::UI;

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
                "\t{} à déclanché un combat contre {}",
                player.get_name(),
                monster.get_name()
            )
        } else {
            format!(
                "\t{} à déclanché un combat contre {}",
                monster.get_name(),
                player.get_name()
            )
        },
        if !can_flee {
            format!("\tVous ne pouvez pas fuir")
        } else {
            String::new()
        },
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
    ]);

    loop {
        enable_raw_mode().unwrap();
        let action = read_active_key();

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
                        "\tVous avez gagné le combat! 🎉".to_string(),
                        "\tAppuyez sur une touche pour continuer".to_string(),
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
                        "\tVous êtes mort 💀".to_string(),
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
                    format!("\t{} attaque {} !", player.get_name(), monster.get_name()),
                    format!("\t{} attaque {} !", monster.get_name(), player.get_name()),
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
                    format!("\t{} attaque {} !", player.get_name(), monster.get_name()),
                    format!("\t{} attaque {} !", monster.get_name(), player.get_name()),
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
                        "\tVous avez fui le combat !".to_string(),
                        "\tAppuyez sur une touche pour continuer".to_string(),
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
                        "\tChoix invalide !".to_string(),
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
                "\tChoix invalide !".to_string(),
            ]),
        }
        turn += 1;
    }
}
