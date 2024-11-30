/**
 * Module monster
 * Utile pour gérer les monstres du jeu
 * 
 * Auteur : Nathan LEPAGE & Antonin TERRASSON
 */

/**
 * Importation des modules
 */
use super::entity::{Entity, EntityTrait, EntityType};
use crate::entities::player::Player;

use rand::Rng;
use std::collections::HashMap;
use std::fs;

/**
 * Chemin du fichier JSON
 */
const FILE_PATH: &str = "./src/entities/monsters.json";

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

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq, Hash)]

/**
 * Enumération des types de monstres
 */
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

/**
 * Structure d'un monstre
 */
pub struct Monster {
    base: Entity,
}

/**
 * Retourne un monstre aléatoire
 */
pub fn get_random_monster(position: (usize, usize)) -> Monster {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..100) {
        0..=7 => Monster::new(MonsterType::Dragon, position),
        8..=15 => Monster::new(MonsterType::Dino, position),
        16..=30 => Monster::new(MonsterType::Whale, position),
        31..=45 => Monster::new(MonsterType::Mosquito, position),
        46..=60 => Monster::new(MonsterType::Boar, position),
        61..=75 => Monster::new(MonsterType::Turkey, position),
        _ => Monster::new(MonsterType::Snail, position),
    }
}

/**
 * Implémentation du monstre
 */
impl Monster {

    /**
     * Crée un nouveau monstre
     */
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

    /**
     * Attaque un joueur
     */
    pub fn attack(&self, target: &mut Player) {
        target.take_damage(self.get_attack());
    }
}

/**
 * Implémentation du trait pour les monstres
 */
impl EntityTrait for Monster {

    /**
     * Retourne le nom du monstre
     */
    fn get_name(&self) -> String {
        self.base.name.clone()
    }

    /**
     * Retourne l'icone du monstre
     */
    fn get_icon(&self) -> &str {
        self.base.get_icon()
    }

    /**
     * Retourne la description du monstre
     */
    fn get_description(&self) -> String {
        self.base.description.clone()
    }

    /**
     * Soigne le monstre
     */
    fn heal(&mut self, heal: i32) {
        self.base.heal(heal);
    }

    /**
     * Augmente l'attaque du monstre
     */
    fn buff_attack(&mut self, buff: i32) {
        self.base.buff_attack(buff);
    }

    /**
     * Retourne les points d'attaque du monstre
     */
    fn get_attack(&self) -> i32 {
        self.base.atk
    }

    /**
     * Retourne les points de vie du monstre
     */
    fn get_health(&self) -> i32 {
        self.base.hp
    }

    /**
     * Retourne la position du monstre
     */
    fn get_position(&self) -> (usize, usize) {
        self.base.position
    }

    /**
     * Retourne le type du monstre
     */
    fn get_type(&self) -> EntityType {
        self.base.entity_type
    }

    /**
     * Retourne si le monstre est visible
     */
    fn is_visible(&self) -> bool {
        self.base.visible
    }

    /**
     * Définit la position du monstre
     */
    fn set_position(&mut self, position: (usize, usize)) {
        self.base.position = position;
    }

    /**
     * Définit si le monstre est visible
     */
    fn set_visible(&mut self, visible: bool) {
        self.base.visible = visible;
    }

    /**
     * Diminue les points de vie du monstre en cas de dégâts
     */
    fn take_damage(&mut self, damage: i32) {
        self.base.take_damage(damage);
    }

    /**
     * Retourne si le monstre est mort
     */
    fn is_dead(&self) -> bool {
        self.base.is_dead()
    }
}

/**
 * Structure du gestionnaire de monstres
 */
pub struct MonsterManager {
    monsters: Vec<Monster>,
}

/**
 * Implémentation du gestionnaire de monstres
 */
impl MonsterManager {

    /**
     * Crée un nouveau gestionnaire de monstres
     */
    pub fn new() -> MonsterManager {
        MonsterManager {
            monsters: Vec::new(),
        }
    }

    /**
     * Ajoute un monstre au gestionnaire
     */
    pub fn add(&mut self, monster: Monster) {
        self.monsters.push(monster);
    }

    /**
     * Définit si un monstre est visible
     */
    pub fn within_range(&mut self, position: (usize, usize), range: usize) -> Vec<&mut Monster> {
        self.monsters.iter_mut().filter(|monster| {
            let (mx, my) = monster.get_position();
            let (mx, my) = (mx as isize, my as isize);
            let (px, py) = (position.0 as isize, position.1 as isize);
            (px - mx).abs() <= range as isize && (py - my).abs() <= range as isize
        }).collect()
    }

    /**
     * Retourne si une position est occupée par un monstre
     */
    pub fn is_position_occupied(&self, position: (usize, usize)) -> bool {
        self.monsters.iter().any(|monster| monster.get_position() == position)
    }

    /**
     * Retourne un monstre à une position donnée
     */
    pub fn get_mut(&mut self, position: (usize, usize)) -> Option<&mut Monster> {
        for monster in &mut self.monsters {
            if monster.get_position() == position {
                return Some(monster);
            }
        }
        None
    }

    /**
     * Retourne tous les monstres
     */
    pub fn get_all_mut(&mut self) -> &mut Vec<Monster> {
        &mut self.monsters
    }

}