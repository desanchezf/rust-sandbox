## 0. Glosario

`rustup`: gestor de versiones de Rust y herramientas asociadas, similar a NVM
`cargo`: gestor de paquetes de Rust, similar a pip, se encarga tambien de:
	👉 Compilar tu código: `cargo build`
	👉 Ejecutar tu aplicación:  `cargo run`
	👉 Ejecutar pruebas unitarias y de integración: `cargo test`
	👉 Manejar dependencias: Usa el archivo `Cargo.toml` para declarar y gestionar las librerías (crates) que tu proyecto necesita.
	👉 Publicar crates en [crates.io](https://crates.io)  `cargo publish`
	👉 Actualizar dependencias  👉 `cargo update`
`crate`:Un crate es la unidad minima de distribución de paquetes, la diferencia principal es que un crate puede comprender varios ficheros. Un fichero generado mediante `rustc` solamente comprende un solo fichero.
`rustc`: compilador de Rust, se llama de la siguiente manera `rustc file.rs -o output` en caso de que no se utilice la flag `-o`se utiliza el nombre del fichero `file` como nombre del fichero de salida.
`trait`: es como un **contrato** o **interfaz** que define **comportamientos (métodos)** que un tipo debe implementar si quiere "cumplir" con ese `trait`

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
- Cuando el _owner_ sale del ambito, el valor será eliminado

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
	user_struct = UserStructu {
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

## [The `match` Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html#the-match-control-flow-construct)