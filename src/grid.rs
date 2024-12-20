/**
 * Module grid
 * Utile pour gérer la grille de jeu
 *
 * Auteur : Antonin TERRASSON & Nathan LEPAGE
 */

/**
 * Importation des modules
 */
use rand::prelude::*;
use rand::Rng;
use std::collections::HashSet;

use crate::combat;
use crate::entities::entity::EntityTrait;
use crate::entities::monster::{self, MonsterManager};
use crate::entities::player::Player;

use crate::equipments::equipment::{Equipment, EquipmentManager, EquipmentType};
use crate::items::item::{Item, ItemManager, ItemType};
use crate::ui::UI;

/**
 * Constantes pour les icônes
 */
const WALL_ICON: &str = "🟧";
const NO_WALL_ICON: &str = "⬛️";
const GOAL_ICON: &str = "👑";
const DEFAULT_ITEM_ICON: &str = "🎁";
const DEAD_PLAYER_ICON: &str = "💀";
const PLAYER_WITH_HAT: &str = "🤠";
const PLAYER_WITH_GLASSES: &str = "🤓";
const COMBAT_ICON: &str = "❌";

/**
 * Structure représentant la grille de jeu
 */
pub struct Grid {
    size: usize,
    player: Player,
    last_movement: char,
    just_flee: bool,
    monsters: MonsterManager,
    items: ItemManager,
    equipments: EquipmentManager,
    goal: (usize, usize),
    walls: Vec<(usize, usize)>,
    visible_walls: HashSet<(usize, usize)>,
    map_to_display: Vec<Vec<String>>,
    ui: UI,
}

/**
 * Implémentation de la grille
 */
impl Grid {
    /**
     * Constructeur pour initialiser une nouvelle grille
     * @param size Taille de la grille
     * @param ui Référence mutable à l'instance de UI
     * @return Une nouvelle instance de Grid
     */
    pub fn new(size: usize, ui: UI) -> Self {
        let mut rng = rand::thread_rng();
        let goal = (
            rng.gen_range((size * 3 / 4)..size),
            rng.gen_range((size * 3 / 4)..size),
        );

        let map_to_display = vec![vec![NO_WALL_ICON.to_string(); size]; size];

        Grid {
            size,
            player: Player::new((0, 0)).unwrap(),
            last_movement: ' ',
            just_flee: false,
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

    /**
     * Initialise la grille en plaçant les murs, les objets, les ennemis et le joueur
     */
    pub fn init(&mut self) {
        self.place_walls();
        self.place_items((self.size * self.size) / 50).unwrap();
        self.place_equipments((self.size * self.size) / 50).unwrap();
        self.place_monsters((self.size * self.size) / 100).unwrap();
        self.update_ui();
    }

    /**
     * Met à jour l'interface utilisateur
     */
    fn update_ui(&mut self) {
        self.ui.update_map(self.map_to_display.clone());
        self.ui
            .update_equipments(self.player.get_equipment().clone());
        self.ui.update_items(self.player.get_items().clone());
    }

    /**
     * Génère un labyrinthe en utilisant un algorithme de parcours en profondeur
     * @return Un vecteur 2D représentant le labyrinthe
     */
    fn generate_maze(&self) -> Vec<Vec<u8>> {
        // Initialisation de la grille
        let mut maze = vec![vec![0; self.size]; self.size];

        // Initialisation des variables de départ et d'arrivée
        let start = self.player.get_position();
        let stop = self.goal;

        // Initialisation de la pile et du générateur de nombres aléatoires
        let mut stack = vec![start];
        let mut rng = rand::thread_rng();

        // Cette fonction génère un labyrinthe en utilisant l'algorithme de parcours en profondeur (Depth-First Search).
        // Elle utilise une pile pour suivre les cellules à visiter et un vecteur pour stocker les voisins non visités.

        // Tant qu'il y a des éléments dans la pile
        while let Some((x, y)) = stack.pop() {
            // Marque la cellule actuelle comme faisant partie du chemin du labyrinthe
            maze[y][x] = 1;
            let mut neighbors = vec![];

            // Vérifie les voisins non visités (cellules à deux cases de distance) et les ajoute à la liste des voisins
            if x > 1 && maze[y][x - 2] == 0 {
                neighbors.push((x - 2, y));
            }
            if x < self.size - 2 && maze[y][x + 2] == 0 {
                neighbors.push((x + 2, y));
            }
            if y > 1 && maze[y - 2][x] == 0 {
                neighbors.push((x, y - 2));
            }
            if y < self.size - 2 && maze[y + 2][x] == 0 {
                neighbors.push((x, y + 2));
            }

            // Si des voisins non visités existent, en choisit un au hasard
            if let Some(&(nx, ny)) = neighbors.choose(&mut rng) {
                // Remet la cellule actuelle dans la pile pour la revisiter plus tard
                stack.push((x, y));
                // Ajoute le voisin choisi à la pile
                stack.push((nx, ny));
                // Marque le mur entre la cellule actuelle et le voisin choisi comme faisant partie du chemin
                maze[(y + ny) / 2][(x + nx) / 2] = 1;
            }
        }
        // Marque la cellule de fin comme faisant partie du chemin du labyrinthe
        maze[stop.1][stop.0] = 1;
        // Retourne le labyrinthe généré
        maze
    }

    /**
     * Place les murs sur la grille en utilisant le labyrinthe généré
     */
    pub fn place_walls(&mut self) {
        let mut maze = self.generate_maze();

        // Retire quelques murs pour ajouter un peu de difficulté
        let mut rng = rand::thread_rng();
        for x in 0..self.size {
            for y in 0..self.size {
                if rng.gen_range(0..100) < 5 {
                    maze[y][x] = 1;
                }
            }
        }

        // Ajoute les murs à la grille
        for x in 0..self.size {
            for y in 0..self.size {
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
    pub fn place_items(&mut self, count: usize) -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            loop {
                let position = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));

                if self.is_position_empty(position) {
                    self.items
                        .add(Item::new(Item::random(), (position.0, position.1))?);
                    break;
                }
            }
        }
        Ok(())
    }

    /**
     * Place des equipements aléatoirement sur la grille
     * @param count Nombre d'objets à placer
     */
    pub fn place_equipments(&mut self, count: usize) -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            loop {
                let position = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));

