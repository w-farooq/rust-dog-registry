use crate::models::dog::Dog;
use crate::models::litter::Litter;
use crate::models::person::Person;

pub struct Registry {
    dogs: Vec<Dog>,
    persons: Vec<Person>,
    litters: Vec<Litter>,
}

impl Registry {
    // Create a new empty registry
    pub fn new() -> Self {
        Registry {
            dogs: Vec::new(),
            persons: Vec::new(),
            litters: Vec::new(),
        }
    }

    // 1. Add a dog to the registry
    pub fn add_dog(&mut self, dog: Dog) {
        self.dogs.push(dog);
    }

    // 2. Find a dog by ID (return a reference, not ownership)
    pub fn find_dog(&self, id: u32) -> Option<&Dog> {
        for dog in &self.dogs {
            if id == dog.id {
                return Some(&dog);
            }
        }

        None
    }

    // 3. Get all dogs (return references)
    pub fn get_all_dogs(&self) -> &Vec<Dog> {
        &self.dogs
    }

    // 4. Delete dog by id
    pub fn delete_dog(&mut self, id: u32) -> Option<Dog> {
        for (index, dog) in self.dogs.iter().enumerate() {
            if dog.id == id {
                let removed_dog = self.dogs.remove(index);
                return Some(removed_dog);
            }
        }

        None
    }

    

  
}
