use std::cmp::Ordering;
use std::io;  // std -> libreria estandar de rust, io -> modulo de entrada y salida
use rand::Rng;


// No oficial

// fn main() {

//     println!("Adivina el número!");
//     println!("Introduce un número.");

//     let mut guess = String::new();
//     let secret_number = rand::rng().random_range(1..=10);  // En caso de queramos excluir el 10, se pone 1..10

//     // Por legibilidad se pone de esta manera
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line 🤦🏻");
//     // Y no así
//     // io::stdin().read_line(&mut guess).expect("Failed to read line");

//     // Esto es un casting que se hace de string a u32
//     let guess: u32 = guess.trim().parse().expect("Por favor, introduce un número válido");


//     // if guess.trim() == secret_number.to_string() {
//     //     println!("Correcto 🎉");
//     // } else {
//     //     println!("Incorrecto 🤦🏻");
//     // }

//     match guess.cmp(&secret_number) {
//         Ordering::Less => println!("Muy bajo"),
//         Ordering::Greater => println!("Muy alto"),
//         Ordering::Equal => println!("Correcto 🎉"),
//     }

//     println!("El número introducido es {} y el generado es {}", guess, secret_number);
// }


// Del tutorial

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Caracter invalido");
                continue
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}