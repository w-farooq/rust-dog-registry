pub enum Gender {
    Male,
    Female,
}

pub struct Dog {
    pub id: u32,
    pub name: String,
    pub breed: String,
    pub gender: Gender,
    pub dob: String,
    pub micro_chip: Option<String>,
    pub litter_id: Option<u32>,
}

impl Dog {
    pub fn new(
        id: u32,
        name: String,
        breed: String,
        gender: Gender,
        dob: String,
        micro_chip: Option<String>,
        litter_id: Option<u32>,
    ) -> Self {
        Dog {
            id,
            name,
            breed,
            gender,
            dob,
            micro_chip,
            litter_id,
        }
    }
}
