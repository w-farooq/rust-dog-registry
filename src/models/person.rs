pub enum Role {
    User,
    Breeder,
    Seller,
    Admin,
    Vet,
}

pub struct Person {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub location: String,
    pub roles: Vec<Role>,
}

impl Person {
    pub fn new(id: u32, name: String, email: String, location: String, role: Role) -> Self {
        Person {
            id,
            name,
            email,
            location,
            roles: vec![role],
        }
    }
}
