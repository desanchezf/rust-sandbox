use std::io;
use rand::Rng;


fn main() {

    let mut number_a: String = String::new();
    let mut number_b: Option<i8> = None;

    // Leemos el número
    println!("Ingrese el primer numero: ");
    io::stdin()
        .read_line(&mut number_a)
        .expect("Fallo al leer la linea");

    // Hacemos un casting
    let number_a: i8 = number_a.trim()  // En caso de poner usize, no tiene sentido porque siempre son positivos
        .parse()
        .expect("Valor introducido incorrecto");

    // Comparación con if (teniendo valor siempre)
    if number_a > 0 {
        println!("El número es positivo");
    }
    if number_a < 0 {
        println!("El número es negativo");
    }
    if number_a == 0 {
        println!("El número es cero");
    }


    // Comparación para ver si tiene valor o no una variable
    // Si utilizamos los valores None, la comparación se realiza con Match
    match number_b {
        Some(number_b) => println!("La variable contiene un valor: {}", number_b),
        None => println!("La variable no contiene un valor"),
    }

    number_b = Some(5);

    match number_b {
        Some(number_b) => println!("La variable contiene un valor: {}", number_b),
        None => println!("La variable no contiene un valor"),
    }

    // let en la definición
    let greater_than = if number_a > 5 { true } else { false};
    println!("El número es mayor que 5: {}", greater_than);


    // Bucles
    // Infinito, usar condicion dentro y break para salir
    loop {
        println!("again!");
        break;
    }

    // Retornar valores de los bucles
    let mut counter = 0;
    let limit = 10;
    let result = loop {
        counter += 1;
        if counter == limit {
            break counter * 2;
        }
    };
    println!("El doble del valor del contador es: {}", result);

    let mut matriz = [[0; 3]; 3]; // Matriz de 3x3

    // Llenar la matriz con valores aleatorios entre 1 y 10
    for i in 0..3 {
        for j in 0..3 {
            matriz[i][j] = rand::rng().random_range(1..=10);
        }
    }

    // Bucle anidado para recorrer la matriz
    for i in 0..3 {
        for j in 0..3 {
            println!("Valor en matriz[{}][{}]: {}", i, j, matriz[i][j]);
        }
    }

    let mut while_counter = 3;

    while while_counter != 0 {
        println!("{while_counter}!");

        while_counter -= 1;
    }


    for number in (1..10).rev() {
        println!("Reverse for counter: {number}!");
    }


}
