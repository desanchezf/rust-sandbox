## 0. Glosario

`rustup`: gestor de versiones de Rust y herramientas asociadas, similar a NVM
`cargo`: gestor de paquetes de Rust, similar a pip, se encarga tambien de:
	👉 Compilar tu código: `cargo build`
	👉 Ejecutar tu aplicación:  `cargo run`
	👉 Ejecutar pruebas unitarias y de integración: `cargo test`
	👉 Manejar dependencias: Usa el archivo `Cargo.toml` para declarar y gestionar las librerías (crates) que tu proyecto necesita.
	👉 `Cargo.lock`: contiene la información exacta sobre las dependencias. Está gestionado por `Cargo` y no debe ser manualmente editado.
	👉 Publicar crates en [crates.io](https://crates.io)  `cargo publish`
	👉 Actualizar dependencias  👉 `cargo update`
`crate`:Un crate es la unidad minima de distribución de paquetes, la diferencia principal es que un crate puede comprender varios ficheros. Un fichero generado mediante `rustc` solamente comprende un solo fichero. Un árbol de módulos que tiene como resultado una librería y un ejecutable
`rustc`: compilador de Rust, se llama de la siguiente manera `rustc file.rs -o output` en caso de que no se utilice la flag `-o`se utiliza el nombre del fichero `file` como nombre del fichero de salida.
`trait`: es como un **contrato** o **interfaz** que define **comportamientos (métodos)** que un tipo debe implementar si quiere "cumplir" con ese `trait`
`package`: una funcionalidad de `Cargo` que permite construir, probar y compartir `crates`.
`modules and use`: te permite controlar la organización, los `scopes` y la privacidad de los directorios
`paths`: la manera de nombrar (o llamar) un elemento, como un `struct`, `función` o `módulo`.

## 1. Cargo
Para crear un nuevo proyecto con **Cargo**

```bash
cargo new nombew_proyecto
```

Y genera un directorio con la siguiente estructura:

```bash
tree .
.
├── Cargo.toml
└── src
    └── main.rs

2 directories, 2 files
```

`Cargo.toml`: es el **archivo de configuración principal** de un proyecto Rust que usa Cargo.
Define **metadatos**, **dependencias**, **versiones**, **scripts de compilación**, y mucho más. Es como el `package.json` de Node.js o el `pyproject.toml` de Python.
👉 Se encuentra siempre en la raíz del proyecto
👉 Escrito en formato TOML ("Tom's Obvious, Minimal Language")

El fichero `Cargo.toml` posee la siguiente estructura:

```toml
[package]
name = "hello-cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

Donde:
- `[package]`:  es un encabezado de sección que indica que las siguientes sentencias están configurando un paquete. A medida que añadamos más información a este archivo, añadiremos otras secciones.
- `[dependencies]`:  es la sección donde se listan todas las dependencias del proyecto.

Cargo espera que tus archivos fuente estén dentro del directorio src. El directorio de proyecto de nivel superior es sólo para los archivos README, información de licencia, archivos de configuración, y cualquier otra cosa no relacionada con su código. Cargo ofrece una organización para los proyectos.

En caso de tener un proyecto que no ha sido creado por `Cargo`, se puede crear metiendo todos los ficheros en el directorio `src` y ejecutando el comando `cargo init`que generará el fichero `Cargo.toml`

> 💡 **Añadir dependencias de manera automatizada**
>
> Con este comando se pueden añadir dependencias de manera automatizada como si utilizacemos `pip install <nombre-paquete>`. Además edita el apartado de dependencias del fichero `Cargo.toml`.
>
> Para poder añadir dependencias mediante `cargo add` es necesario instalarlo mediante `cargo install cargo-edit`, una vez instalado dentro de la raíz del proyecto podremos hacerlo mediante `cargo add <nombre-paquete>`

Para hacer un build de un proyecto con cargo, ejecutar el comando `cargo build` y luego para ejecutarlo `cargo run`.
- A la hora de hacer un build se genera un `Cargo.lock` que mantiene la traza de la versiones de las dependencias utilizadas en el proyecto.

En caso de queramos comprobar que el programa podría compilar (sin generar ejecutable) se puede utilizar el comando `cargo check`.

Para generar un ejecutable de release se utiliza el comando `cargo build release`, generando la salida en el directorio `target/release` en lugar del `target/debug`. La diferencia entre ambos ejecutables es que se aplican optimizaciones que hacen que tu código Rust se ejecute más rápido, pero activarlas alarga el tiempo que tarda tu programa en compilarse.

## 2. Sintaxis
```rust
let mut guess = String::new();  // Crea un objeto de tipo String vacío
```

- `let`: se utiliza para la definición de variables
- `mut`: se utiliza para indicar que la variable puede mutar, ya que en `Rust` son inmutables por defecto.
-  `String::new`: es como si utilizamos en Python el nombre de la clase `obj = str()` retorna una nueva instancia de `String`.
- `::`: es como si fuera el operador `.` en Python `obj = str.new()`. Pero ojo solo para bibliotecas/modulos, para acceder a funciones se utiliza el operador punto `.`
- `&`: El & indica que este argumento es una referencia, lo que te da una forma de permitir que múltiples partes de tu código accedan a una pieza de datos sin necesidad de copiar esos datos en memoria varias veces. Las referencias son inmutables por defecto. Por lo tanto, necesitas escribir `&mut guess` en lugar de `&guess` para hacerla mutable.

- Variables de tipo `Result`:  se genera en llamada a ciertas funciones, por ejemplo en este caso:
```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

Se genera una variable de tipo Result de manera implicita en este punto:

```rust
io::stdin().read_line(&mut guess)
```

Es como si tuviéramos lo siguiente

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

Es igual a:

```rust
let resultado = io::stdin().read_line(&mut guess)
resulado.expect("Failed to read the line")
```

Con `expect` realizamos lo siguiente:
- En caso de que haya sido `Ok` ➔ Muestra el número recogido
- En caso de que haya sido `Err` ➔ Muestra el mensaje de error definido
Internamente la variable de tipo `Result` es un `enum` con los siguientes campos:

```Rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `derive`: es un **atributo** en Rust que le dice al compilador:  _"Genera automáticamente la implementación de ciertos traits para este tipo"_.
	- `Debug` ➔ imprimir con `{:?}`
	- `Clone` ➔ clonar el valor
	- `Copy` ➔ copiar bits (para tipos simples como enteros)
	- `PartialEq`, `Eq` ➔ comparar igualdad (`==`, `!=`)
	- `PartialOrd`, `Ord` ➔ ordenar (`<`, `>`, `sort`)
	- `Hash` ➔ usar como clave en `HashMap` o `HashSet`
	- `Default` ➔ crear un valor por defecto (`T::default()`)
- `dbg!`: se puede utilizar para imprimir el valor de una expresión **junto con el archivo y la línea** donde se ejecuta.
- `impl`: se utiliza para implementar métodos asociados a `struct`

## 3. Movidas
#### Ownership
El _Ownership_ es un conjunto de reglas que gobierna el como Rust administra la memoria
- Cada valor en Rust tiene un _owner_.
- Hay solamente un owner a la vez
- Cuando el _owner_ sale del `scope`, el valor será eliminado

#### Traits
Define **comportamientos comunes** que pueden ser implementados por diferentes tipos. *Son muy parecidos a **interfaces** en otros lenguajes como Java o TypeScript*. Un **trait** define un conjunto de métodos que un tipo debe implementar si quiere "adoptar" ese `trait`. Así, puedes escribir funciones o estructuras que trabajen con cualquier tipo que implemente un `trait`, sin importar cuál sea ese tipo en concreto.

El `trait` `Copy` indica que **un tipo puede ser duplicado con una simple copia de bits**. Es decir, cuando haces una asignación o pasas una variable a una función, Rust hace una **copia** en lugar de un **move**. Los tipos que implementan `Copy` no "pierden" su valor al ser asignados o pasados como argumento.

> _- Existe tambien el trait de ***Clone*** que permite la copia de una variable_
> - _Existe tambien el trait de ***Debug*** que que permite formatear estructuras, enums o cualquier tipo** en una representación legible para desarrolladores

Rust **implementa automáticamente `Copy`** para los siguientes tipos si **no contienen datos que requieran liberar memoria**, como:
- Todos los tipos enteros como por ejemplo `u32`.
- Los boolesanos (`bool`) que solamente tienen los valores `true` and `false`.
- Los tipos de punto flotante como `f64`.
- Los de tipo caracter, `char`.
- Las tuplas, solamente si contiene tipos que si tienen el `trait` `Copy`. Como por ejemplo, `(i32, i32)` implementa `Copy`, pero `(i32, String)` no.

El tipo de dato String, por ejemplo no tiene este `trait`.

When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.

#### References
Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.

Y no podemos tener la referencia inmutable guardada en una variable y la misma referencia mutable guardada en otra variable. Sin embargo si es posible usar multiples referencias inmutables ya que solamente leeran el valor de la variable.

#### Slices
Un **slice** es una **vista prestada** (una referencia) a una parte contigua de una colección en memoria. No es dueño de los datos, solo los **referencia con inicio y longitud**.
- **Referencia + longitud** (dos valores detrás de escena).
- **No dueños** de datos.
- **Inmutables por defecto** (`&[T]`), pero también existen **mutables** (`&mut [T]`).
- Evitan copiar memoria innecesariamente.
##### str vs String
>  _En Rust podemos crear cadenas de dos formas diferentes, podemos utilizar_ `str` _y podemos utilizar String:
>  - str:
> 	 - tipo de dato primitivo
> 	 - es inmutable, por lo tanto, no puede cambiar de valor
> 	 - el tipo se indica mediante referencia (`let cadena: &str = "Hola mundo"`);
> 	 - se almacena en el ***stack*** si es referencia
>  -String:
> 	 - es dinámica_
> 	 - _es mutable, por lo tanto, se puede redimensionar_
> 	 - _es propietaria de sus datos (tiene owner)_
> 	 - _se almacena en el ***heap***_
>
> _Se puede pasar de un tipo a otro de la siguiente manera:_


```rust
// De str a String
let cadena_str: &str = "Hola Mundo!";
conversion_str_string: String = cadena_str.to_string();
// De String a str
let cadena_string: String = String::from("Hola Mundo!");
conversion_string_str: &str = &cadena_string;
```

#### Struct
La finalidad de las estructuras es el de agrupar información relacionada, como si fuera un diccionario pero sin _clave-valor_. Se define de la siguiente manera

```rust
struct UserStruct {
	name: String,
	age: u8,
	...
}
```

Y para definir una variable de tipo `UserStruct`se haría de la siguiente manera:

```rust
let user_prueba = UserStruct {
	name: String = String::from("Jose Luis"),
	age: u8 = 20,
	...
}
```

> _Para imprimir los valores debemos añadir_ `#[derive(Debug)]` _antes de la definición del struct y a la hora de imprimir la variable se debe añadir_ `:?` _en el_ `println!`. De la siguiente manera:

```rust
// Definición del struct
#[derive(Debug)]
struct UserStruct {
	name: String,
	age: u8,
}
fn main {
	user_struct = UserStruct {
		name: String = String::from("Jose Luis"),
		age: u8 = 20,
	}
	println!("Usuario estructurado {user_struct:?}"); // Inline
	println!("Usuario estructurado {user_struct:#?}"); // JSON
}
```

Las salidas dependiendo del formateo usado son:
- `:?`
```bash
El rectangulo es: RectanglePrintable { width: 22, height: 22 }
```
- `:#?`
```bash
RectanglePrintable {
    width: 22,
    height: 22,
}
```

Nos quedamos en Here’s an example where we’re interested in the value that gets assigned to the `width` field, as well as the value of the whole struct in `rect1`:

```rust
struct SquareDBG {
	width: u8,
	height: u8,
}
fn main {
	let cuadrado = SquareDBG {
		width = 8,
		height = 8,
	}
	dbg!(&cuadrado)
}
```

#### Methods
Los métodos son similar a las funciones, los declaramos con la palabra reservada `fn`, pueden tener **parámetros** y pueden tener **valor de retorno**. Contienen código que es ejecutado cuando se llama desde cualquier lado.
A diferencia de las funciones, los métodos están definidos dentro del contexto de una estructura (_o un enum o un trait_). Al igual que las funciones dentro de una clase de `Python`, el primer argumento de una función es `self`, y es la instancia de la estructura que contiene al método.
Estos se implementan mediante la palabra reservada `impl`. Un ejemplo de definición de un método es:
```rust
struct Cuadrado {
	width: u32,
	height: u32,
}
impl Cuadrado {
	fn area (&self) -> u32 {
		self.width * self.height
	}
}

fn main {
	let cuadrado_methods = Cuadrado {
		height = 8,
		width = 8,
	};
	println!("El área del cuadrado es: {}", cuadrado_methods.area());
}
```

En el método utilizamos `&self` ya que no vamos a editar la instancia, en el caso de que queramos cambiar algunos de los parámetros, debemos utilizar `&mut self` para ello.

#### Associated functions
Las funciones asociadas poseen las siguientes características:
- Están asociadas a un `struct`
- No es necesario `self` como argumento
- Retornan una instancia del `struct` asociado
- No es necesario una instancia del `struct` para trabajar con los métodos asociados
- Se accede mediante el operador `::`(_Ver ejemplo abajo_)

```rust
struct Square {
	width: u32,
	height: u32,
}

impl Square {
	fn square(lado: u32) -> Self {
		Self {
			width: lado,
			height: lado,
		}
	}
}
fn main {
	let lado = 5;
	let square_constructor = Square::generate_square(lado);
}
```

> También se puede utilizar el nombre de la clase de la siguiente manera, pero queda mas limpio `Self`. El nombre de la clase solamente queda reservado para casos complejos donde se requiera explícitamente el nombre del `struct`.

```rust
```rust
struct Square {
	width: u32,
	height: u32,
}

impl Square {
	fn square (lado: u32) -> Self {
		Self {
			width: lado,
			height: lado,
		}
	}
	// Un `struct` puede contener múltiples métodos!!
	fn get_area (Self) -> u32 {
		return self.width * self.height
	}
	fn can_hold(&little_square: Square) -> bool {
		let anchos = little_square.width < self.width;
		let altos = little_square.height < self.height;
		return anchos && altos
	}
}
fn main {
	let lado = 5;
	let square_constructor = Square::square(lado);
	let lado = 2
	let little_square = Square::square(lado);
	let cabe = square_constructor.can_hold(&little_square);
}
```

Son funciones que se pueden utilizar a modo de constructores de "objetos" de tipo `struct`.
#### Enums and Pattern matching

Los tipos de datos enum, son secuencias de valores que puede tomar un determinado campo por ejemplo de un `struct`
```rust
enum IPAddrType {
	V4,
	V6,
}
struct IPAddr {
	address: String,
	type: IPAddrType,
}

fn main {
	let ip_home = {
		addres: String::from("127.0.0.1"),
		type: IPAddrType::V4,
	};
}
```

##### enum Option \<T\>
Rust por defecto no implementa el tipo de dato `null` pero, no obstante hay un _workaround_ que permite usarlo en la comparación de valores.

Cuando por ejemplo definimos una variable con el tipo de dato `Option<i8>` el **TIPO** de la variable es `Option<i8>`, que es un `enum` genérico que puede contener:
	- `Some(valor)` - Un valor de tipo `i8` (entero de 8 bits con signo)
	- `None` - Ningún valor (equivalente a `null` en otros lenguajes)

_Un ejemplo útil es cuando queremos obtener el primer dato de una lista, en el caso de que la lista contenga elementos, se retorna el valor, pero en el caso de que la lista esté vacía "se retornaría null" _

```rust
enum Option<T> {
	None,
	Some(T),
}
```

```rust
// Estamos implementando un numero y un caracter
let number = Some(5)
let character = Some('e')
// Estamos implementando un tipo de dato entero Y VACÍO
let absent_number: Option<i32> = None;
```

> _*Ojocuidao 1* si existe el tipo de dato `None`_
> _*Ojocuidao 2* Estamos implementando un entero a través del `enum`, por lo tanto, si queremos implementarlo no podríamos porque no se trata de un entero o un caracter, lanzando el siguiente error. Para poder imprimirlo, sería necesario implementar un `trait`que lo permita.

```bash
  --> src/main.rs:35:34
   |
35 |     println!("El numero es: {}", number);
   |                             --   ^^^^^^ `std::option::Option<{integer}>` cannot be formatted with the default formatter
   |                             |
   |                             required by this formatting parameter
   |
   = help: the trait `std::fmt::Display` is not implemented for `std::option::Option<{integer}>`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
```

En este caso daría error porque en la suma no coinciden los tipos `i8` + `Option<i8>`

```Rust
let x: i8 = 5;
let y: Option<i8> = Some(5); // Aqui estamos diciendo que el Some es de tipo i8
// Pero
let sum = x + y;
```

Dando el siguiente error:

```rust
error[E0277]: cannot add `Option<i8>` to `i8`
  --> src/main.rs:37:17
   |
37 |     let sum = x + y;
   |                 ^ no implementation for `i8 + Option<i8>`
   |
   = help: the trait `Add<Option<i8>>` is not implemented for `i8`
   = help: the following other types implement trait `Add<Rhs>`:
             `&i8` implements `Add<i8>`
             `&i8` implements `Add`
             `i8` implements `Add<&i8>`
             `i8` implements `Add`

```
##### Pattern matching
Permite comparar valores contra una serie de patrones y en base a ello ejecutar cierta funcionalidad del patrón con el que coincide.

Similar a los switch-case de otros lenguajes.
```rust
enum Coin{
    Penny,
    Niquel,
    Dime,
    Quarter,
    Dolar,
}

fn main {
	let amount: Coin = Coin::Quarter;
	let amount_in_cents = value_in_cents(amount);
	print("Cantidad {}", amount_in_cents)  // -> Esto imprime 50
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Niquel => 5,
        Coin::Dime => 25,
        Coin::Quarter => 50,
        // Obviamente se puede hacer lo siguiente
        Coin::Dollar => println!("Un dolarín"),
        // ó {} si queremos utilizar multiples lineas
        Coin::Dollar => {
	        println!("Un dolarín");
	        100
	    },
    }
}
```

En el caso de que no se contemplen todos los casos del match, `Rust`indicará que hay un error, por ejemplo, en este trozo de código

```bash
    let dice_roll = 10;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // _ => reroll(),  -> Comentamos esta linea
    }

