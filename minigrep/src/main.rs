// use std -> Libreria estandar de Rust
/// Esto es un programa que busca una cadena de texto dentro de un fichero
/// y muestra las líneas que contienen la cadena de texto.
///
/// Uso:
///     minigrep <query> <file_path>
///
/// Ejemplo:
///     minigrep "rust" src/main.rs
///
/// Opciones:
///     -i, --ignore-case    Ignore case
/// # Examples
/// Pruebote

use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::{print_matches, search, search_case_insensitive};

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // Comprobamos si existe
        let exists_ignore_case = env::var("IGNORE_CASE").is_ok();
        // Asociamos el valor si existe y es 1
        let ignore_case = exists_ignore_case && env::var("IGNORE_CASE").unwrap() == "1";

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let arg_1 = &args[1]; // Usamos referencia o la función .clone() pero no = args[1] aunque sea vector
    // let arg_2 = &args[2]; // Usamos referencia o la función .clone() pero no = args[2] aunque sea vector
    // println!("arg-1: {}", arg_1);
    // println!("arg-2: {}", arg_2);

    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1); // Salimos con el codigo 1 -> Error
    });

    // _ es un comodin para ignorar el valor de retorno de la función
    if let Err(e) = run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// Box<dyn Error> -> Es un trait que significa que el error puede ser de cualquier tipo
fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(&config.file_path)?;
    let matches = if config.ignore_case {
        println!("Searching for case insensitive");
        search_case_insensitive(&config.query, &contents)
    } else {
        println!("Searching for case sensitive");
        search(&config.query, &contents)
    };

    println!("Coincidences found: {}", matches.len());
    print_matches(&config.query, &matches, config.ignore_case);

    Ok(())
}