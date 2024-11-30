mod combat;
mod entities;
mod equipments;
mod grid;
mod items;

use grid::Grid;
mod ui;

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    ui::display_welcome_message();
    ui::display_map_size();
    let size = read_number();

    let ui = ui::UI::new(size);
    let grid = Arc::new(Mutex::new(Grid::new(size, ui)));

    grid.lock().unwrap().init();

    let grid_player = Arc::clone(&grid);
    let grid_monster = Arc::clone(&grid);
    let grid_1 = Arc::clone(&grid);

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
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    // Thread pour gérer les mouvements des monstres
    let move_monster_thread = thread::spawn(move || {
        loop {
            grid_monster.lock().unwrap().move_monsters();
            thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    loop {

        grid.lock().unwrap().check_for_combat(true);
        grid.lock().unwrap().check_for_item();
        grid.lock().unwrap().check_for_equipment();

        if grid.lock().unwrap().has_won() {
            ui::display_victory_message();
            std::process::exit(0);
        } else if grid.lock().unwrap().has_lost() {
            ui::display_game_over_message();
            std::process::exit(0);
        }
        
        grid.lock().unwrap().display();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn read_number() -> usize {
    use std::io::{self, Write};

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<usize>().unwrap()
}

fn read_key() -> char {
    use std::io::{self, Write};

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let key = input.chars().next().unwrap();
    key
}

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
