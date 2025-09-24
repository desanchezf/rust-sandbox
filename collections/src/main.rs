enum CsvRow {
    Int(i32),
    Float(f32),
    String(String),
}



fn main() {

    // Declaramos un vector de enteros vacío
    let mut v: Vec<i32> = Vec::new();
    // Siempre van al final del vector
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector original: {:?}", v);
    // Elimina el último elemento introducido
    v.pop();
    println!("Vector después de pop: {:?}", v);


    // El vector puede almacenar diferentes tipos de datos
    let v_str: Vec<&str> = vec!["Hola", "Mundo"];
    // ó
    let v_str_2: Vec<String> = vec!["Hola".to_string(), "Mundo".to_string()];

    println!("Vector de strings: {:?}", v_str);
    println!("Vector de strings 2: {:?}", v_str_2);

    // Para leer los elementos del vector
    let vector_elements = vec![4, 5, 6, 7, 8, 9, 10];

    // Acceder al valor del vector a través de la referencia
    let third_element = &vector_elements[2];
    println!("Tercer elemento del vector: {}", third_element);

    // Tambien se puede acceder a los elementos del vector mediante el método get
    let second_element: Option<&i32> = vector_elements.get(1);

    // Si existe el elemento, se imprime, si no, se imprime un mensaje de error
    if let Some(second_element) = second_element {
        println!("El segundo elemento del vector es: {}", second_element);
    } else {
        println!("El segundo elemento del vector no existe");
    }

    // Ejemplo de acceso a una posición no valida del vector
    if let Some(invalid_element) = vector_elements.get(100) {
        println!("El elemento en la posición 100 del vector es: {}", invalid_element);
    } else {
        println!("El elemento en la posición 100 del vector no existe");
    }

    // Si intentamos acceder a una posición sin el Some el programa panikea dando error
    // let third_element = &vector_elements[200];
    // thread 'main' panicked at src/main.rs:27:41:
    // index out of bounds: the len is 7 but the index is 200
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    let mut vector_reference_error = vec![1, 2, 3, 4, 5, 6, 7];

    // let third_element = &vector_reference_error[2];  // Prestamo inmutable
    // Para solventar el error se tiene que hacer lo siguiente:
    let third_element = vector_reference_error[2];  // Copia del valor
    vector_reference_error.push(8);  // Prestamo mutable
    println!("El tercer elemento del vector es: {}", third_element);

    // - Se puede tener multiples referencua inmutables
    // - o una referencia mutable
    // - Pero nunca ambas al mismo tiempo

    //     error[E0502]: cannot borrow `vector_reference_error` as mutable because it is also borrowed as immutable
    //   --> src/main.rs:56:5
    //    |
    // 55 |     let third_element = &vector_reference_error[2];
    //    |                          ---------------------- immutable borrow occurs here
    // 56 |     vector_reference_error.push(8);
    //    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
    // 57 |     println!("El tercer elemento del vector es: {}", third_element);
    //    |                                                      ------------- immutable borrow later used here


    // Para iterar por los elementos del vector
    let vector_iter  = vec![1, 2, 3, 4, 5, 6, 7];
    for element in vector_iter {
        println!("Elemento del vector: {}", element);
    }

    // O si queremos iterar por referencias mutables
    let mut vector_mut_iter = vec![8,9,10];
    for i in &mut vector_mut_iter {
        *i += 20;
        println!("Elemento del vector (por referencia mutable): {}", i);
        // ó println!("Elemento del vector (por referencia mutable): {}", *i);
    }

    // Da warning por no usarlo
    let vector_row = vec![
        CsvRow::Int(1),
        CsvRow::Float(1.0),
        CsvRow::String(String::from("Hola")),
    ];
    // Hay que hacer un trait para imprimirlo
    // println!("Vector de filas: {}", vector_row[0]);
    // println!("Vector de filas: {}", vector_row[1]);
    // println!("Vector de filas: {}", vector_row[2]);


    // Strings
    // Si queremos concatenar tipo de dato String, podemos hacerlo de la siguiente manera
    let mut hello = "Hola ".to_string();
    println!("{}", hello);
    hello.push_str("Mundo!");
    println!("push_str: {}", hello);


    // Tambien podemos hacerlo con el operador +
    let string_1 = "Hola ".to_string();
    let string_2 = "Mundo!".to_string();
    let string_3 = string_1 + &string_2;  // -> Se usa la referencia & porque así es la signatura de la función +
    // println!("Concatenacion con +: {}", string_1);  // ¡¡Ha perdido el ownership!!
    println!("Concatenacion con +: {}", string_2);
    println!("Concatenacion con +: {}", string_3);

    // Tambien podemos añadir un caracter solo al final de la cadena (no deja de ser un vector de caracteres)
    hello.push('!');  //  Importante, usar comillas simples!!
    println!("push de caracter: {}", hello);

    let concat_string_1 = String::from("Hola ");
    let concat_string_2 = String::from("Mundo");
    let concat_string_3 = String::from("!");

    let concat_string = concat_string_1 + &concat_string_2 + &concat_string_3;
    // Impoortante si se van a concatenar cadenas, se debe poner & en la segunda y siguientes
    println!("Concatenacion con +: {}", concat_string);

    let concat_string_4 = String::from("Hola ");
    let concat_string_5 = String::from("Mundo");
    let concat_string_6 = String::from("!");

    let concat_string = format!("{concat_string_4}{concat_string_5}{concat_string_6}");
    println!("Concatenacion con format: {}", concat_string);



}
