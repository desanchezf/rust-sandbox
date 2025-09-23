// Ejemplo para mostrar diferencias entre mod y use

// 1. DECLARACIÓN DE MÓDULOS con 'mod'
mod matematicas {
    pub fn sumar(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiplicar(a: i32, b: i32) -> i32 {
        a * b
    }
}

mod utilidades {
    pub fn saludar(nombre: &str) {
        println!("¡Hola, {}!", nombre);
    }
}

// 2. IMPORTACIÓN con 'use' para acceso más fácil
use matematicas::sumar;           // Importa solo 'sumar'
use matematicas::multiplicar;     // Importa solo 'multiplicar'
use utilidades::*;               // Importa todo de 'utilidades'

pub fn ejemplo() {
    // SIN 'use' - necesitas el path completo
    let resultado1 = matematicas::sumar(5, 3);

    // CON 'use' - acceso directo
    let resultado2 = sumar(10, 2);        // Más limpio
    let resultado3 = multiplicar(4, 5);   // Más limpio

    // Función importada con * (wildcard)
    saludar("Rust");

    println!("Resultados: {}, {}, {}", resultado1, resultado2, resultado3);
}


