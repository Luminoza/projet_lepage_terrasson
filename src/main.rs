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
    println!("\n\n==========================================================================");
    println!("=== Bienvenue dans l'aventure RPG Indiana Jones (TaTala Ta TataLAAAAA) ===");
    println!("==========================================================================\n");
    println!("Votre mission : Atteignez l'artefact caché dans le labyrinthe, mais prennez garde aux monstres !");
    println!("Des artefacts secondaires peuvent vous aider à survivre...\n");

    // Crée la carte et le joueur
    let width = 10;
    let height = 10;
    let mut grid = Grid::new(width, height); // Grille 10x10
    
    let mut player = Player::new("Joueur".to_string(), 100);

    // Calculer le nombre de murs, d'objets et de monstres en fonction de la taille de la carte
    let num_walls = (width * height) / 4;
    let num_items = (width * height) / 10;
    let num_enemies = (width * height) / 8;

    // Ajouter des ennemis, des objets et des murs
    grid.place_random_enemies(num_enemies);
    grid.place_random_items(num_items);
    grid.place_random_walls(num_walls);

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
        let player = player_shared.lock().unwrap();
        grid.display(&player);
        drop(player); // Libérer le verrou avant de reprendre le verrou plus tard

        let mut player = player_shared.lock().unwrap();
        if player.is_dead() {
            println!("Game Over ! Vous êtes mort...");
            break;
        }

        if grid.has_won() {
            println!("\nBravo ! Vous avez trouvé l'artefact !");
            break;
        }

        // Déplacement du joueur
        grid.move_player(&mut player);

        // Vérification des rencontres
        if let Some(enemy) = grid.check_for_enemy() {
            println!("\nUn monstres apparaît !");
            combat::start_combat(&mut player, enemy);
            grid.remove_enemy_at_player_position();
        }

        // Vérification des objets
        if let Some(item) = grid.check_for_item() {
            println!("\nVous trouvez un objet : {} !", item.name);
            player.pick_item(item);
            grid.remove_item_at_player_position();
        }
    }
}
