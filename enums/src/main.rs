
enum IPAddrType {
    V4,
    V6,
}

struct IPAddr {
    kind: IPAddrType,
    address: String,
}
// Esto es porque no existe el tipo null en Rust
// enum Option<T> {
//     Some(T),
//     None,
// }
// Para el apartado de matching con bindeos da problema a la hora de ejecutarlo
// por tener definida la enumeración ya que se está concretando el tipo, si
// no se define, admite cualquier tipo

enum Coin{
    Penny,
    Niquel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
}

fn main() {

    let localhost= IPAddr {
        kind: IPAddrType::V4,
        address: String::from("127.0.0.1"),
    };

    // No entiendo estos ejemplos de enum, se pueden imprimir??
    // println!("El localhost es: {}", localhost.kind);
    // println!("El localhost es: {}", localhost.address);

    // Estamos implementando un numero y un caracter
    // let number = Some(5)
    // let character = Some('e')
    // Estamos implementando un tipo de dato entero Y VACÍO
    // let absent_number = None;

    let x: i8 = 5;

    // Se debe implementar un trait para sumar datos de tipo Option, de base da error
    // let x_option: Option<i8> = Some(5);
    // let y: Option<i8> = Some(5);
    // let sum = x_option + y;

    // Para poder sumar dos variables que tiene el tipo de dato Option debemos hacer un casting
    // de Option<T> a T

    let number: Option<i32> = Option::Some(10);
    let nothing: Option<i32> = Option::None;

    match number {
        Option::Some(valor) => println!("El número es: {}", valor),
        Option::None => println!("No hay número"),
    }

    match nothing {
        Option::Some(valor) => println!("El número es: {}", valor),
        Option::None => println!("No hay número"),
    }

    // The match Control Flow Construct


    // Match Control Flow Construct
    // Similar al switch case de otros lenguajes
    let amount_penny: Coin = Coin::Penny;
    let amount_niquel: Coin = Coin::Niquel;
    let amount_dime: Coin = Coin::Dime;
    // Patterns to bind values
    // Se pueden añadir condiciones para realizar un filtrado mas específico
    let amount_quarter: Coin = Coin::Quarter(UsState::Alabama);
    let amount_in_cents: u8 = value_in_cents(amount_penny);
    let amount_in_niquel: u8 = value_in_cents(amount_niquel);
    let amount_in_dime: u8 = value_in_cents(amount_dime);
    let amount_in_quarter: u8 = value_in_cents(amount_quarter);
    println!("Cents amount {}", amount_in_cents);
    println!("Cents amount {}", amount_in_niquel);
    println!("Cents amount {}", amount_in_dime);
    println!("Cents amount {}", amount_in_quarter);

    // Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("El valor de six es: {:?}", six);
    // println!("El valor de none es: {:?}", none);

    // CatchAll patterns
    // Esto es basicamente para controlar el resto de casos no definidos
    let dice_roll = 7;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        num_spaces => move_player(dice_roll),
        // Se puede utilizar el comoodin para añadir los casos que no se contemplan
    }

    let dice_roll = 10;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
        // OJO!!! Para los casos no contemplados ponerlo siempre al final porque
        // si se pone al principio siempre ejecutaría el mismo caso
    }




}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Niquel => 5,
        Coin::Dime => 25,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            50
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // Si no se contemplan todos los casos no compila, comentar cualquiera
        // de los dos da error, OJO SIEMPRE QUE SE UTILIZA OPTION<T>!!!
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("Fancy hat added");
}
fn remove_fancy_hat() {
    println!("Fancy hat removed");
}
fn move_player(dice_roll: u8) {
    println!("Player moved {} spaces", dice_roll);
}
fn reroll() {
    println!("Player rerolled");
}