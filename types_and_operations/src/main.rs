use std::io;


fn main() {
    // Operations
    // addition
    let sum = 5 + 10;
    println!("Sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference: {}", difference);
    // multiplication
    let product = 4 * 30;

    println!("Product: {}", product);
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let truncated_float = -5.0 / 3.0; // Results in -1.6666666666666667

    println!("Quotient: {}", quotient);
    println!("Truncated: {}", truncated);
    println!("Truncated float: {}", truncated_float);
    // remainder
    let remainder = 43 % 5;
    println!("Remainder: {}", remainder);


    let true_value = true;
    let false_value: bool = false;

    println!("True value: {}", true_value);
    println!("False value: {}", false_value);

    // Types
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    //  let z: char = 'ℤZZZZ'; // No es puede definir -> esto ya es una cadena

    println!("Character: {}", c);
    println!("Character: {}", z);
    println!("Character: {}", heart_eyed_cat);

    // Tuples
    let tup_uniform = (500, 10, 1);
    let tup_uniform_typed: (u32, u32, u32) = (500, 10, 1);
    let tup_irregular = (500, 6.4, "Prueba");
    let tup_irregular_typed: (u32, char, String) = (500, 'z', String::from("Prueba"));

    println!("Tupla uniforme: {:?}", tup_uniform);
    println!("Tupla uniforme tipada: {:?}", tup_uniform_typed);
    println!("Tupla irregular: {:?}", tup_irregular);
    println!("Tupla irregular tipada: {:?}", tup_irregular_typed);


    // Arrays
    let a = [1, 2, 3, 4, 5];
    let a_typed: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let months_typed: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let same_value_array = [3; 5];  // Inicializando con el mismo valor

    println!("Array: {:?}", a);
    println!("Array tipado: {:?}", a_typed);
    println!("Months: {:?}", months);
    println!("Months tipados: {:?}", months_typed);
    println!("Array con el mismo valor: {:?}", same_value_array);
    println!("Primer mes del año: {}", months[0]);


    let mut index: String = String::new();
    println!("Que numero de mes quieres saber?:");

    // Primero capturamos el valor en modo cadena
    io::stdin().read_line(&mut index).expect("Fallo al leer el indice");

    // Hacer un casting a entero
    let index: usize = index.trim().parse().expect("El valor de indice no es correcto");

    // Imprimimos el mes correspondiente
    if index >= 1 && index <= 12 {
        // Imprimimos mes[N]
        println!("El mes es: {}", months[index-1]);
    } else {
        println!("El indice no es correcto");
    }

}