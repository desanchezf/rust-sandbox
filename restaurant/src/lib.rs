// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// // Al ser privado por defecto, no se puede acceder

// fn deliver_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order() {}
// }

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast.
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change our mind about what bread we'd like.
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal.
//     // meal.seasonal_fruit = String::from("blueberries");
// }


// En el caso de paths largos tenemos la opción de utilizar el comando use
// que permite traer un path al scope actual

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     // Aqui directamente utilizamos hosting->método para llamar a la función
//     hosting::add_to_waitlist();
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;
// // Esto dará error porque al ser otro módulo, es otro scope diferente

// mod customer {
//     // Para correcgir este error, debemos definir el use dentro del módulo
//     // use crate::front_of_house::hosting;
//     // o podemos referenciar al modulo con super::hosting
//     // use super::hosting;
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }


// Si queremos reexportar nombre con pub y use

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}