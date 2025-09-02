
// ¡¡Se puede definir tanto dentro como fuera del main
struct User {
    name: String,
    age: i32,
    email: String,
}
// Structs como tuplas
struct RgbColor(i32, i32, i32);

// Structs units-like
// struct Unit;  // Útil para la implementación de traits pero no tienes definido ningún campo

// Ejemplo práctico
struct Rectangle {
    width: u32,
    height: u32,
}

// #[derive(Debug)]
// struct SquareDBG {
//     width: u32,
//     height: u32,
// }

// Para imprimir el struct
#[derive(Debug)]
struct RectanglePrintable {
    width: u32,
    height: u32,
}

struct SquareWithMethods {
    width: u32,
    height: u32,
}

impl SquareWithMethods {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct ParametrizedSquareWithMethods {
    width: u32,
    height: u32,
}

impl ParametrizedSquareWithMethods {
    // Constructor de un struct -> No es necesario añadir el parámetro self y retorna una instancia de struct
    fn area(&self, factor: u32) -> u32 {
        (self.width * factor) * (self.height * factor)
    }
    fn can_hold(&self, square: &ParametrizedSquareWithMethods) -> bool {
        self.width > square.width && self.height > square.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}



fn main() {
    // ############
    // ¡¡¡IMPORTANTE, STRUCTS DEFINIDAS FUERA DEL MAIN!!!
    // ############

    // Inicializacion de un struct
    let user = User {
        name: String::from("Juan"),
        age: 20,
        email: String::from("juan@example.com"),
    };

    // Acceso a los campos de un struct
    println!("El nombre del usuario es: {}", user.name);
    println!("La edad del usuario es: {}", user.age);
    println!("El email del usuario es: {}", user.email);


    // Inicializacion de un struct mediante función
    let user_fn = build_user(25);
    println!("El nombre del usuario es: {}", user_fn.name);
    println!("La edad del usuario es: {}", user_fn.age);
    println!("El email del usuario es: {}", user_fn.email);

    // Se puede crear instancias a partir de otras editando los valores
    let user_copy = User {
        name: String::from("Persona con la misma edad de Paco"),
        age: user_fn.age,
        email: String::from("copiapaco@example.com"),
    };
    println!("El nombre del usuario es: {}", user_copy.name);
    println!("La edad del usuario es: {}", user_copy.age);
    println!("El email del usuario es: {}", user_copy.email);


    // Se pueden crear instancias a partir de otras ignorando algunos valores (A la hora de copiar)
    // por lo tanto manteniene los valores que tiene user_copy
    let user_copy_2 = User {
        name: String::from("Copia vaga de user_copy"),
        ..user_copy
    };
    println!("El nombre del usuario es: {}", user_copy_2.name);
    println!("La edad del usuario es: {}", user_copy_2.age);
    println!("El email del usuario es: {}", user_copy_2.email);

    // Podemos definir structs mediante tuplas -> Es útil
    let color = RgbColor(1,1,1);
    println!("El color es: {}", color.0);
    println!("El color es: {}", color.1);
    println!("El color es: {}", color.2);

    // Podemos copiar valores desde otra instancia
    let RgbColor(r, g, b) = color;
    println!("El color es: {}", r);
    println!("El color es: {}", g);
    println!("El color es: {}", b);


    // Structs units-like
    // let unit = Unit;
    // El ejemplo es demasiado complejo por ahora

    // Area de un rectangulo
    let rectangle = Rectangle {
        width: 10,
        height: 10,
    };
    // Incorrecto dejaría de existir rectangle y no se podría utilizar en la siguiente función
    // println!("El area del rectangulo es: {}", area(rectangle));
    // -> correcto, se pasa una referencia
    println!("El area del rectangulo es: {}", area(&rectangle));
    println!("El area del rectangulo es: {}", area_args(rectangle.width, rectangle.height));
    println!("El area del rectangulo es: {}", area_refactored((rectangle.width, rectangle.height)));
    println!("El area del rectangulo es: {}", area_refactored_with_context(&rectangle));

    let rectangle_printable = RectanglePrintable {
        width: 22,
        height: 22,
    };
    // Estilo inline
    println!("El rectangulo es: {rectangle_printable:?}");
    // Estilo json (?)
    println!("El rectangulo es: \n{rectangle_printable:#?}");

    let dimensions: u32 = get_dimension(&rectangle_printable);
    println!("El area del rectangulo es: {dimensions}");

    // let square = SquareDBG {
    //     width: dbg!(123),
    //     height: 10,
    // };
    // dbg!(&square);

    let square_with_methods = SquareWithMethods {
        width: 11,
        height: 13,
    };
    println!("El area del cuadrado es: {}", square_with_methods.area());

    // Metodos parametrizados
    let parametrized_square_with_methods = ParametrizedSquareWithMethods {
        width: 100,
        height: 100,
    };

    let factor = 5;
    println!("El area del cuadrado con factor {} es: {}",factor, parametrized_square_with_methods.area(factor));

    let little_square = ParametrizedSquareWithMethods {
        width: 10,
        height: 10,
    };
    let large_square = ParametrizedSquareWithMethods {
        width: 1000,
        height: 1000,
    };

    println!("El cuadrado grande puede contener al cuadrado pequeño: {}", parametrized_square_with_methods.can_hold(&little_square));
    println!("El cuadrado grande puede contener al cuadrado pequeño: {}", parametrized_square_with_methods.can_hold(&large_square));

    // "Constructor" de un struct
    let square_constructor = ParametrizedSquareWithMethods::square(10);
    println!("El area del cuadrado constructor es: {}x{}", square_constructor.height, square_constructor.width);

}


// Definición de un struct mediante función
fn build_user(age: i32) -> User {
    User {
        name: String::from("Paco"),
        age: age,
        email: String::from("paco@example.com"),
    }

}

// Usando el struct como argumento
fn area(rectangle: &Rectangle) -> u32{
    return rectangle.width * rectangle.height;
}

// Usando la base y la altura como argumentos
fn area_args(width: u32, height: u32) -> u32{
    return width * height;
}

// Esta es la forma mas eficiente de hacerlo (pero no tiene tanto significado)
fn area_refactored(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

// Esta es la forma mas eficiente de hacerlo (y que mas significado aporta)
fn area_refactored_with_context(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

fn get_dimension(rectangle: &RectanglePrintable) -> u32 {
    return rectangle.width * rectangle.height;
}