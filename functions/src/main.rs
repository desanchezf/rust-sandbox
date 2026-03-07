fn main() {
    let a = 5;
    let b = 6;

    println!("El valor de a es: {}", a);
    println!("El valor de b es: {}", b);

    let resultado_suma = suma(a, b);
    println!("El resultado de la suma es: {}", resultado_suma);

    let resultado_resta = resta(a, b);
    println!("El resultado de la resta es: {}", resultado_resta);

    let resultado_multiplicacion = multiplicacion(a, b);
    println!("El resultado de la multiplicacion es: {}", resultado_multiplicacion);

    let resultado_division = division(a, b);
    println!("El resultado de la division es: {}", resultado_division);

}


fn suma (a: i32, b: i32) -> i32 {
    a + b
}

fn resta(a: i32, b: i32) -> i32 {
    a - b
}

fn multiplicacion(a: i32, b: i32) -> i32 {
    a * b
}

fn division(a: i32, b: i32) -> i32 {
    a / b
}
