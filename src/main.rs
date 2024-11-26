mod combat;
mod entities;
mod equipments;
mod grid;
mod items;

use crossterm::event;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use grid::Grid;

fn main() {
    println!("\n\n==========================================================================");
    println!("=== Bienvenue dans l'aventure RPG Indiana Jones (TaTala Ta TataLAAAAA) ===");
    println!("==========================================================================\n");

    println!("Votre mission : Atteignez l'artefact caché dans le labyrinthe, mais prennez garde aux monstres !");
    println!("Des artefacts secondaires peuvent vous aider à survivre...\n");

    let width = 31;
    let height = 31;
    let mut grid = Grid::new(width, height);

    grid.init();

    loop {
        grid.display();

        if grid.has_won() {
            println!("\nBravo ! Vous avez trouvé l'artefact !");
            println!(
                "\n==========================================================================\n"
            );
            break;
        } else if grid.has_lost() {
            println!("Game Over ! Vous êtes mort...");
            break;
        }

        println!("\nEntrez votre déplacement (z : hauts, q : gauche, s : bas, d : droite, c : suicide) :");
        
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
            println!(
                "\nIndiana à préféré se suicider que d'essayer de survivre dans ce labyrinthe..."
            );
            println!(
                "\n=============================================================================\n"
            );
            std::process::exit(0);
        }

        grid.move_player(movement);
        grid.check_for_item();
        grid.check_for_equipment();
        grid.check_for_monster();
    }
}
