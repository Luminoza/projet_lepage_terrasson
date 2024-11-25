use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

use crate::combat;
use crate::entities::entity::{Entity, EntityTrait};
use crate::entities::monster::{self, Monster};
use crate::entities::player::Player;

use crate::equipments::equipment::{Equipment, EquipmentType};
use crate::items::item::{self, Item};

const WALL_ICON: &str = "🟧";
const NO_WALL_ICON: &str = "⬛️";
const GOAL_ICON: &str = "👑";
const DEFAULT_ITEM_ICON: &str = "🎁";
const DEFAULT_ENEMY_ICON: &str = "💀";

/// Structure représentant la grille de jeu
pub struct Grid {
    width: usize,
    height: usize,
    player: Player,
    last_movement: char,
    monsters: Vec<Monster>,
    items: Vec<Item>,
    equipments: Vec<Equipment>,
    goal: (usize, usize),
    walls: Vec<(usize, usize)>,
    visible_walls: HashSet<(usize, usize)>,
}

impl Grid {
    /**
     * Constructeur pour initialiser une nouvelle grille
     * @param width Largeur de la grille
     * @param height Hauteur de la grille
     * @return Une nouvelle instance de Grid
     */
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let goal = (rng.gen_range(width/2..width), rng.gen_range(height/2..height));
        Grid {
            width,
            height,
            player: Player::new((0, 0)),
            last_movement: ' ',
            monsters: vec![],
            items: vec![],
            equipments: vec![],
            goal,
            walls: vec![],
            visible_walls: HashSet::new(),
        }
    }

    pub fn init(&mut self) {
        self.place_walls();
        self.place_items((self.width * self.height) / 50);
        self.place_equipments((self.width * self.height) / 50);
        self.place_monsters((self.width * self.height) / 30);
    }

    /**
     * Génère un labyrinthe en utilisant l'algorithme de parcours en profondeur
     * @return Un vecteur 2D représentant le labyrinthe
     */
    fn generate_maze(&self) -> Vec<Vec<u8>> {
        let mut maze = vec![vec![0; self.width]; self.height];
        let mut stack = vec![(0, 0)];
        let mut rng = rand::thread_rng();

        while let Some((x, y)) = stack.pop() {
            maze[y][x] = 1;
            let mut neighbors = vec![];

            if x > 1 && maze[y][x - 2] == 0 {
                neighbors.push((x - 2, y));
            }
            if x < self.width - 2 && maze[y][x + 2] == 0 {
                neighbors.push((x + 2, y));
            }
            if y > 1 && maze[y - 2][x] == 0 {
                neighbors.push((x, y - 2));
            }
            if y < self.height - 2 && maze[y + 2][x] == 0 {
                neighbors.push((x, y + 2));
            }

            if let Some(&(nx, ny)) = neighbors.choose(&mut rng) {
                stack.push((x, y));
                stack.push((nx, ny));
                maze[(y + ny) / 2][(x + nx) / 2] = 1;
            }
        }
        maze
    }

    /**
     * Place les murs sur la grille en utilisant le labyrinthe généré
     */
    pub fn place_walls(&mut self) {
        let mut maze = self.generate_maze();

        let mut rng = rand::thread_rng();
        for x in 0..self.width {
            for y in 0..self.height {
                if rng.gen_range(0..100) < 10 {
                    maze[y][x] = 1;
                }
            }
        }

        for x in 0..self.width {
            for y in 0..self.height {
                if maze[y][x] == 0 {
                    self.walls.push((x, y));
                }
            }
        }
    }

    /**
     * Place des objets aléatoirement sur la grille
     * @param count Nombre d'objets à placer
     */
    pub fn place_items(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            loop {
                let position = (rng.gen_range(0..self.width), rng.gen_range(0..self.height));

                if self.is_position_empty(position) {
                    self.items
                        .push(Item::new(Item::random(), (position.0, position.1)));
                    break;
                }
            }
        }
    }

    /**
     * Place des objets aléatoirement sur la grille
     * @param count Nombre d'objets à placer
     */
    pub fn place_equipments(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            loop {
                let position = (rng.gen_range(0..self.width), rng.gen_range(0..self.height));

                if self.is_position_empty(position) {
                    self.equipments.push(Equipment::new(
                        Equipment::random(),
                        (position.0, position.1),
                    ));
                    break;
                }
            }
        }
    }

    /**
     * Place des ennemis aléatoirement sur la grille
     * @param count Nombre d'ennemis à placer
     */
    pub fn place_monsters(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            loop {
                let position = (rng.gen_range(0..self.width), rng.gen_range(0..self.height));
                if self.is_position_empty(position) {
                    self.monsters
                        .push(monster::get_random_monster((position.0, position.1)));
                    break;
                }
            }
        }
    }

    /**
     * Vérifie si une position est vide (pas de mur, objet, ennemi, etc.)
     * @param position La position à vérifier
     * @return Vrai si la position est vide, sinon faux
     */
    fn is_position_empty(&self, position: (usize, usize)) -> bool {
        position != self.player.get_position()
            && position != self.goal
            && !self.walls.contains(&position)
            && !self
                .items
                .iter()
                .any(|item| item.get_position() == position)
            && !self
                .equipments
                .iter()
                .any(|equipment| equipment.get_position() == position)
            && !self
                .monsters
                .iter()
                .any(|monster| monster.get_position() == position)
    }

    /**
     * Affiche la grille avec les éléments visibles
     * @param player Le joueur actuel
     */
    pub fn display(&mut self) {
        println!(
            "Carte ({} : joueur, {} : artefact, {} : objet, {} : ennemi, {} : mur) : \n",
            self.player.get_icon(),
            GOAL_ICON,
            DEFAULT_ITEM_ICON,
            DEFAULT_ENEMY_ICON,
            WALL_ICON,
        );
        for y in 0..self.height {
            for x in 0..self.width {
                if self.player.get_position() == (x, y) {
                    if self.player.is_dead() {
                        print!("💀");
                    } else if self.player.has_equipment(EquipmentType::Hat) {
                        print!("🤠");
                    } else if self.player.has_equipment(EquipmentType::Glasses) {
                        print!("🤓");
                    } else {
                        print!("{}", self.player.get_icon());
                    }
                } else if self.goal == (x, y) {
                    print!("{}", GOAL_ICON);
                } else if self.should_display_wall((x, y)) {
                    print!("{}", WALL_ICON);
                } else if self.should_display_item((x, y)) {
                    if self.player.has_equipment(EquipmentType::Glasses) {
                        for item in &self.items {
                            if item.get_position() == (x, y) {
                                if item.is_visible() {
                                    print!("{}", item.get_icon());
                                }
                            }
                        }
                    } else {
                        print!("{}", DEFAULT_ITEM_ICON);
                    }
                } else if self.should_display_equipment((x, y)) {
                    if self.player.has_equipment(EquipmentType::Glasses) {
                        for equipment in &self.equipments {
                            if equipment.get_position() == (x, y) {
                                if equipment.is_visible() {
                                    print!("{}", equipment.get_icon());
                                }
                            }
                        }
                    } else {
                        print!("{}", DEFAULT_ITEM_ICON);
                    }
                } else if self.should_display_monster((x, y)) {
                    for monster in &self.monsters {
                        if monster.get_position() == (x, y) {
                            if monster.is_visible() {
                                print!("{}", monster.get_icon());
                            }
                        }
                    }
                } else {
                    print!("{}", NO_WALL_ICON);
                }
            }
            println!();
        }
    }

    /**
     * Vérifie si un mur doit être affiché
     * @param player Le joueur actuel
     * @param position La position du mur
     * @return Vrai si le mur doit être affiché, sinon faux
     */
    fn should_display_wall(&mut self, position: (usize, usize)) -> bool {
        let distance = ((self.player.get_position().0 as isize - position.0 as isize)
            .abs()
            .max((self.player.get_position().1 as isize - position.1 as isize).abs()))
            as usize;
        let visibility_range = if self.player.has_equipment(EquipmentType::Hat) {
            5
        } else {
            2
        };
        if distance <= visibility_range && self.walls.contains(&position) {
            self.visible_walls.insert(position);
            true
        } else {
            self.visible_walls.contains(&position)
        }
    }

    /**
     * Vérifie si un objet doit être affiché
     * @param player Le joueur actuel
     * @param position La position de l'objet
     * @return Vrai si l'objet doit être affiché, sinon faux
     */
    fn should_display_item(&mut self, position: (usize, usize)) -> bool {
        let distance = ((self.player.get_position().0 as isize - position.0 as isize)
            .abs()
            .max((self.player.get_position().1 as isize - position.1 as isize).abs()))
            as usize;

        let visibility_range = if self.player.has_equipment(EquipmentType::Hat) {
            5
        } else {
            2
        };

        let mut should_display = false;

        for item in &mut self.items {
            if item.get_position() == position {
                if distance <= visibility_range && item.is_visible() == true {
                    should_display = true;
                } else {
                    item.set_visible(false);
                }
            }
        }
        should_display
    }

    /**
     * Vérifie si un objet doit être affiché
     * @param player Le joueur actuel
     * @param position La position de l'objet
     * @return Vrai si l'objet doit être affiché, sinon faux
     */
    fn should_display_equipment(&mut self, position: (usize, usize)) -> bool {
        let distance = ((self.player.get_position().0 as isize - position.0 as isize)
            .abs()
            .max((self.player.get_position().1 as isize - position.1 as isize).abs()))
            as usize;
        let visibility_range = if self.player.has_equipment(EquipmentType::Hat) {
            5
        } else {
            2
        };

        let mut should_display = false;

        for equipment in &mut self.equipments {
            if equipment.get_position() == position && equipment.is_visible() == true {
                if distance <= visibility_range {
                    equipment.set_visible(true);
                    should_display = true;
                } else {
                    equipment.set_visible(false);
                }
            }
        }
        should_display
    }

    /**
     * Vérifie si un ennemi doit être affiché
     * @param player Le joueur actuel
     * @param position La position de l'ennemi
     * @return Vrai si l'ennemi doit être affiché, sinon faux
     */
    fn should_display_monster(&mut self, position: (usize, usize)) -> bool {
        let distance = ((self.player.get_position().0 as isize - position.0 as isize)
            .abs()
            .max((self.player.get_position().1 as isize - position.1 as isize).abs()))
            as usize;
        let visibility_range = if self.player.has_equipment(EquipmentType::Hat) {
            5
        } else {
            2
        };

        let mut should_display = false;
        for monster in &mut self.monsters {
            if monster.get_position() == position {
                if distance <= visibility_range && !monster.is_dead(){
                    monster.set_visible(true);
                    should_display = true;
                } else {
                    monster.set_visible(false);
                }
            }
        }
        should_display
    }

    /**
     * Supprime l'objet à la position du joueur
     */
    pub fn check_for_item(&mut self) {
        for item in &mut self.items {
            if item.get_position() == self.player.get_position() {
                self.player.add_item(item.clone());
                item.set_visible(false);
                item.set_equiped(true);
            }
        }
    }

    /**
     * Supprime l'objet à la position du joueur
     */
    pub fn check_for_equipment(&mut self) {
        for equipment in &mut self.equipments {
            if equipment.get_position() == self.player.get_position() {
                self.player.add_equipment(equipment.clone());
                equipment.set_visible(false);
                equipment.set_equiped(true);
            }
        }
    }

    /**
     * Supprime l'ennemi à la position du joueur
     */
    pub fn check_for_monster(&mut self) {
        let mut flee = false;
        for monster in &mut self.monsters {
            if monster.get_position() == self.player.get_position()
                && monster.is_visible()
                && monster.is_hostile()
            {
                if combat::start_combat(&mut self.player, &mut *monster) {
                    monster.set_visible(false);
                    monster.set_hostile(false);
                } else {
                    flee = true;
                }
            }
        }
        if flee {
            self.flee();
        }
    }

    /**
     * Déplace le joueur en fonction de l'entrée utilisateur
     * @param player Le joueur actuel
     */
    pub fn move_entity(&mut self, entity: &mut Entity, movement: char) {
        let (mut x, mut y) = entity.get_position();
        let new_position = match movement {
            'z' if y > 0 => (x, y - 1),
            'q' if x > 0 => (x - 1, y),
            's' if y < self.height - 1 => (x, y + 1),
            'd' if x < self.width - 1 => (x + 1, y),
            _ => {
                println!("Mouvement invalide");
                return;
            }
        };
        if !self.walls.contains(&new_position) {
            entity.set_position(new_position);
        } else {
            println!("\nVous ne pouvez pas traverser un mur !\n");
        }
    }

    pub fn move_player(&mut self, movement: char) {
        let (mut x, mut y) = self.player.get_position();
        let new_position = match movement {
            'z' if y > 0 => (x, y - 1),
            'q' if x > 0 => (x - 1, y),
            's' if y < self.height - 1 => (x, y + 1),
            'd' if x < self.width - 1 => (x + 1, y),
            _ => {
                println!("Mouvement invalide");
                return;
            }
        };
        if !self.walls.contains(&new_position) {
            self.player.set_position(new_position);
        } else {
            println!("\nVous ne pouvez pas traverser un mur !\n");
        }
        self.last_movement = movement;
    }

    /**
     * Gère la fuite lors d'un combat
     */
    pub fn flee(&mut self) {
        let movement = match self.last_movement {
            'z' => self.move_player('s'),
            'q' => self.move_player('d'),
            's' => self.move_player('z'),
            'd' => self.move_player('q'),
            _ => {}
        };
    }

    /**
     * Vérifie si le joueur a atteint l'objectif
     * @return Vrai si le joueur a atteint l'objectif, sinon faux
     */
    pub fn has_won(&self) -> bool {
        self.player.get_position() == self.goal
    }

    /**
     * Vérifie si le joueur a perdu
     * @return Vrai si le joueur a perdu, sinon faux
     */
    pub fn has_lost(&self) -> bool {
        if self.player.is_dead() {
            true
        } else {
            false
        }
    }
}
