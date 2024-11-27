use crate::{
    combat,
    entities::player::Player,
    equipments::equipment::{Equipment, EquipmentManager, EquipmentType},
    items::item::Item,
};

#[derive(Debug, Clone)]
pub struct UI {
    map_to_display: Vec<Vec<String>>,
    equipments_to_display: Vec<Equipment>,
    items_to_display: Vec<Item>,
}

impl UI {
    pub fn new(width: usize, height: usize) -> Self {
        UI {
            map_to_display: vec![vec![String::new(); width]; height],
            equipments_to_display: Vec::new(),
            items_to_display: Vec::new(),
        }
    }

    pub fn update_map(&mut self, map: Vec<Vec<String>>) {
        self.map_to_display = map;
    }

    pub fn update_equipments(&mut self, equipments: Vec<Equipment>) {
        self.equipments_to_display = equipments;
    }

    pub fn update_items(&mut self, items: Vec<Item>) {
        self.items_to_display = items;
    }

    pub fn display_message(message: &str) {
        println!("{}", message);
    }

    pub fn display_game_view(&self) {
        let mut item_counts = std::collections::HashMap::new();
        for item in &self.items_to_display {
            let entry = item_counts.entry(item.get_name()).or_insert((
                item.get_icon(),
                item.get_description(),
                0,
            ));
            entry.2 += 1;
        }

        for y in 0..self.map_to_display.len() {
            for x in 0..self.map_to_display[y].len() {
                print!("{}", self.map_to_display[x][y]);
            }
            if y == 0 {
                print!("\tEquipments:");
            } else if y <= self.equipments_to_display.len() {
                if let Some(equipment) = self.equipments_to_display.get(y - 1) {
                    print!(
                        "\t\t{}: {}, {}",
                        equipment.get_icon(),
                        equipment.get_name(),
                        equipment.get_description()
                    );
                }
            } else if y == self.equipments_to_display.len() + 1 {
                print!("\tItems:");
            } else {
                let item_index = y - self.equipments_to_display.len() - 2;
                if item_index < item_counts.len() {
                    let (item_name, (item_icon, item_description, count)) =
                        item_counts.iter().nth(item_index).unwrap();
                    print!(
                        "\t\t{}: {} {}; {}",
                        item_icon, count, item_name, item_description
                    );
                }
            }
            println!();
        }
        display_movement_prompt();
    }

    pub fn display_combat_start(
        &self,
        can_flee: bool,
        turn: i32,
        player_icon: &str,
        player_name: &str,
        player_health: i32,
        monster_icon: &str,
        monster_name: &str,
        monster_health: i32,
    ) {
        let combat_intro = "------------------ ❌ Combat ❌ ------------------";
        let combat_info = format!(
            "Tour {}: {} {}: {} Hp vs {} {}: {} Hp",
            turn,
            player_icon,
            player_name,
            player_health,
            monster_icon,
            monster_name,
            monster_health
        );

        let mut item_counts = std::collections::HashMap::new();
        for item in &self.items_to_display {
            let entry = item_counts.entry(item.get_name()).or_insert((
                item.get_icon(),
                item.get_description(),
                0,
            ));
            entry.2 += 1;
        }

        let mut printed_combat_info = 0;

        for y in 0..self.map_to_display.len() {
            for x in 0..self.map_to_display[y].len() {
                print!("{}", self.map_to_display[x][y]);
            }
            if y == 0 {
                print!("\tEquipments:");
            } else if y <= self.equipments_to_display.len() {
                if let Some(equipment) = self.equipments_to_display.get(y - 1) {
                    print!(
                        "\t\t{}: {}, {}",
                        equipment.get_icon(),
                        equipment.get_name(),
                        equipment.get_description()
                    );
                }
            } else if y == self.equipments_to_display.len() + 1 {
                print!("\tItems:");
            } else if y <= self.equipments_to_display.len() + 1 + item_counts.len() {
                let item_index = y - self.equipments_to_display.len() - 2;
                if item_index < item_counts.len() {
                    let (item_name, (item_icon, item_description, count)) =
                        item_counts.iter().nth(item_index).unwrap();
                    print!(
                        "\t\t{}: {} {}; {}",
                        item_icon, count, item_name, item_description
                    );
                }
            } else if printed_combat_info == 0 {
                print!("\t{}", combat_intro);
                printed_combat_info += 1;
            } else if printed_combat_info == 1 {
                print!("\t{}", combat_info);
                printed_combat_info += 1;
            }
            println!();
        }

        display_combat_options(can_flee);
        if printed_combat_info == 0 {
            println!("\t{}", combat_intro);
            println!("\t{}", combat_info);
        }
    }

