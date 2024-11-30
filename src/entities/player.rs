/**
 * Module player
 * Utile pour gérer le joueur
 * 
 * Auteur : Nathan LEPAGE  & Antonin TERRASSON
 */

/**
 * Importation des modules
 */
use super::entity::{Entity, EntityTrait, EntityType};
use crate::equipments::equipment::{Equipment, EquipmentType};
use crate::items::item::{Item, ItemType};
use crate::entities::monster::Monster;

use std::collections::HashMap;
use std::fs;

/**
 * Chemin du fichier JSON
 */
const FILE_PATH: &str = "./src/entities/player.json";

#[derive(serde::Deserialize)]

/**
 * Structure des données d'une entité
 */
struct EntityData {
    name: String,
    icon: String,
    description: String,
    hp: i32,
    atk: i32,
}

/**
 * Structure du joueur
 */
pub struct Player {
    base: Entity,
    equipments: Vec<Equipment>,
    items: Vec<Item>,
    range: usize,
}

/**
 * Implémentation du joueur
 */
impl Player {

    /**
     * Crée un nouveau joueur
     */
    pub fn new(position: (usize, usize)) -> Player {
        let data = fs::read_to_string(FILE_PATH).expect("Unable to read file");
        let entity_map: HashMap<String, EntityData> =
            serde_json::from_str(&data).expect("JSON was not well-formatted");

        let entity_data = entity_map.get("Player").expect("Player data not found");
        let range = 2;
        Player {
            base: Entity {
                name: entity_data.name.clone(),
                icon: entity_data.icon.clone(),
                description: entity_data.description.clone(),
                entity_type: EntityType::Monster,
                hp: entity_data.hp,
                atk: entity_data.atk,
                position,
                visible: true,
            },
            equipments: Vec::new(),
            items: Vec::new(),
            range,
        }
    }

    /**
     * Retourne la portée du joueur
     */
    pub fn add_equipment(&mut self, equipment: Equipment) {
        self.equipments.push(equipment);
    }

    /**
     * Retourne les équipements du joueur
     */
    pub fn get_equipment(&self) -> &Vec<Equipment> {
        &self.equipments
    }

    /**
     * Retourne les items du joueur
     */
    pub fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

    /**
     * Ajoute un item à l'inventaire du joueur
     */
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    /**
     * Retire un item de l'inventaire du joueur
     */
    pub fn remove_item(&mut self, item_type: ItemType) {
        for item in self.get_items() {
            if item.get_type() == &item_type {
                let index = self
                    .items
                    .iter()
                    .position(|i| i.get_type() == &item_type)
                    .unwrap();
                self.items.remove(index);
                break;
            }
        }
    }

    /**
     * Utilise un item de l'inventaire
     */
    pub fn use_item(&mut self, item_type: ItemType) {
        if let Some(item) = self.items.iter().find(|&i| i.get_type() == &item_type) {
            match item.get_type() {
                ItemType::HealingPotion => {
                    self.remove_item(ItemType::HealingPotion);
                    self.base.heal(20);
                }
            }
        }
    }

    /**
     * Retourne si le joueur possède un équipement
     */
    pub fn has_equipment(&self, equipment_type: EquipmentType) -> bool {
        for equipment in &self.equipments {
            if equipment.get_type() == equipment_type {
                return true;
            }
        }
        false
    }

    /**
     * Attaque une cible
     */
    pub fn attack(&self, target: &mut Monster) {
        target.take_damage(self.get_attack());
    }

    /**
     * Définir la portée du joueur
     */
    pub fn set_range(&mut self, range: usize){
        self.range = range;
    }

    /**
     * Retourne la portée du joueur
     */
    pub fn get_range(&self) -> usize{
        self.range
    }

    /**
     * Définir l'icône du joueur
     */
    pub fn set_icon(&mut self, icon: &str) {
        self.base.icon = icon.to_string();
    }

    /**
     * Définir le nom du joueur
     */
    pub fn heal(&mut self, amount: i32) {
        self.base.heal(amount);
    }
}

/**
 * Implémentation du trait pour le joueur
 */
impl EntityTrait for Player {

    /**
     * Retourne les points de vie du joueur
     */
    fn get_health(&self) -> i32 {
        self.base.hp
    }

    /**
     * Retourne les points d'attaque du joueur
     */
    fn get_attack(&self) -> i32 {
        if self.has_equipment(EquipmentType::Whip) {
            return self.base.atk + 20;
        }
        self.base.atk
    }

    /**
     * Retourne le nom du joueur
     */
    fn get_name(&self) -> String {
        self.base.get_name()
    }

    /**
     * Retourne l'icône du joueur
     */
    fn get_icon(&self) -> &str {
        self.base.get_icon()
    }

    /**
     * Retourne la description du joueur
     */
    fn get_description(&self) -> String {
        self.base.get_description()
    }

    /**
     * Retourne la position du joueur
     */
    fn get_position(&self) -> (usize, usize) {
        self.base.get_position()
    }

    /**
     * Retourne le type du joueur
     */
    fn get_type(&self) -> EntityType {
        self.base.get_type()
    }

    /**
     * Définit la position du joueur
     */
    fn set_position(&mut self, position: (usize, usize)) {
        self.base.set_position(position);
    }

    /**
     * Diminue les points de vie du joueur en cas de dégâts
     */
    fn take_damage(&mut self, damage: i32) {
        let mut resist = 0;
        if self.has_equipment(EquipmentType::Vest) {
            resist += 20;
        }
        if self.has_equipment(EquipmentType::Pants) {
            resist += 10;
        }
        if damage - resist > 0 {
            self.base.take_damage(damage - resist);
        }
    }

    /**
     * Soigne le joueur
     */
    fn heal(&mut self, heal: i32) {
        self.base.heal(heal);
    }

    /**
     * Augmente les points d'attaque du joueur
     */
    fn buff_attack(&mut self, buff: i32) {
        self.base.buff_attack(buff);
    }

    /**
     * Retourne si le joueur est mort
     */
    fn is_dead(&self) -> bool {
        self.base.is_dead()
    }

    /**
     * Retourne si le joueur est visible
     */
    fn is_visible(&self) -> bool {
        self.base.is_visible()
    }

    /**
     * Définit si le joueur est visible
     */
    fn set_visible(&mut self, visible: bool) {
        self.base.set_visible(visible);
    }
}
