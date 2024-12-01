/**
 * Module main
 * Fichier principal du jeu
 * 
 * Auteur : Antonin TERRASSON
 */

/**
 * Importation des modules
 */
mod grid;
mod entities;
mod combat;
mod items;
mod equipments;
mod ui;
mod utils;

use grid::Grid;
use utils::{read_number, read_key};

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

/**
 * Fonction principale
 */
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Affichage des messages de bienvenue et de demande de taille de la carte
    ui::display_welcome_message()?;
    ui::display_map_size()?;
    let size = read_number()?;

    // Initialisation de la grille et de l'interface utilisateur
    let ui = ui::UI::new(size);
    let grid = Arc::new(Mutex::new(Grid::new(size, ui)));

    grid.lock().unwrap().init();

    // Affichage de la grille en début de partie
    grid.lock().unwrap().display();

    // Clonage de la grille pour les différents threads
    let grid_player = Arc::clone(&grid);
    let grid_monster = Arc::clone(&grid);
    let grid_heath = Arc::clone(&grid);

    // Création des canaux de communication
    let (tx, rx): (Sender<&str>, Receiver<&str>) = mpsc::channel();

    // Clonage des émetteurs pour les différents threads
    let tx_player = tx.clone();
    let tx_monster = tx.clone();

    // Thread pour gérer les actions du joueur
    thread::spawn(move || {
        loop {
            // let movement = read_active_key();
            let movement = read_key().unwrap();
            if movement == 'c' {
                if let Err(e) = ui::display_suicide_message() {
                    eprintln!("Error displaying suicide message: {}", e);
                }
                std::process::exit(0);
            }
            grid_player.lock().unwrap().move_player(movement);

            tx_player.send("player_moved").unwrap();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    // Thread pour gérer les mouvements des monstres
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(1000));
        grid_monster.lock().unwrap().move_monsters();
        tx_monster.send("monster_moved").unwrap();
    });

    // Thread pour ajouter 10 points de vie au joueur toutes les 10 sec
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(10000));
        grid_heath.lock().unwrap().heal_player(10);
    });

    loop {
        // Attendre un message des threads
        let message = rx.recv().unwrap();

        // Affichage de la grille si le joueur ou un monstre a bougé
        if message == "player_moved" || message == "monster_moved" {
            grid.lock().unwrap().display();
            grid.lock().unwrap().check_for_item();
            grid.lock().unwrap().check_for_equipment();
            grid.lock().unwrap().check_for_combat(true);
            grid.lock().unwrap().display();
        }

        // Si le joueur a gagné ou perdu, on affiche un message et on quitte le jeu
        if grid.lock().unwrap().has_won() {
            ui::display_victory_message()?;
            std::process::exit(0);
        } else if grid.lock().unwrap().has_lost() {
            ui::display_game_over_message()?;
            std::process::exit(0);
        }
    }
}