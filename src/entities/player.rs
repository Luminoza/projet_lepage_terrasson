use super::entity::{Entity, EntityType, SpecificEntity};
use crate::equipments::equipment::{Equipment, EquipmentType};
use crate::items::item::{Item, ItemType};

pub struct Player {
    base: Entity,
    equipments: Vec<Equipment>,
    items: Vec<Item>,
}

impl Player {
    pub fn new(position: (usize, usize)) -> Player {
        Player {
            base: Entity::new(EntityType::Player, position),
            equipments: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn add_equipment(&mut self, equipment: Equipment) {
        self.equipments.push(equipment);
    }

    pub fn get_equipment(&self) -> &Vec<Equipment> {
        &self.equipments
    }

    pub fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item_type: ItemType) {
        for item in self.get_items() {
            if item.get_type() == &item_type {
                let index = self.items.iter().position(|i| i.get_type() == &item_type).unwrap();
                self.items.remove(index);
                break;
            }
        }
    }

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
}

impl SpecificEntity for Player {
    fn get_health(&self) -> i32 {
        self.base.hp
    }

    fn get_attack(&self) -> i32 {
        for equipment in &self.equipments {
            if equipment.get_type() == EquipmentType::Whip
            {
                return self.base.atk + 20
            }
        }
        self.base.atk
    }

    fn is_hostile(&self) -> bool {
        self.base.hostile
    }

    fn get_name(&self) -> String {
        self.base.get_name()
    }

    fn get_icon(&self) -> char {
        self.base.get_icon()
    }

    fn get_description(&self) -> String {
        self.base.get_description()
    }

    fn get_position(&self) -> (usize, usize) {
        self.base.get_position()
    }

    fn get_type(&self) -> EntityType {
        self.base.get_type()
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.base.set_position(position);
    }

    fn take_damage(&mut self, damage: i32) {
        let mut resist = 0;
        for equipment in &self.equipments {
            if equipment.get_type() == EquipmentType::Vest
            {
                resist += 20;
            }
            if equipment.get_type() == EquipmentType::Pants {
                resist += 10;
            }
        }
        if damage - resist > 0  {
            self.base.take_damage(damage-resist);
        }   
    }

    fn attack(&self, target: &mut dyn SpecificEntity) {
        self.base.attack(target);
    }

    fn heal(&mut self, heal: i32) {
        self.base.heal(heal);
    }

    fn buff_attack(&mut self, buff: i32) {
        self.base.buff_attack(buff);
    }
}
