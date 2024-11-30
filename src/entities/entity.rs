/**
 * Module des entités
 * Utile pour gérer les entités du jeu
 * 
 * Auteur : Nathan LEPAGE
 */

/**
 * Importation des modules
 */
use core::str;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Eq, Hash, PartialEq)]

/**
 * Enumération des types d'entités
 */
pub enum EntityType {
    Player,
    Monster,
}

#[derive(Debug, Clone)]
/**
 * Structure d'une entité
 */
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

/**
 * Trait pour les entités
 */
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

/**
 * Implémentation du trait pour les entités
 */
impl EntityTrait for Entity {

    /**
     * Retourne le nom de l'entité
     */
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    /**
     * Retourne l'icône de l'entité
     */
    fn get_icon(&self) -> &str {
        &self.icon
    }

    /**
     * Retourne la description de l'entité
     */
    fn get_description(&self) -> String {
        self.description.to_string()
    }

    /**
     * Retourne les points d'attaque de l'entité
     */
    fn get_attack(&self) -> i32 {
        self.atk
    }

    /**
     * Retourne les points de vie de l'entité
     */
    fn get_health(&self) -> i32 {
        self.hp
    }

    /**
     * Retourne la position de l'entité
     */
    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    /**
     * Retourne le type de l'entité
     */
    fn get_type(&self) -> EntityType {
        self.entity_type
    }

    /**
     * Retourne si l'entité est visible
     */
    fn is_visible(&self) -> bool {
        self.visible
    }

    /**
     * Définit si l'entité est visible
     */
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible
    }

    /**
     * Définit la position de l'entité
     */
    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position
    }

    /**
     * Soigne l'entité
     */
    fn heal(&mut self, heal: i32) {
        self.hp += heal
    }

    /**
     * Augmente l'attaque de l'entité
     */
    fn buff_attack(&mut self, buff: i32) {
        self.atk += buff
    }

    /**
     * Diminue les points de vie de l'entité en cas de dégâts
     */
    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage
    }

    /**
     * Retourne si l'entité est morte
     */
    fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}
