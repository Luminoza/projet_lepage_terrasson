use core::str;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, Deserialize, Eq, Hash, PartialEq)]
pub enum EntityType {
    Player,
    Dino,
    Dodo,
    Boar,
    Snail,
    Mosquito,
    Dragon,
    Whale,
}

#[derive(Deserialize)]
struct EntityData {
    name: String,
    icon: String,
    description: String,
    hp: i32,
    atk: i32,
    hostile: bool,
}

pub struct Entity {
    pub name: String,
    pub icon: String,
    pub description: String,
    pub entity_type: EntityType,
    pub hp: i32,
    pub atk: i32,
    pub position: (usize, usize),
    pub hostile: bool,
}

const FILE_PATH: &str = "./src/entities/entities.json";

pub trait SpecificEntity {
    fn get_attack(&self) -> i32;
    fn get_health(&self) -> i32;
    fn is_hostile(&self) -> bool;
    fn get_name(&self) -> String;
    fn get_icon(&self) -> char;
    fn get_description(&self) -> String;
    fn get_position(&self) -> (usize, usize);
    fn get_type(&self) -> EntityType;
    fn set_position(&mut self, position: (usize, usize));
    fn take_damage(&mut self, damage: i32);
    fn attack(&self, target: &mut dyn SpecificEntity);
    fn heal(&mut self, heal: i32);
    fn buff_attack(&mut self, buff: i32);
}

impl Entity {
    pub fn new(entity_type: EntityType, position: (usize, usize)) -> Entity {
        let data = fs::read_to_string(FILE_PATH).expect("Unable to read file");
        let entity_map: HashMap<EntityType, EntityData> =
            serde_json::from_str(&data).expect("JSON was not well-formatted");
        let entity_data = entity_map.get(&entity_type).expect("Entity type not found");

        Entity {
            name: entity_data.name.clone(),
            icon: entity_data.icon.clone(),
            description: entity_data.description.clone(),
            entity_type,
            hp: entity_data.hp,
            atk: entity_data.atk,
            position,
            hostile: entity_data.hostile,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_icon(&self) -> char {
        self.icon.chars().next().unwrap()
    }

    pub fn get_description(&self) -> String {
        self.description.to_string()
    }

    pub fn get_position(&self) -> (usize, usize) {
        self.position
    }

    pub fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }

    pub fn get_type(&self) -> EntityType {
        self.entity_type
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }

    pub fn attack(&self, target: &mut dyn SpecificEntity) {
        target.take_damage(self.atk);
    }

    pub fn heal(&mut self, heal: i32) {
        self.hp += heal;
    }

    pub fn buff_attack(&mut self, buff: i32) {
        self.atk += buff;
    }
}
