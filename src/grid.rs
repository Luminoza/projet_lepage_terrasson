use rand::Rng;
use crate::player::{Player, Enemy, Item};

pub struct Grid {
    width: usize,
    height: usize,
    player_position: (usize, usize),
    enemies: Vec<Enemy>,
    items: Vec<Item>,
    goal: (usize, usize),
    walls: Vec<(usize, usize)>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let goal = (width - 1, height - 1);
        Grid {
            width,
            height,
            player_position: (0, 0),
            enemies: vec![],
            items: vec![],
            goal,
            walls: vec![],
        }
    }

    pub fn place_random_enemies(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            self.enemies.push(Enemy::random(
                rng.gen_range(0..self.width),
                rng.gen_range(0..self.height),
            ));
        }
    }

    pub fn place_random_items(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            self.items.push(Item::random(
                rng.gen_range(0..self.width),
                rng.gen_range(0..self.height),
            ));
        }
    }

    pub fn place_random_walls(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            self.walls.push((
                rng.gen_range(0..self.width),
                rng.gen_range(0..self.height),
            ));
        }
    }

    pub fn display(&self) {
        println!("Carte (P = joueur, E = ennemi, O = objet, G = objectif, # = mur) :");
        for y in 0..self.height {
            for x in 0..self.width {
                if self.player_position == (x, y) {
                    print!("P ");
                } else if self.goal == (x, y) {
                    print!("G ");
                } else if self.walls.contains(&(x, y)) {
                    print!("◼️ ");
                } else if self.enemies.iter().any(|e| e.position == (x, y)) {
                    print!("E ");
                } else if self.items.iter().any(|i| i.position == (x, y)) {
                    print!("O ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }

    pub fn move_player(&mut self, player: &mut Player) {
        println!("Entrez votre déplacement (z = haut, q = gauche, s = bas, d = droite) :");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
        let direction = input.trim();

        let (mut x, mut y) = self.player_position;
        let new_position = match direction {
            "z" if y > 0 => (x, y - 1),
            "q" if x > 0 => (x - 1, y),
            "s" if y < self.height - 1 => (x, y + 1),
            "d" if x < self.width - 1 => (x + 1, y),
            _ => {
                println!("Mouvement invalide");
                return;
            }
        };
        if !self.walls.contains(&new_position) {
            self.player_position = new_position;
        } else {
            println!("Vous ne pouvez pas traverser un mur !");
        }
    }

    pub fn check_for_enemy(&self) -> Option<Enemy> {
        self.enemies
            .iter()
            .find(|e| e.position == self.player_position)
            .cloned()
    }

    pub fn check_for_item(&self) -> Option<Item> {
        self.items
            .iter()
            .find(|i| i.position == self.player_position)
            .cloned()
    }

    pub fn has_won(&self) -> bool {
        self.player_position == self.goal
    }
}
