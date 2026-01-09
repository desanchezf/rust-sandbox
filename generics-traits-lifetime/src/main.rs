struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

fn main() {

    // Pasamos de esto //////////////////////////////////////////////////
    // let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {largest}");

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {largest}");

    // A esto ////////////////////////////////////////////////////////////////

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    // Ahora se puede utilizar la misma función tanto para enteros ...
    let result = largest_generic(&char_list);
    println!("(Using generic) The largest char is {result}");

    // ...como para caracteres
    let result = largest_generic(&number_list);
    println!("(Using generic) The largest number is {result}");


    // Definimos una variable entera usando el struct
    let int_point = Point { x: 1, y: 2 };

    // Definimos una variable flotante usando el struct
    let float_point = Point { x: 1.0, y: 2.0 };

    // ¡¡ Si queremos combinar tipos diferentes de datos, debemos de utilizar <T, U>
    let mixed_point = MixedPoint { x: 1, y: 2.0 };

    println!("Int point: {0}, {1}", int_point.x, int_point.y);
    println!("Float point: {0}, {1}", float_point.x, float_point.y);
    println!("Mixed point: {0}, {1}", mixed_point.x, mixed_point.y);


    let x_method = Point{x: 5, y: 10};
    println!("x_method.x() = {}", x_method.x());

    let x_float_method = Point{x: 5.12, y: 10.12};
    println!("x_float_method.x() = {}", x_float_method.x());

    let x_f32_method = Point{x: 5.12, y: 10.12};
    println!("x_f32_method.distance_from_origin() = {}", x_f32_method.distance_from_origin());

}

// Funcion concretas para enteros
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Funcion concretas para caracteres
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Funcion generica para encontrar el mayor valor
// T es una convención para determinar el tipo de dato generico
// y recibe una lista de valores de tipo T
// y retorna el mayor valor de la lista de tipo T
// Añadir el trait PartialOrd para que el tipo T pueda ser comparado
fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}