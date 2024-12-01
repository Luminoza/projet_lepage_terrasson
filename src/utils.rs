/**
 * Module utils
 * Utile pour gérer les fonctions utilitaires
 * 
 * Auteur : Antonin TERRASSON
 */

/**
 * Fonction pour lire un nombre depuis l'entrée standard
 */
pub fn read_number() -> Result<usize, Box<dyn std::error::Error>> {
    use std::io::{self, Write};

    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let number = input.trim().parse::<usize>()?;
    Ok(number)
}

/**
 * Fonction pour lire un caractère depuis l'entrée standard
 */
pub fn read_key() -> Result<char, Box<dyn std::error::Error>> {
    use std::io::{self, Write};

    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let key = input.chars().next().ok_or("No character read")?;
    Ok(key)
}

// /**
//  * Fonction pour lire un caractère en continu depuis l'entrée standard
//  */
// pub fn read_active_key() -> Result<char, Box<dyn std::error::Error>> {
//     use crossterm::event::{self, KeyCode};
//     use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

//     enable_raw_mode()?;
//     let key = match event::read()? {
//         event::Event::Key(event::KeyEvent { code: KeyCode::Char(c), .. }) => c,
//         _ => ' ',
//     };
//     disable_raw_mode()?;
//     Ok(key)
// }
