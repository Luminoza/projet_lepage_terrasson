/**
 * Module utils
 * Utile pour gérer les fonctions utilitaires
 * 
 * Auteur : Antonin TERRASSON
 */

/**
 * Fonction pour lire un nombre depuis l'entrée standard
 */
pub fn read_number() -> usize {
    use std::io::{self, Write};

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<usize>().unwrap()
}

/**
 * Fonction pour lire un caractère depuis l'entrée standard
 */
pub fn read_key() -> char {
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
// pub fn read_active_key() -> char {
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
