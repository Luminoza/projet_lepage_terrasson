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

#[derive(Debug, PartialEq )]
pub struct Item {
    name: String,
    icon: String,
    description: String,
    item_type: ItemType,
    position: (usize, usize),
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
}


