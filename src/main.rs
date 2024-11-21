use clap::{Arg, Command};
use rand::Rng;
use std::iter;

fn main() {
    // Configuración de CLI con clap
    let matches = Command::new("Generador de Contraseñas Seguras")
        .version("1.0")
        .author("Tu Nombre <tu.email@ejemplo.com>")
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
        .get_matches();

    // Obtención de valores
    let length: usize = matches
        .get_one::<String>("length")
        .unwrap()
        .parse()
        .expect("Por favor, introduce un número válido para la longitud.");
    let use_symbols = matches.get_flag("symbols");
    let use_numbers = matches.get_flag("numbers");
    let use_uppercase = matches.get_flag("uppercase");

    // Generación de contraseña
    let password = generate_password(length, use_symbols, use_numbers, use_uppercase);
    println!("Contraseña generada: {}", password);
}

fn generate_password(length: usize, symbols: bool, numbers: bool, uppercase: bool) -> String {
    // Base de caracteres
    let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");

    if uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if numbers {
        charset.push_str("0123456789");
    }
    if symbols {
        charset.push_str("!@#$%^&*()-_=+[]{}|;:',.<>?/");
    }

    // Generación aleatoria
    let mut rng = rand::thread_rng();
    iter::repeat_with(|| {
        charset
            .chars()
            .nth(rng.gen_range(0..charset.len()))
            .unwrap()
    })
    .take(length)
    .collect()
}
