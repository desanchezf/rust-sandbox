fn main() {

    // En caso de usar variables no mutables tenemos que utilizar el let en las asignaciones (shadowning)
    let var_a: u32 = 10;
    println!("El valor de la variable es: {}", var_a);

    let var_a: u32 = 20;
    println!("El valor de la variable es: {}", var_a);

    let var_a: u32 = var_a + 10;
    println!("El valor de la variable es: {}", var_a);


    // Si utilizamos variables mutables no es necesario el uso de let en las asignaciones
    let mut var_b: u32 = 50;
    println!("El valor de la variable es: {}", var_b);

    var_b = 60;  // Si usamos let var_b = 60; salta warning -> No es necesario
    println!("El valor de la variable es: {}", var_b);

    var_b = var_b + 10;
    println!("El valor de la variable es: {}", var_b);

    let string = "Cadena de prueba";
    println!("El valor de la variable es: {}", string);

    let mut spaces = "   ";
    let spaces = spaces.len();
    println!("El valor de la variable es: {}", spaces);
}
