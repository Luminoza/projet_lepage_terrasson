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
        self.icon.as_str()
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

pub struct ItemManager {
    items: Vec<Item>,
}

impl ItemManager {
    pub fn new() -> ItemManager {
        ItemManager { items: Vec::new() }
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn get(&self, position: (usize, usize)) -> Option<&Item> {
        for item in &self.items {
            if item.get_position() == position {
                return Some(item);
            }
        }
        None
    }

    pub fn within_range(&self, position: (usize, usize), range: usize) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| {
                let (x, y) = item.get_position();
                (x as i32 - position.0 as i32).abs() <= range as i32
                    && (y as i32 - position.1 as i32).abs() <= range as i32
            })
            .collect()
    }

    pub fn is_position_occupied(&self, position: (usize, usize)) -> bool {
        self.items.iter().any(|item| item.get_position() == position)
    }

    pub fn get_mut(&mut self, position: (usize, usize)) -> Option<&mut Item> {
        for item in &mut self.items {
            if item.get_position() == position {
                return Some(item);
            }
        }
        None
    }

    pub fn remove(&mut self, position: (usize, usize)) {
        self.items.retain(|item| item.get_position() != position);
    }
}
