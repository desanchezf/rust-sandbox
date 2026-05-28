// use std -> Libreria estandar de Rust

use std::env; // Modulo env para pasar parametros por linea de comandos
use std::fs; // Modulo fs para operaciones sobre archivos/ficheros
use std::process; // Modulo process para salir del programa

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Usage: minigrep <query> <file_path>");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // let arg_1 = &args[1]; // Usamos referencia o la función .clone() pero no = args[1] aunque sea vector
    // let arg_2 = &args[2]; // Usamos referencia o la función .clone() pero no = args[2] aunque sea vector
    // println!("arg-1: {}", arg_1);
    // println!("arg-2: {}", arg_2);

    let config = Config::build(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {error}");
        process::exit(1); // Salimos con el codigo 1 -> Error
    });

    run(config);
}


fn run(config: Config) {

    // Flujo para leer el fichero
    // - Coge el file_path
    // - Abre el fichero
    // - Retorna un valor de tipo std::io::Result<String> -> Result es un enum con los siguientes campos:
    //     - Ok(String) -> Si el fichero se ha leido correctamente
    //     - Err(std::io::Error) -> Si el fichero no se ha podido leer
    //   -> expect("Should have been able to read the file") -> Si el fichero no se ha podido leer, se lanza un error
    let contents = fs::read_to_string(&config.file_path).expect("Should have been able to read the file");
    let mut matches = 0;

    for (i, line) in contents.lines().enumerate() {
        // OJO!! -> Distingue entre mayusculas y minusculas -> Lorem != lorem
        if line.contains(&config.query) {
            let line = line.replace(&config.query, &format!("\x1b[31m{}\x1b[0m", &config.query));
            println!("Coincidence ({i}): {line}");
            matches += 1;
        }
    }
    if matches == 0 {
        println!("No matches found for {} in file {}", &config.query, &config.file_path);
    }
}