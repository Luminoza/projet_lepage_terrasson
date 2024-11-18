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

    pub fn place_random_items(&mut self, count: usize) {
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

    pub fn place_random_walls(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            loop {
                let position = (
                    rng.gen_range(0..self.width),
                    rng.gen_range(0..self.height),
                );
                if self.is_position_empty(position) {
                    self.walls.push(position);
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

    pub fn display(&self, player: &Player) {
        println!("Carte (😃 = joueur sans chapeau, 🤠 = joueur avec chapeau, 🎁 = objet, 👑 = objectif, ◼️ = mur, 🐉 = ennemi) :");
        for y in 0..self.height {
            for x in 0..self.width {
                if self.player_position == (x, y) {
                    if player.has_hat() {
                        print!("🤠 ");
                    } else {
                        print!("😃 ");
                    }
                } else if self.goal == (x, y) {
                    print!("👑 ");
                } else if self.walls.contains(&(x, y)) {
                    print!("🟥 ");
                } else if self.items.iter().any(|i| i.position == (x, y)) {
                    print!("🎁 ");
                } else if self.should_display_enemy(player, (x, y)) {
                    print!("🐉 ");
                } else {
                    print!("⬛️ ");
                }
            }
            println!();
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
