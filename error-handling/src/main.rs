use std::fs::File;
use std::io::ErrorKind;

// No es necesario redefinir Result, ya está en el prelude de Rust
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }


fn main() {
    // panic!("Crash and burn!");

    // thread 'main' panicked at src\main.rs:2:5:
    // Crash and burn!
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // error: process didn't exit successfully: `target\debug\error-handling.exe` (exit code: 101)

    //let v = vec![1, 2, 3];
    //v[99];

    // thread 'main' panicked at src\main.rs:10:6:
    // index out of bounds: the len is 3 but the index is 99
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // error: process didn't exit successfully: `target\debug\error-handling.exe` (exit code: 101)

    // Esto sirve para ver el conjunto de funciones que se han llamado hasta ese punto
    // RUST_BACKTRACE=1

    // -> Result<File, std::io::Error>
    // let greeting_file = File::open("hello.txt");
    // let greeting_file_result = match greeting_file {
    //     Ok(file) => file,
    //     Err(e) => panic!("Error al abrir el archivo: {}", e),
    // }; // Da error por que no existe el archivo

    // thread 'main' panicked at src\main.rs:32:19:
    // Error al abrir el archivo: El sistema no puede encontrar el archivo especificado. (os error 2)
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // error: process didn't exit successfully: `target\debug\error-handling.exe` (exit code: 101)

    let hello_file = File::open("src/hello_world.txt");
    let hello_file_result = match hello_file {
        Ok(file) => {
            println!("Archivo abierto correctamente");
            file  // Devolver el archivo
        },
        Err(e) => panic!("Error al abrir el archivo: {}", e),
    };
    // println!("El valor de hello_file_result es: {:?}", hello_file_result); --> Irrelevante retorna el APTH

    let greeting_file_result = File::open("src/hello-error.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello-error.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // Esto se puede hacer de la siguiente manera:

    let greeting_file = File::open("src/hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("src/hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

}
