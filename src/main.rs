mod grid;
mod player;
mod combat;
mod utils;

use grid::Grid;
use player::Player;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Bienvenue dans l'aventure RPG Indiana Jones (TaTala Ta TataLAAAAA) ===");
    println!("Votre mission : Atteignez l'artefact caché dans le labyrinthe, mais prennez garde aux monstres !");
    println!("Des artefacts secondaires peuvent vous aider à survivre...");

    // Crée la carte et le joueur
    let mut grid = Grid::new(10, 10); // Grille 10x10
    
    let mut player = Player::new("Héros".to_string(), 100);

    // Ajouter des ennemis et des objets et les murs
    grid.place_random_enemies(5);
    grid.place_random_items(3);
    grid.place_random_walls(20);

    // Mutex pour gérer la concurrence
    let player_shared = Arc::new(Mutex::new(player));

    // Thread pour un timer ou des événements aléatoires
    let player_clone = Arc::clone(&player_shared);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(10));
            let mut player = player_clone.lock().unwrap();
            player.restore_health(10); // Restaure la vie toutes les 10 secondes
            println!("[INFO] Un peu de repos : +10 points de vie !");
        }
    });

    // Boucle principale du jeu
    loop {
        grid.display();
        let mut player = player_shared.lock().unwrap();
        if player.is_dead() {
            println!("Game Over ! Vous êtes mort...");
            break;
        }

        if grid.has_won() {
            println!("Bravo ! Vous avez trouvé l'artefact !");
            break;
        }

        // Déplacement du joueur
        grid.move_player(&mut player);

        // Vérification des rencontres
        if let Some(enemy) = grid.check_for_enemy() {
            println!("Un monstres apparaît !");
            combat::start_combat(&mut player, enemy);
        }

        // Vérification des objets
        if let Some(item) = grid.check_for_item() {
            println!("Vous trouvez un objet : {} !", item.name);
            player.pick_item(item);
        }
    }
}
