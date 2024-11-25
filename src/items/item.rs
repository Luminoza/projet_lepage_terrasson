use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
pub enum ItemType {
    HealingPotion,
}

#[derive(Deserialize)]
struct ItemData {
    name: String,
    icon: String,
    description: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    name: String,
    icon: String,
    description: String,
    item_type: ItemType,
    position: (usize, usize),
    visible: bool,
    equiped: bool,
}

const FILE_PATH: &str = "./src/items/items.json";

impl Item {
    pub fn new(item_type: ItemType, position: (usize, usize)) -> Item {
        let data = fs::read_to_string(FILE_PATH).expect("Unable to read file");
        let item_map: HashMap<ItemType, ItemData> =
            serde_json::from_str(&data).expect("JSON was not well-formatted");
        let item_data = item_map.get(&item_type).expect("Item type not found");

        Item {
            name: item_data.name.clone(),
            icon: item_data.icon.clone(),
            description: item_data.description.clone(),
            item_type,
            position,
            visible: true,
            equiped: false,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_icon(&self) -> &str {
        &self.icon
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_position(&self) -> (usize, usize) {
        self.position
    }

    pub fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }

    pub fn get_type(&self) -> &ItemType {
        &self.item_type
    }

    pub fn random() -> ItemType {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..6) {
            0 => ItemType::HealingPotion,
            _ => ItemType::HealingPotion,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn is_equiped(&self) -> bool {
        self.equiped
    }

    pub fn set_equiped(&mut self, equiped: bool) {
        self.equiped = equiped;
    }
}
