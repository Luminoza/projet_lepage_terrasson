use std::collections::HashSet;
use rand::Rng;
use rand::prelude::SliceRandom;
use crate::player::{Player, Enemy, Item};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct Grid {
    width: usize,
    height: usize,
    player_position: (usize, usize),
    enemies: Vec<Enemy>,
    items: Vec<Item>,
    goal: (usize, usize),
    walls: Vec<(usize, usize)>,
    visible_walls: HashSet<(usize, usize)>,
    visible_items: HashSet<(usize, usize)>,
}

impl Grid {

    pub fn new(width: usize, height: usize) -> Self {
        let goal = (width - 1, height - 1);
        let mut grid = Grid {
            width,
            height,
            player_position: (0, 0),
            enemies: vec![],
            items: vec![],
            goal,
            walls: vec![],
            visible_walls: HashSet::new(),
            visible_items: HashSet::new(),
        };
        grid.place_walls();
        // grid.place_items((width * height) / 20);
        // grid.place_enemies((width * height) / 10);
        grid
    }

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

    pub fn place_walls(&mut self) {
        let mut maze = self.generate_maze();

        let mut rng = rand::thread_rng();
        for x in 0..self.width {
            for y in 0..self.height {
                if (x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1) && !(x == 0 && y == 0) && !(x == self.width - 1 && y == 0) && !(x == 0 && y == self.height - 1) && !(x == self.width - 1 && y == self.height - 1) {
                    if rng.gen_range(0..1) == 1 {
                        maze[y][x] = 1;
                    }
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

    pub fn place_items(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            loop {
                let position = (
                    rng.gen_range(0..self.width),
                    rng.gen_range(0..self.height),
                );

                if self.is_position_empty(position) {
                    self.items.push(Item::random(position.0, position.1));
                    break;
                }
            }
        }
    }

    pub fn place_enemies(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            loop {
                let position = (
                    rng.gen_range(0..self.width),
                    rng.gen_range(0..self.height),
                );
                
                if self.is_position_empty(position) {
                    self.enemies.push(Enemy::random(position.0, position.1));
                    break;
                }
            }
        }
    }

    fn is_position_empty(&self, position: (usize, usize)) -> bool {
        position != self.player_position
            && position != self.goal
            && !self.walls.contains(&position)
            && !self.items.iter().any(|i| i.position == position)
            && !self.enemies.iter().any(|e| e.position == position)
    }

    pub fn display(&mut self, player: &Player) {
        println!("Carte (😃 = joueur, 🎁 = objet, 👑 = artefact, 🐉 = ennemi, 🟥 = mur) : \n");
        for y in 0..self.height {
            for x in 0..self.width {
                if self.player_position == (x, y) {
                    if player.has_hat() {
                        print!("🤠");
                    } else {
                        print!("😃");
                    }
                } else if self.goal == (x, y) {
                    print!("👑");
                } else if self.should_display_wall(player, (x, y)) {
                    print!("🟥");
                } else if self.should_display_item(player, (x, y)) {
                    print!("🎁");
                } else if self.should_display_enemy(player, (x, y)) {
                    print!("🐉");
                } else {
                    print!("⬛️");
                }
            }
            println!();
        }
    }

    fn should_display_wall(&mut self, player: &Player, position: (usize, usize)) -> bool {
        let distance = ((self.player_position.0 as isize - position.0 as isize).abs().max((self.player_position.1 as isize - position.1 as isize).abs())) as usize;
        let visibility_range = if player.has_hat() { 5 } else { 2 };
        if distance <= visibility_range && self.walls.contains(&position) {
            self.visible_walls.insert(position);
            true
        } else {
            self.visible_walls.contains(&position)
        }
    }

    fn should_display_item(&mut self, player: &Player, position: (usize, usize)) -> bool {
        let distance = ((self.player_position.0 as isize - position.0 as isize).abs().max((self.player_position.1 as isize - position.1 as isize).abs())) as usize;
        let visibility_range = if player.has_hat() { 5 } else { 2 };
        if distance <= visibility_range && self.items.iter().any(|i| i.position == position) {
            self.visible_items.insert(position);
            true
        } else {
            self.visible_items.contains(&position)
        }
    }

    fn should_display_enemy(&self, player: &Player, position: (usize, usize)) -> bool {
        let distance = ((self.player_position.0 as isize - position.0 as isize).abs().max((self.player_position.1 as isize - position.1 as isize).abs())) as usize;
        let visibility_range = if player.has_hat() { 5 } else { 2 };
        self.enemies.iter().any(|e| e.position == position && distance <= visibility_range)
    }

    pub fn remove_enemy_at_player_position(&mut self) {
        self.enemies.retain(|e| e.position != self.player_position);
    }

    pub fn remove_item_at_player_position(&mut self) {
        self.items.retain(|i| i.position != self.player_position);
        self.visible_items.remove(&self.player_position);
    }

    pub fn move_player(&mut self, player: &mut Player) {
        println!("\nEntrez votre déplacement (z = hauts, q = gauche, s = bas, d = droite) :");

        let direction = self.read_input();

        let (mut x, mut y) = self.player_position;
        let new_position = match direction {
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
            self.player_position = new_position;
        } else {
            println!("\nVous ne pouvez pas traverser un mur !\n");
        }
    }

    fn read_input(&self) -> char {
        enable_raw_mode().unwrap();
        let result = loop {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(c) => break c,
                    _ => {}
                }
            }
        };
        disable_raw_mode().unwrap();
        result
    }

    pub fn check_for_item(&self) -> Option<Item> {
        self.items
            .iter()
            .find(|i| i.position == self.player_position)
            .cloned()
    }

    pub fn check_for_enemy(&self) -> Option<Enemy> {
        self.enemies
            .iter()
            .find(|e| e.position == self.player_position)
            .cloned()
    }

    pub fn has_won(&self) -> bool {
        self.player_position == self.goal
    }
}