```

Dará el siguiente error:
```rust
error[E0004]: non-exhaustive patterns: `i32::MIN..=2_i32`, `4_i32..=6_i32` and `8_i32..=i32::MAX` not covered
   --> src/main.rs:112:11
    |
112 |     match dice_roll {
    |           ^^^^^^^^^ patterns `i32::MIN..=2_i32`, `4_i32..=6_i32` and `8_i32..=i32::MAX` not covered
    |
    = note: the matched value is of type `i32`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
```

#### Crates, Modules and Packages
A medida que los proyectos se van haciendo mas grandes, es mandatorio organizar el código en componentes independientes utilizando la parte pública (o interfaces) del mismo, dando igual el como esté implementado. `Rust` ofrece la siguiente organización del código
- **Packages**: una funcionalidad de `Cargo` que permite construir, probar y compartir `crates`.
- **Crates**: un árbol de módulos que tiene como resultado una librería y un ejecutable.
- **Modules and use**: te permite controlar la organización, el `scope` y la privacidad de los directorios
- **Paths**: la manera de nombrar un elemento, como un `struct`, `función` o `módulo`.

Ejemplo de `module` and `use`:
```rust

// Modules ///////
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn secret() {
        println!("Esto es privado 😎");
    }
}

// Use ///////
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    // sin use
    let x = crate::math::add(2, 3);

    // con use
    use crate::math::add;
    let y = add(5, 7);

    println!("x = {}, y = {}", x, y);
}

