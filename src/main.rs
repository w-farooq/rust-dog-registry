mod models;
mod registry;

use models::dog::{Dog, Gender};
use registry::Registry;

fn main() {
    let mut registry = Registry::new();

    // Create dogs
    let oliver = Dog::new(
        1,
        String::from("Oliver"),
        String::from("Rottweiler"),
        Gender::Male,
        String::from("12-12-2020"),
        Some(String::from("123456789")),
        None,
    );

    let bella = Dog::new(
        2,
        String::from("Bella"),
        String::from("German Shepherd"),
        Gender::Female,
        String::from("05-03-2019"),
        None,
        None,
    );

    // Add dogs
    registry.add_dog(oliver);
    registry.add_dog(bella);

    // Test find_dog
    match registry.find_dog(1) {
        Some(dog) => println!("Found dog: {}", dog.name),
        None => println!("Dog not found"),
    }

    // Test delete dog
    match registry.delete_dog(1) {
        Some(dog) => println!("Deleted: {}", dog.name),
        None => println!("Not found"),
    }

    // Test get_all_dogs
    println!("Total dogs: {}", registry.get_all_dogs().len());
}
