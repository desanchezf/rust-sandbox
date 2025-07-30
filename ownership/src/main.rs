// Ownership Rules
// -> Each value in Rust has an owner. Cada valor en Rust tiene un propietario.
// -> There can only be one owner at a time. Solo puede haber un propietario a la vez.
// -> When the owner goes out of scope, the value will be dropped. Cuando el propietario sale del scope, el valor se eliminará.


fn main() {

    let s = String::from("Fuera del ambito");
    println!("{s}");


    // Si salimos del scope, el valor se eliminará
    {
        let s = String::from("Dentro del ambito");
        println!("{s}");
        let s_inside = String::from("Esta variable no existe fuera de aqui");
        println!("{s_inside}");
    }

    println!("{s}");
    // println!("{s_inside}"); // Si imprimimos una variable definida dentro de un scope, fuera
    // error[E0425]: cannot find value `s_inside` in this scope
    //     --> src/main.rs:21:16
    //     |
    //     21 |     println!("{s_inside}");
    //     |                ^^^^^^^^ not found in this scope

    //     For more information about this error, try `rustc --explain E0425`.
    //     error: could not compile `ownership` (bin "ownership") due to 1 previous error


    // Uso de cadenas
    let s1 = String::from("Hola");
    let s2 = s1;
    println!("{s2}");
    // println!("{s1}"); // s1 ya no existe, porque se movio a la pila

    // --> src/main.rs:36:15
    //     |
    // 34 |     let s1 = String::from("Hola");
    //     |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    // 35 |     let s2 = s1;
    //     |              -- value moved here
    // 36 |     println!("{s1}"); // s1 ya no existe, porque se movio a la pila
    //     |               ^^^^ value borrowed here after move
    //     |

    // Si queremos hacer una copia del valor de la variable
    let s3 = String::from("Hola S3");
    let s4 = s3.clone();
    // Ahora si podemos imprimir ambas variables
    println!("Valor de S3: {s3}");
    println!("Valor de S4: {s4}");

    // Ahora con datos de tipo entero
    let x = 5;
    let y = x;
    println!("Valor de X: {x} y el valor de Y: {y}");
    // Esto es por que los enteros tiene un tamaño fijo, por lo tanto se almacenan en el stack y no en el heap

    // Ownership
    let string_prueba = String::from("hello");  // s comes into scope

    takes_ownership(string_prueba);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x_numero = 5;                      // x comes into scope

    makes_copy(x_numero);                  // because i32 implements the Copy trait,
                                    // x does NOT move into the function,
    println!("{}", x_numero);              // so it's okay to use x afterward


    // println!("Valor de S: {string_prueba}");    // ❌ Fallo en tiempo de compilacion
                                                // Eso es porque la propiedad ha pasado a la función y al pasar
                                                // la propieda a la función Rust hace un drop de la memoria de la variable

    let string_1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let string_2 = String::from("hello");    // s2 comes into scope

    let string_3 = takes_and_gives_back(string_2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    println!("Valor de S1: {string_1}");
    // println!("Valor de S2: {string_2}"); // ❌ Fallo en tiempo de compilacion, ya que pierde la propiedad
    println!("Valor de S3: {string_3}");


    // Como en python se pueden devolver multiple valores en el mismo return
    let (string_4, length) = calculate_length(string_3);
    println!("Valor de S4: {string_4} y la longitud es: {length}");


    let mut string_prueba = String::from("Hola, esto es una cadena de prueba");
    let reference_1 = &string_prueba;
    let reference_2 = &string_prueba;

    println!("Valor de la referencia 1: {reference_1}");
    println!("Valor de la referencia 1: {reference_2}");

    let reference_3 = &mut string_prueba;
    println!("Valor de la referencia 1, otra vez: {reference_3}");


    // let referencia_3 = &mut string_prueba;

    // let referencia_cadena = dangle();
    // println!("Valor de la referencia de la cadena: {referencia_cadena}");


    // Slices
    let string_len = first_word(&String::from("Hola, mundo!!"));
    println!("La longitud de la cadena es: {string_len}");

    // Como en Python las cadenas se pueden manejar mediante slices
    let string_slice = &String::from("Hola, mundo!!");
    println!("La cadena es: {string_slice}");

    // Para sacar el mundo!! -> debería ser algo así
    let string_hola = &string_slice[0..5];
    // [..5] es lo mismo que 0..5
    // [5..] es lo mismo que 5..13
    // [..] es lo mismo que 0..13
    // [..=13] es lo mismo que 0..=13

    println!("La cadena es: {string_hola}");
    // En caso de acceder a un indice que no existe, se producira un error de tiempo de ejecucion -> es decir, peta

    let string_world = &string_slice[6..13];
    println!("La cadena es: {string_world}");

}

// FUNCIONES /////////

// fn dangle() -> &String {
//     let s = String::from("Hola, esto es una cadena de prueba");
//     &s
//     // Fallo porque cuando sale del scope, la variable s se elimina y por lo tanto la referencia es invalida
// }

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Para sacar el world!! -> debería ser algo así
// fn second_word(s: &String) -> (usize, usize) {

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


fn gives_ownership() -> String {       // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

// Devuelve la cadena y su longitud
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}