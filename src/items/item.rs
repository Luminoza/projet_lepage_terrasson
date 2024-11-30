/**
 * Module des items
 * Utile pour gérer les items du jeu
 * 
 * Auteur : Nathan LEPAGE
 */
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]

/**
 * Enumération des types d'items
 */
pub enum ItemType {
    HealingPotion,
}

#[derive(Deserialize)]

/**
 * Structure des données d'un item
 */
struct ItemData {
    name: String,
    icon: String,
    description: String,
}

#[derive(Debug, PartialEq, Clone)]

/**
 * Structure d'un item
 */
pub struct Item {
    name: String,
    icon: String,
    description: String,
    item_type: ItemType,
    position: (usize, usize),
    visible: bool,
    equiped: bool,
}

/**
 * Chemin du fichier JSON
 */
const FILE_PATH: &str = "./src/items/items.json";

/**
 * Implémentation de l'item
 */
impl Item {

    /**
     * Crée un nouvel item
     */
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

    /**
     * Retourne le nom de l'item
     */
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /**
     * Retourne l'icone de l'item
     */
    pub fn get_icon(&self) -> &str {
        self.icon.as_str()
    }

    /**
     * Retourne la description de l'item
     */
    pub fn get_description(&self) -> &str {
        &self.description
    }

    /**
     * Retourne la position de l'item
     */
    pub fn get_position(&self) -> (usize, usize) {
        self.position
    }

    /**
     * Retourne le type de l'item
     */
    pub fn get_type(&self) -> &ItemType {
        &self.item_type
    }

    /**
     * Retourne un item aléatoire
     */
    pub fn random() -> ItemType {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..6) {
            0 => ItemType::HealingPotion,
            _ => ItemType::HealingPotion,
        }
    }

    /**
     * Retourne si l'item est visible
     */
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /**
     * Définit si l'item est visible
     */
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /**
     * Retourne si l'item est équipé
     */
    pub fn is_equiped(&self) -> bool {
        self.equiped
    }

    /**
     * Définit si l'item est équipé
     */
    pub fn set_equiped(&mut self, equiped: bool) {
        self.equiped = equiped;
    }
}

/**
 * Structure du gestionnaire d'items
 */
pub struct ItemManager {
    items: Vec<Item>,
}

/**
 * Implémentation du gestionnaire d'items
 */
impl ItemManager {

    /**
     * Crée un nouveau gestionnaire d'items
     */
    pub fn new() -> ItemManager {
        ItemManager { items: Vec::new() }
    }

    /**
     * Ajoute un item au gestionnaire
     */
    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    /**
     * Retourne les items dans une certaine portée de joueur
     */
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

    /**
     * Retourne si une position est occupée par un item
     */
    pub fn is_position_occupied(&self, position: (usize, usize)) -> bool {
        self.items.iter().any(|item| item.get_position() == position)
    }

    /**
     * Retourne un item mutable à une certaine position
     */
    pub fn get_mut(&mut self, position: (usize, usize)) -> Option<&mut Item> {
        for item in &mut self.items {
            if item.get_position() == position {
                return Some(item);
            }
        }
        None
    }
}
