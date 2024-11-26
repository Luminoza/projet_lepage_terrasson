use core::str;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Eq, Hash, PartialEq)]
pub enum EntityType {
    Player,
    Monster,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub name: String,
    pub icon: String,
    pub description: String,
    pub entity_type: EntityType,
    pub hp: i32,
    pub atk: i32,
    pub position: (usize, usize),
    pub visible: bool,
}

pub trait EntityTrait {
    fn get_name(&self) -> String;
    fn get_icon(&self) -> &str;
    fn get_description(&self) -> String;
    fn get_attack(&self) -> i32;
    fn get_health(&self) -> i32;
    fn get_position(&self) -> (usize, usize);
    fn get_type(&self) -> EntityType;
    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
    fn set_position(&mut self, position: (usize, usize));
    fn heal(&mut self, heal: i32);
    fn buff_attack(&mut self, buff: i32);
    fn take_damage(&mut self, damage: i32);
    fn is_dead(&self) -> bool;
}

impl EntityTrait for Entity {
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_icon(&self) -> &str {
        &self.icon
    }

    fn get_description(&self) -> String {
        self.description.to_string()
    }

    fn get_attack(&self) -> i32 {
        self.atk
    }

    fn get_health(&self) -> i32 {
        self.hp
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn get_type(&self) -> EntityType {
        self.entity_type
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position
    }

    fn heal(&mut self, heal: i32) {
        self.hp += heal
    }

    fn buff_attack(&mut self, buff: i32) {
        self.atk += buff
    }

    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}
