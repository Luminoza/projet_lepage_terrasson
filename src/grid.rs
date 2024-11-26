use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

use crate::combat;
use crate::entities::entity::{Entity, EntityTrait};
use crate::entities::monster::{self, Monster, MonsterManager};
use crate::entities::player::Player;

use crate::equipments::equipment::{self, Equipment, EquipmentManager, EquipmentType};
use crate::items::item::{Item, ItemManager, ItemType};

const WALL_ICON: &str = "🟧";
const NO_WALL_ICON: &str = "⬛️";
const GOAL_ICON: &str = "👑";
const DEFAULT_ITEM_ICON: &str = "🎁";
const DEFAULT_ENEMY_ICON: &str = "💀";
const PLAYER_WITH_HAT: &str = "🤠";
const PLAYER_WITH_GLASSES: &str = "🤓";

/// Structure représentant la grille de jeu
pub struct Grid {
    width: usize,
    height: usize,
    player: Player,
    last_movement: char,
    monsters: MonsterManager,
    items: ItemManager,
    equipments: EquipmentManager,
    goal: (usize, usize),
    walls: Vec<(usize, usize)>,
    visible_walls: HashSet<(usize, usize)>,
    map_to_display: Vec<Vec<String>>,
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
        let goal = (
            rng.gen_range(width / 2..width),
            rng.gen_range(height / 2..height),
        );

        let map_to_display = vec![vec![NO_WALL_ICON.to_string(); width]; height];

        Grid {
            width,
            height,
            player: Player::new((0, 0)),
            last_movement: ' ',
            monsters: MonsterManager::new(),
            items: ItemManager::new(),
            equipments: EquipmentManager::new(),
            goal,
            walls: vec![],
            visible_walls: HashSet::new(),
            map_to_display,
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
                        .add(Item::new(Item::random(), (position.0, position.1)));
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
                    self.equipments.add(Equipment::new(
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
                        .add(monster::get_random_monster((position.0, position.1)));
                    break;
                }
            }
        }
    }

    /**
     * Vérifie si une position est vide (pas de mur, objet, ennemi, etc.)
     * @param position La position �� vérifier
     * @return Vrai si la position est vide, sinon faux
     */
    fn is_position_empty(&self, position: (usize, usize)) -> bool {
        position != self.player.get_position()
            && position != self.goal
            && !self.walls.contains(&position)
            && !self.equipments.is_position_occupied(position)
            && !self.items.is_position_occupied(position)
            && !self.monsters.is_position_occupied(position)
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

        self.build_map();

        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.map_to_display[x][y]);
            }
            // Print player's equipment to the right of the maze
            if y == 0 {
                print!("\tEquipments:");
            } else if y <= self.player.get_equipment().len() {
                if let Some(equipment) = self.player.get_equipment().get(y - 1) {
                    print!(
                        "\t\t{}: {}, {}",
                        equipment.get_icon(),
                        equipment.get_name(),
                        equipment.get_description()
                    );
                }
            } else if y == self.player.get_equipment().len() + 1 {
                print!("  Items:");
            } else {
                if let Some(item) = self
                    .player
                    .get_items()
                    .get(y - self.player.get_equipment().len() - 2)
                {
                    print!(
                        "\t\t{}: {}, {}",
                        item.get_icon(),
                        item.get_name(),
                        item.get_description()
                    );
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
        let visibility_range = self.player.get_range();
        if distance <= visibility_range && self.walls.contains(&position) {
            self.visible_walls.insert(position);
            true
        } else {
            self.visible_walls.contains(&position)
        }
    }

    /**
     * Supprime l'objet à la position du joueur
     */
    pub fn check_for_item(&mut self) {
        if let Some(item) = self.items.get_mut(self.player.get_position()) {
            if item.get_position() == self.player.get_position() {
                if !item.is_equiped() {
                    self.player.add_item(item.clone());
                    item.set_visible(false);
                    item.set_equiped(true);
                }
            }
        }
    }

    /**
     * Supprime l'objet à la position du joueur
     */
    pub fn check_for_equipment(&mut self) {
        if let Some(equipment) = self.equipments.get_mut(self.player.get_position()) {
            if equipment.get_position() == self.player.get_position() {
                if self.player.has_equipment(equipment.get_type()) {
                    self.player
                        .add_item(Item::new(ItemType::HealingPotion, equipment.get_position()));
                } else {
                    if equipment.get_type() == EquipmentType::Hat {
                        self.player.set_range(5);
                        self.player.set_icon("🤠");
                    } else if equipment.get_type() == EquipmentType::Glasses {
                        self.player.set_icon("🤓");
                    }
                    self.player.add_equipment(equipment.clone());
                }
                equipment.set_visible(false);
                equipment.set_equiped(true);
            }
        }
    }

    /**
     * Supprime l'ennemi à la position du joueur
     */
    pub fn check_for_monster(&mut self) {
        if self.monsters.get_mut(self.player.get_position()).is_none() {
            return;
        } else {
            let monster = self.monsters.get_mut(self.player.get_position()).unwrap();
            if monster.get_position() == self.player.get_position() && monster.is_visible() {
                if combat::start_combat(&mut self.player, &mut *monster) {
                    monster.set_visible(false);
                } else {
                    self.flee();
                }
            }
        }
    }

    /**
     * Déplace le joueur en fonction de l'entrée utilisateur
     * @param player Le joueur actuel
     */
    pub fn move_monster(&mut self, entity: &mut Entity, movement: char) {
        let (x, y) = entity.get_position();
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
        let (x, y) = self.player.get_position();
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
        match self.last_movement {
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

    pub fn build_map(&mut self) {
        let mut position;
        {
            for y in 0..self.height {
                for x in 0..self.width {
                    position = (x, y);
                    if self.should_display_wall(position) {
                        self.map_to_display[x][y] = WALL_ICON.to_string();
                    } else if self.player.get_position() == position {
                        self.map_to_display[x][y] = self.player.get_icon().to_string();
                    } else if self.goal == position {
                        self.map_to_display[x][y] = GOAL_ICON.to_string();
                    } else {
                        self.map_to_display[x][y] = NO_WALL_ICON.to_string();
                    }
                }
            }
        }
        let equipment_within_range = self
            .equipments
            .within_range(self.player.get_position(), self.player.get_range());

        let monsters_within_range = self
            .monsters
            .within_range(self.player.get_position(), self.player.get_range());

        let items_within_range = self
            .items
            .within_range(self.player.get_position(), self.player.get_range());

        for item in items_within_range {
            let (x, y) = item.get_position();
            if item.is_visible() {
                if self.player.has_equipment(EquipmentType::Glasses) {
                    self.map_to_display[x][y] = item.get_icon().to_string();
                } else {
                    self.map_to_display[x][y] = DEFAULT_ITEM_ICON.to_string();
                }
            }
        }

        for equipment in equipment_within_range {
            if equipment.is_visible() {
                if self.player.has_equipment(EquipmentType::Glasses) {
                    self.map_to_display[equipment.get_position().0][equipment.get_position().1] =
                        equipment.get_icon().to_string();
                } else {
                    self.map_to_display[equipment.get_position().0][equipment.get_position().1] =
                        DEFAULT_ITEM_ICON.to_string();
                }
            }
        }

        for monster in monsters_within_range {
            if monster.is_visible() {
                self.map_to_display[monster.get_position().0][monster.get_position().1] =
                    monster.get_icon().to_string();
            }
        }
    }
}