// Privacy ///////
mod outer {
    pub mod inner {
        pub fn hello() {}
        fn hidden() {}
    }
}

fn main() {
    outer::inner::hello(); // funciona
    // outer::inner::hidden(); // ❌ error: función privada
}
```

Ejemplo de `paths`
- simple
```rust
fn foo() {}
fn main() {
    foo(); // path simple
}
```
- absoluto
```rust
mod a {
    pub fn hello() {}
}

fn main() {
    crate::a::hello(); // path absoluto
}
```
- relativo
```rust
mod a {
    pub fn f() {}
    pub mod b {
        pub fn g() {}
        pub fn call_parent() {
            super::f(); // sube un nivel y llama a f
        }
    }
}
```

##### Crates
Existen dos tipos de crates:
- binarios: son los ejecutables generados a raíz del código fuentes, es decir, los ficheros compilados. *Deben tener una función `main` que defina lo que hay que hacer cuando el módulo se ejecute*
- librerías: No llevan función `main` y no se compilan en un ejecutable, solamente definen funcionalidad que se utiliza en distintos proyectos. Por ejemplo el módilo `rand` o el `math`.

> El _crate root_ es un archivo fuente desde el que parte el compilador Rust y que constituye el módulo raíz de tu crate.
##### Packages
Un `paquete` es un conjunto de uno o más `crates` que proporciona un conjunto de funcionalidades. Un `Package` contiene un archivo `Cargo.toml` que describe cómo compilar esos `crates`.

##### Modules cheatsheet
- **Start from the crate root**: cuando compilamos un `crate`, el compilador primeramente mira el `crate root` (`main.rs` o `binary crate`) buscando código para compilar.

- **Declaring modules**: en el fichero `crate root` puedes declarar nuevos módulos con el comando `mod nombre-modulo`, el compilador buscará el código de ese módulo en los siguientes sitios:
    - `Inline`, dentro de {} que reemplazan al punto y coma después del `mod nombre-modulo`
    - En el fichero `src/garden.rs`
    - En el fichero `src/garden/mod.rs`

- **Declaring submodules**: En cualquier fichero que no sea el `crate root`, puedes declarar submodulos. Por ejemplo, tu puedes declarar `mod vegetables;` in `src/garden.rs`. El compilador mirará por el código del submódulo dentro del directorio nombrado por el modulo padre en los siguientes sitios:
    - Inline, directamente siguiendo `mod vegetables`, dentro de `{}` en lugar del punto y coma `;`
    - En el fichero `src/garden/vegetables.rs`
    - En el fichero `src/garden/vegetables/mod.rs`

- **Paths to code in modules**: Una vez el módulo sea parte del `crate`, tu te puedes referir al código de ese módulo desde donde sea del mismo `crate`, siempre que lo permitan las reglas de privacidad. Por ejemplo, un tipo de vegetal`Asparagus` del módulo de los vegetales puede ser encontrado en `crate::garden::vegetables::Asparagus`.

- **Private vs. public**: El código dentro de un módulo es privado por sus módulos padres por defecto. Para hacerlo público hay que declararlo con el comando  `pub mod` en lugar de `mod`. Para hacer elementos públicos dentro de un módulo público usar `pub` antes de declararlo.

- **The `use` keyword**: Dentro de un `scope`, la palabra clave `use` crea un atajo a los elementos para reducir la repetición de `paths` largos. En cualquier `scope` que se refiera a  `crate::garden::vegetables::Asparagus`, se puede crear un atajo con `use crate::garden::vegetables::Asparagus;` entonces solamente necesitarás escribir `Asparagus` para hacer uso en el `scope`

La manera mas recomendable de usar librerías y módulos es mediante el uso de los paths absolutos (por convención) ya que evita que se produzcan confusiones en el nombrado

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

Se pueden añadir alias mediante el comando `as` permitiendo acortar los `use`:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

#### External Packages
De manera similar a Python con el `requirements.txt`, lo añadimos al fichero `Cargo.toml`
```toml
[package]
name = "restaurant"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
```

Cuando hagamos `build`, se descargará el paquete y será disponible a través de `use`
```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