                if self.is_position_empty(position) {
                    self.equipments.add(Equipment::new(
                        Equipment::random(),
                        (position.0, position.1),
                    )?);
                    break;
                }
            }
        }
        Ok(())
    }

    /**
     * Place des monstres aléatoirement sur la grille
     * @param count Nombre d'ennemis à placer
     */
    pub fn place_monsters(&mut self, count: usize) -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            loop {
                let position = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
                if self.is_position_empty(position) {
                    self.monsters
                        .add(monster::get_random_monster((position.0, position.1)));
                    break;
                }
            }
        }
        Ok(())
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
        if self.player.has_equipment(EquipmentType::Shoes) {
            if let Err(e) = self.ui.display_game_view_and_message(vec![
                "".to_string(),
                "--------------------- Déplacement ----------------------".to_string(),
                "(z : hauts, q : gauche, s : bas, d : droite, c : suicide)".to_string(),
                "(Z : hauts, Q : gauche, S : bas, D : droite - Chaussures)".to_string(),
                "Appuyer sur entré pour valider".to_string(),
            ]) {
                eprintln!("Error displaying game view: {}", e);
            }
        } else {
            if let Err(e) = self.ui.display_game_view_and_message(vec![
                "".to_string(),
                "--------------------- Déplacement ----------------------".to_string(),
                "(z : hauts, q : gauche, s : bas, d : droite, c : suicide)".to_string(),
                "Appuyer sur entré pour valider".to_string(),
            ]) {
                eprintln!("Error displaying game view: {}", e);
            }
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
        if (distance <= visibility_range) && self.walls.contains(&position) {
            self.visible_walls.insert(position);
            true
        } else {
            self.visible_walls.contains(&position)
        }
    }

    /**
     * Vérifies si il y a un item à la position du joueur
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
     * Vérifies si il y a un équipement à la position du joueur
     */
    pub fn check_for_equipment(&mut self) {
        if let Some(equipment) = self.equipments.get_mut(self.player.get_position()) {
            if equipment.get_position() == self.player.get_position() {
                if !equipment.is_equiped() {
                    if self.player.has_equipment(equipment.get_type()) {
                        self.player.add_item(
                            Item::new(ItemType::HealingPotion, equipment.get_position()).unwrap(),
                        );
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
     * Vérifies si il y a un monstre à la position du joueur et déclanche le combat dans le cas échéant
     * @param can_flee : si le joueur à droit de fuire le combat
     */
    pub fn check_for_combat(&mut self, can_flee: bool) {
        let mut flee = false;
        for monster in self
            .monsters
            .within_range(self.player.get_position(), self.player.get_range())
        {
            if monster.get_position() == self.player.get_position() && monster.is_visible() {
                if combat::start_combat(can_flee, &mut self.player, &mut *monster, &mut self.ui) {
                    monster.set_visible(false);
                } else {
                    if can_flee {
                        flee = true;
                    }
                }
            }
        }

        if flee {
            self.flee();
        }
    }

    /**
     * Déplace les monstres vers le joueur
     */
    pub fn move_monsters(&mut self) {
        let player_position = self.player.get_position();
        let mut new_positions = Vec::new();
        let monster_positions: Vec<_> = self
            .monsters
            .get_all_mut()
            .iter()
            .map(|m| m.get_position())
            .collect();
        if !self.just_flee {
            for (i, monster) in self.monsters.get_all_mut().iter_mut().enumerate() {
                let (mx, my) = monster_positions[i];
                let (px, py) = player_position;

                let mut possible_moves = Vec::new();

                // Check each possible move and ensure it is within grid boundaries
                if mx + 1 < self.size {
                    possible_moves.push((mx + 1, my));
                }
                if mx > 0 {
                    possible_moves.push((mx - 1, my));
                }
                if my + 1 < self.size {
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
            // self.check_for_combat(false);
        }
    }

    /**
     * Déplace le joueur en fonction de l'entrée utilisateur
     * @param movement La direction du déplacement
     */
    pub fn move_player(&mut self, movement: char) {
        let (x, y) = self.player.get_position();

        let new_position;

        if self.player.has_equipment(EquipmentType::Shoes) {
            new_position = match movement {
                'z' if y > 0 => (x, y - 1),             // Move up
                'q' if x > 0 => (x - 1, y),             // Move left
                's' if y < self.size - 1 => (x, y + 1), // Move down
                'd' if x < self.size - 1 => (x + 1, y), // Move right

                'Z' if y > 1 => (x, y - 2),             // Move up x2
                'Q' if x > 1 => (x - 2, y),             // Move left x2
                'S' if y < self.size - 2 => (x, y + 2), // Move down x2
                'D' if x < self.size - 2 => (x + 2, y), // Move right x2
                _ => return,
            };
        } else {
            new_position = match movement {
                'z' if y > 0 => (x, y - 1),             // Move up
                'q' if x > 0 => (x - 1, y),             // Move left
                's' if y < self.size - 1 => (x, y + 1), // Move down
                'd' if x < self.size - 1 => (x + 1, y), // Move right
                _ => return,
            };
        }

        if !self.walls.contains(&new_position) {
            self.player.set_position(new_position);
        }

        self.last_movement = movement;
        self.just_flee = false;
    }

    /**
     * Ajoute des points de vie au joueur
     * @param amount Nombre de points de vie à ajouter
     */
    pub fn heal_player(&mut self, amount: i32) {
        self.player.heal(amount);
        println!(
            "{} a été soigné de {} points de vie",
            self.player.get_name(),
            amount
        );
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
            'Z' => self.move_player('S'),
            'Q' => self.move_player('D'),
            'S' => self.move_player('Z'),
            'D' => self.move_player('Q'),
            _ => {}
        };
        self.just_flee = true;
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

    /**
     * Construit la carte à afficher
     */
    pub fn build_map(&mut self) {
        let mut position;
        {
            for y in 0..self.size {
                for x in 0..self.size {
                    // Pour chaque position x,y du tableau à afficher
                    position = (x, y);
                    if self.should_display_wall(position) {
                        // Ajout de l'icone du mur si il y a un mur dans le tableau à afficher
                        self.map_to_display[x][y] = WALL_ICON.to_string();
                    } else if self.player.get_position() == position {
                        // Ajout de l'icone du joueur dans le tableau à afficher
                        if self.player.is_dead() {
                            self.map_to_display[x][y] = DEAD_PLAYER_ICON.to_string();
                        } else {
                            self.map_to_display[x][y] = self.player.get_icon().to_string();
                        }
                    } else if self.goal == position {
                        // Ajout de l'icone du but dans le tableau à afficher
                        self.map_to_display[x][y] = GOAL_ICON.to_string();
                    } else {
                        // Ajout de l'icone des chemins (la ou il n'y a pas de murs) dans le tableau à afficher
                        self.map_to_display[x][y] = NO_WALL_ICON.to_string();
                    }
                }
            }
        }

        // Création d'un tableau de tous les équipements à porté de la vision du joueur
        let equipment_within_range = self
            .equipments
            .within_range(self.player.get_position(), self.player.get_range());

        // Création d'un tableau de tous les items à porté de la vision du joueur
        let items_within_range = self
            .items
            .within_range(self.player.get_position(), self.player.get_range());

        // Création d'un tableau de tous les monstres à porté de la vision du joueur
        let monsters_within_range = self
            .monsters
            .within_range(self.player.get_position(), self.player.get_range());

        let monster_positions: HashSet<_> = monsters_within_range
            .iter()
            .map(|m| m.get_position())
            .collect();

        for item in items_within_range {
            if item.is_visible()
                && !monster_positions.contains(&item.get_position())
                && self.player.get_position() != item.get_position()
            {
                if self.player.has_equipment(EquipmentType::Glasses) {
                    // Si le joueur porte des lunnettes ajout de l'icone de l'item dans le tableau à afficher
                    self.map_to_display[item.get_position().0][item.get_position().1] =
                        item.get_icon().to_string();
                } else {
                    // Ajout de l'icone part défaut dans le tableau à afficher
                    self.map_to_display[item.get_position().0][item.get_position().1] =
                        DEFAULT_ITEM_ICON.to_string();
                }
            }
        }
        
        // Pour tous les équipements dans la vision du joueur
        for equipment in equipment_within_range {
            if equipment.is_visible()
                && !monster_positions.contains(&equipment.get_position())
                && self.player.get_position() != equipment.get_position()
            {
                if self.player.has_equipment(EquipmentType::Glasses) {
                    // Si le joueur porte des lunnettes ajout de l'icone de l'équipement dans le tableau à afficher
                    self.map_to_display[equipment.get_position().0][equipment.get_position().1] =
                    equipment.get_icon().to_string();
                } else {
                    // Ajout de l'icone part défaut dans le tableau à afficher
                    self.map_to_display[equipment.get_position().0][equipment.get_position().1] =
                    DEFAULT_ITEM_ICON.to_string();
                }
            }
        }
        
        // Pour tous les monstres dans la vision du joueur
        for monster in monsters_within_range.iter() {
            if monster.is_visible() {
                if self.player.get_position() == monster.get_position() {
                    // Ajout de l'icone du combat dans le tableau à afficher si il y a un monstre à l'emplacement du joueur
                    self.map_to_display[monster.get_position().0][monster.get_position().1] =
                    COMBAT_ICON.to_string();
                } else {
                    // Ajout de l'icone du monstre dans le tableau à afficher
                    self.map_to_display[monster.get_position().0][monster.get_position().1] =
                        monster.get_icon().to_string();
                }
            }
        }
    }
}
