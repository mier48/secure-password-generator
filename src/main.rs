// main.rs
use clap::{Arg, Command};
use rand::Rng;
use std::iter;

/// Configuración de la CLI con Clap
fn build_cli() -> Command {
    Command::new("Generador de Contraseñas Seguras")
        .version("1.0")
        .author("Alberto Mier <info@albertomier.com>")
        .about("Genera contraseñas seguras con opciones personalizables")
        .arg(
            Arg::new("length")
                .short('l')
                .long("length")
                .value_name("NUM")
                .help("Longitud de la contraseña")
                .default_value("16"),
        )
        .arg(
            Arg::new("symbols")
                .short('s')
                .long("symbols")
                .help("Incluir símbolos en la contraseña")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("numbers")
                .short('n')
                .long("numbers")
                .help("Incluir números en la contraseña")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("uppercase")
                .short('u')
                .long("uppercase")
                .help("Incluir mayúsculas en la contraseña")
                .action(clap::ArgAction::SetTrue),
        )
}

/// Genera una contraseña segura basada en las opciones proporcionadas
fn generate_password(length: usize, use_symbols: bool, use_numbers: bool, use_uppercase: bool) -> String {
    // Base de caracteres
    let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");

    if use_uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if use_numbers {
        charset.push_str("0123456789");
    }
    if use_symbols {
        charset.push_str("!@#$%^&*()-_=+[]{}|;:',.<>?/");
    }

    // Validación de que el conjunto de caracteres no esté vacío
    if charset.is_empty() {
        panic!("El conjunto de caracteres está vacío. Activa al menos una opción.");
    }

    // Generación aleatoria de la contraseña
    let mut rng = rand::thread_rng();
    iter::repeat_with(|| {
        let idx = rng.gen_range(0..charset.len());
        charset.chars().nth(idx).unwrap()
    })
    .take(length)
    .collect()
}

fn main() {
    // Configuración de la CLI
    let matches = build_cli().get_matches();

    // Obtención de valores desde los argumentos
    let length: usize = matches
        .get_one::<String>("length")
        .unwrap()
        .parse()
        .expect("Por favor, introduce un número válido para la longitud.");
    let use_symbols = matches.get_flag("symbols");
    let use_numbers = matches.get_flag("numbers");
    let use_uppercase = matches.get_flag("uppercase");

    // Generación de la contraseña
    let password = generate_password(length, use_symbols, use_numbers, use_uppercase);
    println!("Contraseña generada: {}", password);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_length() {
        let password = generate_password(10, false, false, false);
        assert_eq!(password.len(), 10);
    }

    #[test]
    fn test_password_symbols() {
        let password = generate_password(20, true, false, false);
        assert!(password.chars().any(|c| "!@#$%^&*()-_=+[]{}|;:',.<>?/".contains(c)));
    }

    #[test]
    fn test_password_uppercase() {
        let password = generate_password(15, false, false, true);
        assert!(password.chars().any(|c| c.is_uppercase()));
    }

    #[test]
    fn test_password_numbers() {
        let password = generate_password(12, false, true, false);
        assert!(password.chars().any(|c| c.is_numeric()));
    }
}