Y en el caso de que nos queramos traer varios módulos de la misma librería, podemos anidar los `use`:
```rust
use std::Collections::Hasmap;
use std::io;
// Se puede simplidicar como
use std::{Collections::Hasmap, io};

// En caso de que coincidan módulos, se puede sacar todo lo común
use std::io;
use std::io::write;
// Se puede simplificar como
use std::io{self, write};
```

##### Glob Operator
Sirve para, de manera similar a `Python`, importar todos los items definidos en una colección:

```python
from settings import *
```

En `Rust` sería
```rust
use std::io::*;
```

Un ejemplo del `use` de global:
```rust
// use std::io::stdin; Se puede utilizar este en el ejemplo
use std::io::*;

fn main() {
    println!("Escribe tu nombre:");

    let mut nombre = String::new();
    // El stdin pertenece al modulo std::io
    stdin().read_line(&mut nombre).expect("Error al leer la línea");

    println!("Hola, {}!", nombre.trim());
}```

##### Separating Modules into Different Files
Para mantener cierta organización dentro del módulo es posible separar funcionalidad en distintos ficheros.
Se puede tirar del crate o del módulo usando:
- `use`: importa elementos que ya existen para usarlos más fácilmente.
	- _El módulo ya debe existir (declarado con mod) para poder importarlo_
