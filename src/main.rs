/**
 * Module main
 * Fichier principal du jeu
 * 
 * Auteur : Antonin TERRASSON & Nathan LEPAGE
 */

/**
 * Importation des modules
 */
mod combat;
mod entities;
mod equipments;
mod grid;
mod items;

use grid::Grid;
mod ui;

use std::sync::{Arc, Mutex};
use std::thread;

/**
 * Fonction principale
 */
fn main() {

    // Affichage des messages de bienvenue et de demande de taille de la carte
    ui::display_welcome_message();
    ui::display_map_size();
    let size = read_number();

    // Initialisation de la grille et de l'interface utilisateur
    let ui = ui::UI::new(size);
    let grid = Arc::new(Mutex::new(Grid::new(size, ui)));

    grid.lock().unwrap().init();

    // Clonage de la grille pour les différents threads
    let grid_player = Arc::clone(&grid);
    let grid_monster = Arc::clone(&grid);
    let grid_heath = Arc::clone(&grid);

    // Création de variables pour les threads
    let player_moved = Arc::new(Mutex::new(false));
    let monster_moved = Arc::new(Mutex::new(false));

    // Clonage des variables pour les différents threads
    let player_moved_clone = Arc::clone(&player_moved);
    let monster_moved_clone = Arc::clone(&monster_moved);

    // Thread pour gérer les actions du joueur
    let move_player_thread = thread::spawn(move || {
        loop {
            // let movement = read_active_key();
            let movement = read_key();
            if movement == 'c' {
                ui::display_suicide_message();
                std::process::exit(0);
            }
            grid_player.lock().unwrap().move_player(movement);
            *player_moved_clone.lock().unwrap() = true;
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    // Thread pour gérer les mouvements des monstres
    let move_monster_thread = thread::spawn(move || {
        loop {
            thread::sleep(std::time::Duration::from_millis(1000));
            grid_monster.lock().unwrap().move_monsters();
            *monster_moved_clone.lock().unwrap() = true;
        }
    });

    // Thread pour ajouter 10 points de vie au joueur toutes les 10 sec
    let heal_player_thread = thread::spawn(move || {
        loop {
            thread::sleep(std::time::Duration::from_millis(10000));
            grid_heath.lock().unwrap().heal_player(10);
        }
    }); 

    loop {

        grid.lock().unwrap().check_for_combat(true);
        grid.lock().unwrap().check_for_item();
        grid.lock().unwrap().check_for_equipment();

        // Si le joueur a gagné ou perdu, on affiche un message et on quitte le jeu
        if grid.lock().unwrap().has_won() {
            ui::display_victory_message();
            std::process::exit(0);
        } else if grid.lock().unwrap().has_lost() {
            ui::display_game_over_message();
            std::process::exit(0);
        }

        // Affichage de la grille si le joueur ou un monstre a bougé
        let mut player_moved = player_moved.lock().unwrap();
        let mut monster_moved = monster_moved.lock().unwrap();

        if *player_moved || *monster_moved {
            grid.lock().unwrap().display();
            *player_moved = false;
            *monster_moved = false;
        }
    }
}

/**
 * Fonction pour lire un nombre depuis l'entrée standard
 */
fn read_number() -> usize {
    use std::io::{self, Write};

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<usize>().unwrap()
}

/**
 * Fonction pour lire un caractère depuis l'entrée standard
 */
fn read_key() -> char {
    use std::io::{self, Write};

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let key = input.chars().next().unwrap();
    key
}

// /**
//  * Fonction pour lire un caractère en continu depuis l'entrée standard
//  */
// fn read_active_key() -> char {
//     use crossterm::event::{self, KeyCode};
//     use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
//     enable_raw_mode().unwrap();
//     let key = match event::read().unwrap() {
//         event::Event::Key(event::KeyEvent { code: KeyCode::Char(c), .. }) => c,
//         _ => ' ',
//     };
//     disable_raw_mode().unwrap();
//     key
// }
