// Estamos accediendo al fichero creado en vegetables y concretamente al tipo definido en su interior
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}