- `mod`: declara que existe un módulo. Le dice al compilador "aquí hay un módulo".
	- _Rust buscará el código en front_of_house.rs o front_of_house/mod.rs (está última es la antigua)_

```rust
mod front_of_house;
// ó
pub use crate::front_of_house::hosting;
```

#### Collections
##### Vectors

Para definir un vector se sigue el siguiente formato:
```rust
// Fijo -> No se puede añadir elementos
let vector: vec<i32> = Vec::new()
// Variable -> Se puede añadir elementos
let mut vector: vec<i32> = Vec::new()
// Se puede declarar tambien con la macro
let vector = vec![<items>];
// Admite tambien distintos tipos
let vector_string: v<Str>
```

Para **añadir** elementos, podemos utilizar la función `push(<item>)`, esta añade elementos al final del vector.
```rust
let vector = vec![1, 2, 3];
vector.push(4);
// vector -> vector[1, 2, 3, 4];
```

Para **eliminar** elementos, podemos utilizar la función `pop()`, esta elimina el último elemento del vector.

```rust
let vector = vec![1,2,3]
vector.pop()
// vector -> vector[1, 2]
```

El acceso a los distintos elementos del vector se puede hacer de las siguientes maneras:

- Acceso mediante la referencia
```rust
let vector = vec![1, 2, 3];
println!("El segundo elemento del vector es: {}", &vector[1]);
```

