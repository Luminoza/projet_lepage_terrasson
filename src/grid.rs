use rand::prelude::*;
use rand::Rng;
use std::collections::HashSet;

use crate::combat;
use crate::entities::entity::EntityTrait;
use crate::entities::monster::{self, MonsterManager};
use crate::entities::player::Player;

use crate::equipments::equipment::{Equipment, EquipmentManager, EquipmentType};
use crate::items::item::{Item, ItemManager, ItemType};
use crate::ui::{self, UI};

const WALL_ICON: &str = "🟧";
const NO_WALL_ICON: &str = "⬛️";
const GOAL_ICON: &str = "👑";
const DEFAULT_ITEM_ICON: &str = "🎁";
const DEAD_PLAYER_ICON: &str = "💀";
const PLAYER_WITH_HAT: &str = "🤠";
const PLAYER_WITH_GLASSES: &str = "🤓";
const COMBAT_ICON: &str = "❌";

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
    ui: UI,
}

impl Grid {
    /**
     * Constructeur pour initialiser une nouvelle grille
     * @param width Largeur de la grille
     * @param height Hauteur de la grille
     * @param ui Référence mutable à l'instance de UI
     * @return Une nouvelle instance de Grid
     */
    pub fn new(width: usize, height: usize, ui: UI) -> Self {
        let mut rng = rand::thread_rng();
        let goal = (
            rng.gen_range((width * 3 / 4)..width),
            rng.gen_range((height * 3 / 4)..height),
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
            ui,
        }
    }

    pub fn init(&mut self) {
        self.place_walls();
        self.place_items((self.width * self.height) / 50);
        self.place_equipments((self.width * self.height) / 50);
        self.place_monsters((self.width * self.height) / 100);
        self.update_ui();
    }

    fn update_ui(&mut self) {
        self.ui.update_map(self.map_to_display.clone());
        self.ui
            .update_equipments(self.player.get_equipment().clone());
        self.ui.update_items(self.player.get_items().clone());
    }

    /**
     * Génère un labyrinthe en utilisant l'algorithme de parcours en profondeur
     * @return Un vecteur 2D représentant le labyrinthe
     */
    fn generate_maze(&self) -> Vec<Vec<u8>> {
        let mut maze = vec![vec![0; self.width]; self.height];

        let start = self.player.get_position();
        let stop = self.goal;

        let mut stack = vec![start];
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
        maze[stop.1][stop.0] = 1;
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
                if rng.gen_range(0..100) < 5 {
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
     * @param position La position à vérifier
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
        self.build_map();
        self.update_ui();
        self.ui.display_game_view();
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
        if (distance <= visibility_range) && self.walls.contains(&position) {
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
                if !equipment.is_equiped() {
                    if self.player.has_equipment(equipment.get_type()) {
                        self.player
                            .add_item(Item::new(ItemType::HealingPotion, equipment.get_position()));
                    } else {
                        if equipment.get_type() == EquipmentType::Hat {
                            self.player.set_range(5);
                            self.player.set_icon(PLAYER_WITH_HAT);
                        } else if equipment.get_type() == EquipmentType::Glasses
                            && !self.player.has_equipment(EquipmentType::Hat)
                        {
                            self.player.set_icon(PLAYER_WITH_GLASSES);
                        }
                        self.player.add_equipment(equipment.clone());
                    }
                    equipment.set_visible(false);
                    equipment.set_equiped(true);
                }
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
                if combat::start_combat(&mut self.player, &mut *monster, &mut self.ui) {
                    monster.set_visible(false);
                } else {
                    self.flee();
                    self.display();
                }
            }
        }
    }

    /**
     * Déplace le joueur en fonction de l'entrée utilisateur
     * @param player Le joueur actuel
     */
    pub fn move_monster(&mut self) {
        let player_position = self.player.get_position();
        let mut new_positions = Vec::new();
        let monster_positions: Vec<_> = self
            .monsters
            .get_all_mut()
            .iter()
            .map(|m| m.get_position())
            .collect();

        for (i, monster) in self.monsters.get_all_mut().iter_mut().enumerate() {
            let (mx, my) = monster_positions[i];
            let (px, py) = player_position;

            let mut possible_moves = Vec::new();

            // Check each possible move and ensure it is within grid boundaries
            if mx + 1 < self.width {
                possible_moves.push((mx + 1, my));
            }
            if mx > 0 {
                possible_moves.push((mx - 1, my));
            }
            if my + 1 < self.height {
                possible_moves.push((mx, my + 1));
            }
            if my > 0 {
                possible_moves.push((mx, my - 1));
            }

            // Filter out moves that are blocked by walls or other monsters
            possible_moves.retain(|&(nx, ny)| {
                !self.walls.contains(&(nx, ny))
                    && !new_positions.contains(&(nx, ny))
                    && !monster_positions.contains(&(nx, ny))
            });

            // Choose the move that gets the monster closest to the player
            if let Some(&(nx, ny)) = possible_moves.iter().min_by_key(|&&(nx, ny)| {
                ((nx as isize - px as isize).abs() + (ny as isize - py as isize).abs()) as usize
            }) {
                new_positions.push((nx, ny));
                monster.set_position((nx, ny));
            } else {
                new_positions.push((mx, my));
            }
        }
    }

    pub fn move_player(&mut self, movement: char) {
        let (x, y) = self.player.get_position();
        let new_position = match movement {
            'z' if y > 0 => (x, y - 1),               // Move up
            'q' if x > 0 => (x - 1, y),               // Move left
            's' if y < self.height - 1 => (x, y + 1), // Move down
            'd' if x < self.width - 1 => (x + 1, y),  // Move right
            _ => {
                ui::display_invalid_movement_message();
                return;
            }
        };

        if !self.walls.contains(&new_position) {
            self.player.set_position(new_position);
        } else {
            ui::display_wall_message();
        }
        self.last_movement = movement;
        self.update_ui();
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
                        if self.player.is_dead() {
                            self.map_to_display[x][y] = DEAD_PLAYER_ICON.to_string();
                        } else {
                            self.map_to_display[x][y] = self.player.get_icon().to_string();
                        }
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
            if item.is_visible()
                && !self.monsters.is_position_occupied(item.get_position())
                && self.player.get_position() != item.get_position()
            {
                if self.player.has_equipment(EquipmentType::Glasses) {
                    self.map_to_display[item.get_position().0][item.get_position().1] =
                        item.get_icon().to_string();
                } else {
                    self.map_to_display[item.get_position().0][item.get_position().1] =
                        DEFAULT_ITEM_ICON.to_string();
                }
            }
        }

        for equipment in equipment_within_range {
            if equipment.is_visible()
                && !self.monsters.is_position_occupied(equipment.get_position())
                && self.player.get_position() != equipment.get_position()
            {
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
                if self.player.get_position() == monster.get_position() {
                    self.map_to_display[monster.get_position().0][monster.get_position().1] =
                        COMBAT_ICON.to_string();
                } else {
                    self.map_to_display[monster.get_position().0][monster.get_position().1] =
                        monster.get_icon().to_string();
                }
            }
        }
    }
}
