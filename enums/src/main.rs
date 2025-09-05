
enum IPAddrType {
    V4,
    V6,
}

struct IPAddr {
    kind: IPAddrType,
    address: String,
}
// Esto es porque no existe el tipo null en Rust
enum Option<T> {
    Some(T),
    None,
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

    // The match Control Flow Construct


}