- Acceso mediante el método `vector.get()`
```rust
let vector = vec![1, 2, 3];
println!("El segundo elemento del vector es: {}", vector.get(1));
```

Pero es necesario que controlar que no se pueda acceder a una posición invalida de memoria, tanto si accedemos por referencia como si accedemos por el método `.get()` debemos controlarlo de la siguiente manera:

```rust
let vector = vec![1, 2, 3];
if Some(elemento_100) = vector.get(100) {
	println!("El elemento 100 existe");
} else {
	println!("El elemento 100 no existe");
}
```

El `panic`que muestra es el siguiente:

```bash
# Linea con error
let third_element = &vector_elements[200];
# Salida
thread 'main' panicked at src/main.rs:27:41:
index out of bounds: the len is 7 but the index is 200
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Para recorrer los elementos de un vector lo podemos hacer de la siguiente manera:
```rust
let vector = vec![1, 2, 3, 4];
for item in vector {
	println!("Elemento del vector {}", item);
}
```

Si queremos editar valores del vector, tenemos que declararlo como mutable, y luego dentro del bucle utilizar el operador * (_operador de dereferencia_) para acceder contenido:
```rust
for item in &mut vector {
	println!("Elemento del vector {}", item);
	*i += 20; // Sumamos 20 a cada uno de los elementos del vector
}
```

Para saber el número de elementos del vector podemos utiliza la función `vector.len()`
##### Using `enum` to store multiple data types
El vector no está limitado a un solo tipo de datos, mediante una `enum`, podemos definir un vector que contenga todos los tipos de datos definidos en el `enum`. Por ejemplo:
```rust
let enum CsvRow{
	Int(i32),
	Float(f32),
	String(String),
}

fn main {
	let vector vec![
		CsvRow::Int(8),
		CsvRow::Float(1.89),
		CsvRow::String("Jose Manuel"),
	];
	// -> vector[8, 1.89, "Jose Manuel"]
}
```

##### `Strings`

> No confundir con `str`

Las cadenas no dejan de ser vectores de caracteres. Se pueden definir de la siguiente manera:
```rust
let mut string_1 = "Hola Mundo!".to_string()
// ó
let mut string_2 = String::from("Hola Mundo!");
```

Si queremos concatenar cadenas, a la cadena original se le añade la otra cadena

```rust
let mut hello = String::from("Hola ");
hello.push_str("World");
// Y si queremos añadir solamente un caracter, podemos hacerlo mediante push
hello.push('!'); // Si, con comillas simples
```

Si decidimos concatenar cadenas con el operador `+` debemos tener en cuenta que para las concatenaciones, se debe utilizar la referencia en la segunda y siguientes:

```rust
let concat_string_1 = String::from("Hola ");
let concat_string_2 = String::from("Mundo");
let concat_string_3 = String::from("!");

let concat_string = concat_string_1 + &concat_string_2 + &concat_string_3;
println!("Concatenacion con +: {}", concat_string);
```

> Además `concat_string_1` ya no puede utilizarse

Esto es por la signatura del método `add`

```rust
impl Add<&str> for String {
	fn add(self, other: &str) -> String
}
```

El primer argumento hace referencia a `self`, por lo tanto, se pasa la variable _as is_ pero el segundo argumento que debe recibir es la referencia a `str`.  No obstante, hay una alternativa para concatenar cadenas, mas sencilla y fácil de leer.

```rust
let concat_string_1 = String::from("Hola ");
let concat_string_2 = String::from("Mundo");
let concat_string_3 = String::from("!");

let concat_string = format!("{concat_string_1} {concat_string_2}{concat_string_3}");
```

##### Indexing to String
`Rust` _no soporta el indexado de cadenas, por ejemplo, esto daría error:_

```rust
let cadena: String = String::from("Hola Mundo!");
let h_char = cadena[0];  // No retorna 'H'
```

Esto se debe a que:

>Si alguien te preguntara cuán larga es esta cadena ("`Здравствуйте`"), podrías decir que 12. De hecho, la respuesta de Rust es 24: ese es el número de bytes que se necesitan para codificar dicha cadena en `UTF-8`, porque cada valor escalar `Unicode` en esa cadena ocupa 2 `bytes` de almacenamiento. Por lo tanto, un índice dentro de los `bytes` de la cadena no siempre corresponde a un valor escalar `Unicode` válido.
>La respuesta, entonces, es que para evitar devolver un valor inesperado y causar errores que podrían no descubrirse de inmediato, `Rust` **no compila este código en absoluto** y previene malentendidos desde las primeras etapas del desarrollo.

#### Hash Maps
Al igual que en la mayoría de lenguajes, `Rust` tiene un tipo de dato denominado `HashMap`, por ejemplo en `Python` se llaman Diccionarios (`Dict`).

Para definir un `HashMap` lo podemos hacer de la siguiente manera:

```rust
use std::collections::HashMaps

let mut population_map = Hashmap::new()

