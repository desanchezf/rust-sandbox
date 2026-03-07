fn main() {

    // Longitud de una cadena
    let s1 = String::from("Hello World!");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Edición de cadenas mediante referencua
    let mut s2 = String::from("Hello");
    println!("La cadena sin editart es: {}", s2);
    // edit_value_reference(&s2); No funca
    edit_value_reference(&mut s2);
    println!("La cadena editada es: {}", s2);

    // Para cambiar el contenido de cadena sin cambio de propiedad es necesario
    //  declarar la variable como mutable, y luego utilizar la referencia mutable
    // let mut s2 = String::from("Hello");
    //
    // edit_value_reference(&mut s2);
    //
    // fn edit_value_reference(s: &mut String) {
    //     s.push_str(", world!");
    // }

    // Las referencias mutables tienen una gran restricción: si tienes una referencia
    // mutable a un valor, no puedes tener otras referencias a ese valor.

    // let mut s3 = String::from("Hello");
    // let r1 = &mut s3;
    // let r2 = &mut s3; // Dara error en este punto

    // println!("{}", r1);
    // println!("{}", r2);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("{}", r1);
    }

    let r2 = &mut s;
    println!("{}", r2);

    // Tira de la misma referencia -> r1 no genera problema con r2 porque son
    // scopes diferentes

    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{} {} {}", r1, r2, r3);
    // La referencia es iniciada en el momento en el que se define y finzaliza en el
    // momento que es utilizada, por lo tanto en el println!("{} {} {}", r1, r2, r3);
    // conviven r1, r2, y r3 -> Origina error

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);  // En este punto r1 y r2 se han consumido y por lo tanto se
                                    // puede generar una nueva referencia (r3) a s (en este caso mutable)
                                    // este tipo de referenciado no genera ningún error.
    let r3 = &mut s; // no problem
    println!("{}", r3);


}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn edit_value_reference(s: &String) {
//     s.push_str(", world!");
// }

fn edit_value_reference(s: &mut String) {
    s.push_str(", world!");
}
