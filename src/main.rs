mod models;

use models::dog::{Dog, Gender};


fn main() {
    
    let oliver = Dog::new(1, String::from("Oliver"), String::from("Rottwiler"), Gender::Male, String::from("12-12-2000"), Some(String::from("123456789")),None);

    println!("Created dog: {0},  id: {1}", oliver.name, oliver.id)

}