population_map.insert(String::from("España"), 10);
population_map.insert(String::from("España"), 20);
```

#### Generic Types, Traits, and Lifetimes

Los genéricos son tipos abstractos que permiten escribir código reutilizable sin duplicación. En lugar de crear funciones separadas para cada tipo concreto (como i32 o String), defines una única función que trabaja con cualquier tipo `<T>`. Ya has usado genéricos como `Option<T>`, `Vec<T>`, `HashMap<K, V>` y `Result<T, E>`. Los `traits` te permiten restringir los genéricos para que solo acepten tipos con ciertos comportamientos. Los lifetimes son un tipo especial de genérico que indica al compilador cuánto tiempo vivirán las referencias, garantizando la seguridad de memoria.

```rust
// Función genérica que encuentra el mayor elemento

fn encontrar_mayor<T: PartialOrd>(lista: &[T]) -> &T {
    let mut mayor = &lista[0];
    for item in lista {
        if item > mayor {
            mayor = item;
        }
    }
    mayor
}

fn main() {
    let numeros = vec![34, 50, 25, 100, 65];
    println!("El mayor número es: {}", encontrar_mayor(&numeros));
    let caracteres = vec!['y', 'm', 'a', 'q'];
    println!("El mayor carácter es: {}", encontrar_mayor(&caracteres));
}
```

En este ejemplo, `<T: PartialOrd>` significa que T puede ser cualquier tipo que pueda compararse (PartialOrd), permitiendo usar la misma función para números, caracteres, strings, etc.

#### Removing Duplication by Extracting a Function

[_Apartado un tanto irrelevante_]

Basicamente dice que si tenemos esto:

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}
```

Pasemos a esto:
```rust
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {result}");
}
```

Evitando duplicar de esta manera el código y haciendolo modular.
#### Generic Data Types

Útil para cuando tenemos la misma función para distintos tipos de datos, se implementa añadiendo `<T>` justamente después del nombre de la función o estructura. Por ejemplo:

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let wont_work = Point { x: 5, y: 4.0 }; // Obviamente este no funcionará
    // A la hora de asignar x=5 ya estamos diciendo que el tipo de <T> será
    // int (*)
}
```

De esta manera podemos declarar variables de tipo `struct Point<T>`y asignarle valores `int` o `float`, pero nunca `int` y `float`. Para definir cada variable de un tipo diferente debemos añadir otra variable entre los angulos en la definición de la estructura, de la siguiente manera:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

> _Cuando utilizamos estas signaturas genéricas para implementar una función donde se le mete variables de tipo_ `int` _y de tipo_ `char`, _debemos implementar un_ `trait` ya que puede dar problemas de compilación. Como se puede en el siguiente caso:

 ```rust
 fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}
 ```

Este problema se corrige indicando el *trait* `PartialOrd` junto con el tipo:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

Una vez incluido el `trait` compila sin errores, pudiendo utilizar dicha función tanto para enteros (`i32`) como para caracteres (`char`).

#### In Struct Definitions

También es posible utilizar el tipo generico `<T>`en la definición de estructuras para que los campos de la estructura puedan ser de X tipo o de Y tipo, **pero los dos iguales**.

```rust
struct Point<T> {
	x: T,
	y: T,
}
```

Si queremos utilizar tipos diferentes para cada uno de los campos

```rust
struct MixedPoint<T, U> {
	x: T,
	y: U,
}
```

#### In Enums definitions
De manera similar a las `struct`, también es posible definir `enums` con tipos genéricos en sus variantes. Tomemos otro vistazo al enum `Option<T>` que proporciona la biblioteca estándar:

```rust
enum Option<T> {
	Some(T),
	None,
}

```

Al usar el enum `Option<T>`, podemos expresar el concepto abstracto de un valor opcional, y debido a que `Option<T>` es genérico, podemos usar esta abstracción sin importar cuál sea el tipo del valor opcional.

Los enums también pueden usar múltiples tipos genéricos. La definición del enum `Result` que usamos es un ejemplo:

```rust
enum Result<T, E> {
	Ok(T),
	Err(E),
}
```

El enum `Result` es genérico sobre dos tipos, `T` y `E`, y tiene dos variantes: `Ok`, que contiene un valor de tipo `T`, y `Err`, que contiene un valor de tipo `E`. Esta definición hace conveniente usar el enum `Result` en cualquier lugar donde tengamos una operación que pueda tener éxito (retornar un valor de algún tipo `T`) o fallar (retornar un error de algún tipo `E`). De hecho, esto es lo que usamos para abrir un archivo, donde `T` se rellenó con el tipo `std::fs::File` cuando el archivo se abrió exitosamente y `E` se rellenó con el tipo `std::io::Error` cuando hubo problemas al abrir el archivo.

#### In Method Definitions
Se pueden definir incluso métodos (`impl`) de manera similar que los `struct`, `enums` y demás

```rust

struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}

fn main() {
	let p = Point { x: 5, y: 10 };
	println!("p.x = {}", p.x());
}

```

Se puede incluso definir un tipo concreto (p.ej. `<f32>`) en lugar del `<T>`. Un ejemplo de ello es el siguiente:

```rust

struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}

// Método concreto para floats
impl Point<f32> {
	fn x_float(&self) -> &f32 {
		&self.x
	}
}

// Fin método concreto
fn main() {
	let p = Point { x: 5, y: 10 };
	println!("p.x = {}", p.x());
}

