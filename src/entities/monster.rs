use super::entity::{Entity, EntityTrait, EntityType};
use crate::entities::player::Player;
use rand::Rng;
use std::collections::HashMap;
use std::fs;

const FILE_PATH: &str = "./src/entities/monsters.json";

#[derive(serde::Deserialize)]
struct EntityData {
    name: String,
    icon: String,
    description: String,
    hp: i32,
    atk: i32,
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum MonsterType {
    Player,
    Dino,
    Turkey,
    Boar,
    Snail,
    Mosquito,
    Dragon,
    Whale,
}

#[derive(Debug, Clone)]
pub struct Monster {
    base: Entity,
}

pub fn get_random_monster(position: (usize, usize)) -> Monster {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..7) {
        0 => Monster::new(MonsterType::Dragon, position),
        1 => Monster::new(MonsterType::Dino, position),
        2 => Monster::new(MonsterType::Whale, position),
        3 => Monster::new(MonsterType::Mosquito, position),
        4 => Monster::new(MonsterType::Boar, position),
        5 => Monster::new(MonsterType::Turkey, position),
        6 => Monster::new(MonsterType::Snail, position),
        _ => Monster::new(MonsterType::Snail, position),
    }
}

impl Monster {
    fn new(monster_type: MonsterType, position: (usize, usize)) -> Self {
        let data = fs::read_to_string(FILE_PATH).expect("Unable to read file");
        let entity_map: HashMap<MonsterType, EntityData> =
            serde_json::from_str(&data).expect("JSON was not well-formatted");
        let entity_data = entity_map
            .get(&monster_type)
            .expect("Entity type not found");

        Monster {
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
        }
    }
    pub fn attack(&self, target: &mut Player) {
        target.take_damage(self.get_attack());
    }
}

impl EntityTrait for Monster {
    fn get_name(&self) -> String {
        self.base.name.clone()
    }

    fn get_icon(&self) -> &str {
        self.base.get_icon()
    }

    fn get_description(&self) -> String {
        self.base.description.clone()
    }

    fn heal(&mut self, heal: i32) {
        self.base.heal(heal);
    }

    fn buff_attack(&mut self, buff: i32) {
        self.base.buff_attack(buff);
    }

    fn get_attack(&self) -> i32 {
        self.base.atk
    }

    fn get_health(&self) -> i32 {
        self.base.hp
    }

    fn get_position(&self) -> (usize, usize) {
        self.base.position
    }

    fn get_type(&self) -> EntityType {
        self.base.entity_type
    }

    fn is_visible(&self) -> bool {
        self.base.visible
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.base.position = position;
    }

    fn set_visible(&mut self, visible: bool) {
        self.base.visible = visible;
    }

    fn take_damage(&mut self, damage: i32) {
        self.base.take_damage(damage);
    }

    fn is_dead(&self) -> bool {
        self.base.is_dead()
    }
}

pub struct MonsterManager {
    monsters: Vec<Monster>,
}

impl MonsterManager {
    pub fn new() -> MonsterManager {
        MonsterManager {
            monsters: Vec::new(),
        }
    }

    pub fn add(&mut self, monster: Monster) {
        self.monsters.push(monster);
    }

    pub fn get(&self, position: (usize, usize)) -> Option<&Monster> {
        for monster in &self.monsters {
            if monster.get_position() == position {
                return Some(monster);
            }
        }
        None
    }

        pub fn within_range(&self, position: (usize, usize), range: usize) -> Vec<&Monster> {
        self.monsters.iter().filter(|monster| {
            let (mx, my) = monster.get_position();
            let (mx, my) = (mx as isize, my as isize);
            let (px, py) = (position.0 as isize, position.1 as isize);
            (px - mx).abs() <= range as isize && (py - my).abs() <= range as isize
        }).collect()
    }
    pub fn is_position_occupied(&self, position: (usize, usize)) -> bool {
        self.monsters.iter().any(|monster| monster.get_position() == position)
    }

    pub fn get_mut(&mut self, position: (usize, usize)) -> Option<&mut Monster> {
        for monster in &mut self.monsters {
            if monster.get_position() == position {
                return Some(monster);
            }
        }
        None
    }

    pub fn remove(&mut self, position: (usize, usize)) {
        self.monsters.retain(|monster| monster.get_position() != position);
    }
}