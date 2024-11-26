mod combat;
mod entities;
mod equipments;
mod grid;
mod items;

use grid::Grid;
mod ui;

use crossterm::event;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    ui::display_welcome_message();

    let width = 31;
    let height = 31;
    let ui = ui::UI::new(width, height);
    let grid = Arc::new(Mutex::new(Grid::new(width, height, ui)));

    grid.lock().unwrap().init();
    
    
    // let grid_clone = Arc::clone(&grid);
    // thread::spawn(move || {
    //     loop {
    //         thread::sleep(Duration::from_secs(1));
    //         let mut grid = grid_clone.lock().unwrap();
    //         grid.move_monster();
    //         grid.display();
    //     }
    // });
    
    // grid.lock().unwrap().display();
    loop {
        let mut grid = grid.lock().unwrap();
        if grid.has_won() {
            ui::display_victory_message();
            break;
        } else if grid.has_lost() {
            ui::display_game_over_message();
            break;
        }
        
        
        if let Err(e) = enable_raw_mode() {
            eprintln!("Erreur lors de l'activation du mode brut: {}", e);
            continue;
        }
        let movement = match event::read() {
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            })) => c,
            Ok(_) => continue,
            Err(e) => {
                eprintln!("Erreur de lecture de l'événement: {}", e);
                continue;
            }
        };
        if let Err(e) = disable_raw_mode() {
            eprintln!("Erreur lors de la désactivation du mode brut: {}", e);
        }
        
        if movement == 'c' {
            ui::display_suicide_message();
            std::process::exit(0);
        }
        
        grid.move_player(movement);
        // grid.move_monster();
        grid.display();
        grid.check_for_item();
        grid.check_for_equipment();
        grid.check_for_monster();
    }
}