```

#### Performance of Code Using Generics
El uso de Generics no ralentiza el rendimiento del programa con respecto a los tipos concretos. Esto es por la *monomorfización* del código

> *La monomorfización del código consiste en la generación de código concreto a partir de código generico mediante el rellenado de los tipos concretos cuando va a ser compilado*

El compilador busca todos los sitios donde se llama el código genérico y genera el código concreto que se ha especificado.

```rust
let integer = Some(5);
let float = Some(5.0);
```

Internamente se maneja así

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

#### Defining Shared Behavior with Traits (*Rasgos*)
Un *trait* define funcionalidad que un tipo particular puede tener y que puede ser compartida con otros tipos. Los *traits* pueden ser utilizados para definir comportamiento compartido de manera asbtracta.

> Los *trait* son similares a lo que se conoce en otros lenguajes de programación como interfaces, pero con ciertos matices.

#### Defining a Trait

El comportamiento de un tipo son los métodos que podemos llamar en ese tipo. Por ejemplo, en Python si declaramos una cadena, podemos llamar al método split para trocear una cadena mientras que si declaramos una variable numérica da error al llamar al método `.split()`.

```bash
>>> prueba="Esto es una prueba"
>>> split_prueba = prueba.split(" ")
>>> prueba
'Esto es una prueba'
>>> split_prueba
['Esto', 'es', 'una', 'prueba']
>>> prueba_numero = 5
>>> split_numero = prueba_numero.split()
Traceback (most recent call last):
  File "<python-input-6>", line 1, in <module>
    split_numero = prueba_numero.split()
                   ^^^^^^^^^^^^^^^^^^^
AttributeError: 'int' object has no attribute 'split'
```

Diferentes tipos comparten el mismo comportamiento si podemos llamar al mismo método independientemente del tipo.

#### Implementing a Trait on a Type

 Se hace de manera similar que si se estuviera implementando un método regular

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

La diferencia es que es que después de de `impl` utilizamos `for` y el especificamos el tipo para el que lo queremos implementar. Dentro de las llaves definimos los métodos que queramos para el tipo de método.

> `pub trait <TIPO>` ➔ Define el contrato
> `impl <nombre-trait> for <TIPO>` ➔ Implementa el contrato

#### Using Default Implementations

A veces es útil tener un comportamiento predeterminado para algunos o todos los métodos en un rasgo en lugar de requerir implementaciones para todos los métodos en cada tipo.

Por ejemplo, para usar la implementación por defecto, especificamos un bloque `impl` vacío de la siguiente manera `impl Summary for NewArticle {}`

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

Para usar esta versión de `Summary`, solo necesitamos definir `summarize_author` cuando implementemos el `trait` en un tipo:

```rust
impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

Y la llamada se haría de la siguiente manera:

```rust
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
```

### Using Traits as Parameters

Para utilizar los `traits` como parámetros, lo hacemos de la siguiente manera:

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Especificamos `impl` y el nombre del `trait`. Esto lo hacemos porque este parámetro acepta cualquier tipo especificado por el `trait`.

El cuerpo de `notify` podemos llamar a cualquier método en `item` que provenga del `trait` de `Summary` como por ejemplo `summarize`. Ahora podemos llamar a `notify` y pasarle cualquier instancia `NewsArticle` o `SocialPost`, en caso de  que no lo implemente (como por ejemplo `i32` o `f64`), directamente no compilará.

#### Trait Bound Syntax

La sintaxis del `impl Trait` simplificar la manera original o primitiva de la funcionalidad, la manera larga es la siguiente:
```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

Esta sintaxis es mas completa y, por lo tanto, puede expresar mayor complejidad por ejemplo, en el caso de que haya 2 parámetros que quieran implementar `Summary`.

La sintaxis de `impl Trait` es conveniente y permite un código más conciso en casos sencillos, mientras que el uso de la forma primitiva puede expresar mayor complejidad en otros casos.

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

El uso de la función `impl Trait` es apropiado si queremos que esta función permita que `item1` e `item2` tengan tipos diferentes. Si queremos forzar que ambos parametros tengan el mismo tipo.

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

El tipo genérico T especificado como el tipo de los parámetros item1 e item2 restringe la función de tal manera que el tipo concreto del valor pasado como argumento para item1 e item2 debe ser el mismo.

#### Multiple Trait Bounds with the + Syntax
Podemos definir mas de un `trait bound` con uso del operador `+` por ejemplo, para el caso de `notify` si queremos que se utilice el formato de `summarize` y el de `display`, podemos hacerlo mediante el siguiente código.

```rust
pub fn notify(item: &(impl Summary + Display)) {
```

También es válido en tipos de datos genericos

```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

#### Clearer Trait Bounds with `where` Clauses
Todo este uso de `traits` puede implicar que la signatura de la función quede poco legible, para ello se puede utilizar la cláusula `where` después de la función. Por lo tanto, en lugar de escribir esto:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

Debemos escribir, lo siguiente:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

#### Returning Types That Implement Traits
También podemos usar el `impl Trait` como valor de retorno de algun tipo que implemente un `trait`

```rust
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}
```

Con el uso de `impl Summary`para el tipo de retorno especificamos que `return_summarizable` retorna algún tipo que implementa el `trait Summary` sin mencionar el tipo concreto. En este caso `return_summarizable` retorna `SocialPost`. Pero el código que está llamando a esta función no necesita saberlo.

#### Using Trait Bounds to Conditionally Implement Methods

-> https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=of%20Chapter%2018.-,Using%20Trait%20Bounds%20to%20Conditionally%20Implement%20Methods,-By%20using%20a