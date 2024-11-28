mod combat;
mod entities;
mod equipments;
mod grid;
mod items;

use grid::Grid;
mod ui;

use std::sync::{Arc, Mutex};

fn main() {

    ui::display_welcome_message();

    ui::display_map_size();
    let size = read_key();

    let ui = ui::UI::new(size);
    let grid = Arc::new(Mutex::new(Grid::new(size, ui)));

    grid.lock().unwrap().init();
    grid.lock().unwrap().display();

    loop {

        let mut grid = grid.lock().unwrap();

        if grid.has_won() {
            ui::display_victory_message();
            break;
        } else if grid.has_lost() {
            ui::display_game_over_message();
            break;
        }

        let movement = read_active_key();
        if movement == 'c' {
            ui::display_suicide_message();
            std::process::exit(0);
        }

        grid.move_player(movement);

        grid.display();
        grid.check_for_combat(true);
        grid.check_for_item();
        grid.check_for_equipment();

        grid.move_monster();
    }
}

fn read_key() -> usize {
    use std::io::{self, Write};

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let key = input.trim().parse().unwrap_or(0);
    key

}

fn read_active_key() -> char {
    use crossterm::event;
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

    if let Err(e) = enable_raw_mode() {
        eprintln!("Erreur lors de l'activation du mode brut: {}", e);
    }

    let movement = match event::read() {
        Ok(event::Event::Key(event::KeyEvent {
            code: event::KeyCode::Char(c),
            ..
        })) => c,
        Ok(_) => ' ',
        Err(e) => {
            eprintln!("Erreur de lecture de l'événement: {}", e);
            ' '
        }
    };

    if let Err(e) = disable_raw_mode() {
        eprintln!("Erreur lors de la désactivation du mode brut: {}", e);
    }

    movement
}
