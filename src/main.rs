mod combat;
mod entities;
mod equipments;
mod grid;
mod items;
mod utils;
mod ui;

use crossterm::event;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use grid::Grid;

fn main() {
    ui::display_welcome_message();

    let width = 31;
    let height = 31;
    let mut ui = ui::UI::new(width, height);
    let mut grid = Grid::new(width, height, ui);

    grid.init();
    grid.display();

    loop {
        if grid.has_won() {
            ui::display_victory_message();
            break;
        } else if grid.has_lost() {
            ui::display_game_over_message();
            break;
        }
        
        enable_raw_mode().unwrap();
        let movement = match event::read().unwrap() {
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            }) => c,
            _ => continue,
        };
        disable_raw_mode().unwrap();
        
        if movement == 'c' {
            ui::display_suicide_message();
            std::process::exit(0);
        }
        
        grid.move_player(movement);
        grid.check_for_item();
        grid.check_for_equipment();
        grid.check_for_monster();
        grid.display();
    }
}