    pub fn display_combat_turn(
        &self,
        can_flee: bool,
        turn: i32,
        player_icon: &str,
        player_name: &str,
        player_health: i32,
        monster_icon: &str,
        monster_name: &str,
        monster_health: i32,
    ) {
        let combat_intro = "------------------ ⚔️ Combat ⚔️ ------------------";
        let combat_info = format!(
            "Tour {}: {} {}: {} Hp vs {} {}: {} Hp",
            turn,
            player_icon,
            player_name,
            player_health,
            monster_icon,
            monster_name,
            monster_health
        );
        let mut combat_info_attack_player = " ".to_string();
        let mut combat_info_attack_monster = " ".to_string();
        if turn != 0 {
            combat_info_attack_player = format!("{} attaque {} !", player_name, monster_name);
            combat_info_attack_monster = format!("{} attaque {} !", monster_name, player_name);
        }

        let mut item_counts = std::collections::HashMap::new();
        for item in &self.items_to_display {
            let entry = item_counts.entry(item.get_name()).or_insert((
                item.get_icon(),
                item.get_description(),
                0,
            ));
            entry.2 += 1;
        }

        let mut printed_combat_info = 0;

        for y in 0..self.map_to_display.len() {
            for x in 0..self.map_to_display[y].len() {
                print!("{}", self.map_to_display[x][y]);
            }
            if y == 0 {
                print!("\tEquipments:");
            } else if y <= self.equipments_to_display.len() {
                if let Some(equipment) = self.equipments_to_display.get(y - 1) {
                    print!(
                        "\t\t{}: {}, {}",
                        equipment.get_icon(),
                        equipment.get_name(),
                        equipment.get_description()
                    );
                }
            } else if y == self.equipments_to_display.len() + 1 {
                print!("\tItems:");
            } else if y <= self.equipments_to_display.len() + 1 + item_counts.len() {
                let item_index = y - self.equipments_to_display.len() - 2;
                if item_index < item_counts.len() {
                    let (item_name, (item_icon, item_description, count)) =
                        item_counts.iter().nth(item_index).unwrap();
                    print!(
                        "\t\t{}: {} {}; {}",
                        item_icon, count, item_name, item_description
                    );
                }
            } else if printed_combat_info == 0 {
                print!("\t{}", combat_intro);
                printed_combat_info += 1;
            } else if printed_combat_info == 1 {
                print!("\t{}", combat_info);
                printed_combat_info += 1;
            } else if printed_combat_info == 2 {
                print!("\t{}", combat_info_attack_player);
                printed_combat_info += 1;
            } else if printed_combat_info == 3 {
                print!("\t{}", combat_info_attack_monster);
                printed_combat_info += 1;
            }
            println!();
        }

        display_combat_options(can_flee);
        if printed_combat_info == 0 {
            println!("\t{}", combat_intro);
            println!("\t{}", combat_info);
            println!("\t{}", combat_info_attack_player);
            println!("\t{}", combat_info_attack_monster);
        }
    }
}

pub fn display_welcome_message() {
    println!("\n\n==========================================================================");
    println!("=== Bienvenue dans l'aventure RPG Indiana Jones (TaTala Ta TataLAAAAA) ===");
    println!("==========================================================================\n");
    println!("Votre mission : Atteignez l'artefact caché dans le labyrinthe, mais prennez garde aux monstres !");
    println!("Des artefacts secondaires peuvent vous aider à survivre...\n");
    println!("Attention, des monstres rodent dans le labyrinthe, vous ne pourrez pas les fuires si ils vous attaquent !");
    println!("Carte ({} : joueur, {} : artefact, {} : objet, {} : ennemi, {} : mur) : \n",
    "🧍", // PLAYER_ICON
    "👑", // GOAL_ICON
    "🎁", // DEFAULT_ITEM_ICON
    "💀", // DEFAULT_ENEMY_ICON
    "🟧", // WALL_ICON
    );
}

pub fn display_victory_message() {
    println!("\nBravo ! Vous avez trouvé l'artefact !");
    println!("\n==========================================================================\n");
}

pub fn display_game_over_message() {
    println!("Game Over ! Vous êtes mort...");
}

pub fn display_suicide_message() {
    println!("\nIndiana à préféré se suicider que d'essayer de survivre dans ce labyrinthe...");
    println!("\n=============================================================================\n");
}

pub fn display_movement_prompt() {
    println!(
        "\nEntrez votre déplacement (z : hauts, q : gauche, s : bas, d : droite, c : suicide) :\n"
    );
}

pub fn display_invalid_movement_message() {
    println!("Mouvement invalide");
}

pub fn display_wall_message() {
    println!("\nVous ne pouvez pas traverser un mur !\n");
}

pub fn display_combat_options(can_flee : bool) {
    if can_flee {

        println!("\nRègles de combat : A attaquer, F fuir, P potion");
    }else {
        println!("\nRègles de combat : A attaquer, P potion")
    }
}

pub fn display_attack_message() {
    println!("\nVous attaquez l'ennemi !");
}

pub fn display_victory_in_combat_message() {
    println!("\nVous avez vaincu l'ennemi !");
}

pub fn display_monster_attack_message() {
    println!("\nL'ennemi vous attaque !");
}

pub fn display_death_message() {
    println!("\nVous êtes mort !");
}

pub fn display_flee_message() {
    println!("\nVous avez fui le combat !");
}

pub fn display_invalid_choice_message() {
    println!("Choix invalide");
}
