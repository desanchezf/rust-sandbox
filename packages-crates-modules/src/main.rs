// use std::io::stdin; Se puede utilizar este en el ejemplo
use std::io::*;

fn main() {
    println!("Escribe tu nombre:");

    let mut nombre = String::new();
    // El stdin pertenece al modulo std::io
    stdin().read_line(&mut nombre).expect("Error al leer la línea");

    println!("Hola, {}!", nombre.trim());
}