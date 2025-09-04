// Así se define un enum
use std::fmt;

enum Role{
    Admin(String),
    User(String),
    Superuser(String),
}
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin(description) => write!(f, "Admin: {}", description),
            Role::User(description) => write!(f, "User: {}", description),
            Role::Superuser(description) => write!(f, "Superuser: {}", description),
        }
    }
}

struct User {
    name: String,
    role: Role,
}


fn main() {

    // Inicializacion de un enum
    let admin = User {
        name: String::from("Juan Pedro"),
        role: Role::Admin(String::from("Admin")),
    };
    let user = User {
        name: String::from("Pedro antonio"),
        role: Role::User(String::from("User")),
    };
    let superuser = User {
        name: String::from("Pedro antonio"),
        role: Role::Superuser(String::from("Superuser")),
    };

    println!("El nombre del usuario es: {}", admin.name);
    println!("El rol del usuario es: {}", admin.role);
    println!("El nombre del usuario es: {}", user.name);
    println!("El rol del usuario es: {}", user.role);
    println!("El nombre del usuario es: {}", superuser.name);
    println!("El rol del usuario es: {}", superuser.role);
}
