use crate::{
    equipments::equipment::Equipment,
    items::item::Item,
};

#[derive(Debug, Clone)]
pub struct UI {
    map_to_display: Vec<Vec<String>>,
    equipments_to_display: Vec<Equipment>,
    items_to_display: Vec<Item>,
}

impl UI {
    pub fn new(size: usize) -> Self {

        let mut width = size;
        if width % 2 == 0 {
            width += 1;
        }
    
        let mut height = size;
        if height % 2 == 0 {
            height += 1;
        }

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

    pub fn display_game_view_and_message(&self, additional_lines: Vec<String>) {
        let mut item_counts = std::collections::HashMap::new();
        for item in &self.items_to_display {
            let entry = item_counts.entry(item.get_name()).or_insert((
                item.get_icon(),
                item.get_description(),
                0,
            ));
            entry.2 += 1;
        }

        let mut additional_line_index = 0;

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
            } else if additional_line_index < additional_lines.len() {
                print!("\t{}", additional_lines[additional_line_index]);
                additional_line_index += 1;
            }
            println!();
        }
        println!();
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

pub fn display_map_size() {
    print!("Entrez la taille de la carte: ");
}

pub fn display_victory_message() {
    println!("\n Félicitation ! Vous avez trouvé l'artefact !");
    println!("\n==================================================================================================================");
}

pub fn display_game_over_message() {
    println!("Game Over ! Vous êtes mort...");
    println!("\n==================================================================================================================");
}

pub fn display_suicide_message() {
    println!("\nIndiana à préféré se suicider que d'essayer de survivre dans ce labyrinthe...");
    println!("\n==================================================================================================================");
}