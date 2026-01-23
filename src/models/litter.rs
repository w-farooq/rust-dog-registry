pub struct Litter {
    pub id: u32,
    pub sire_id: u32,
    pub dam_id: u32,
    pub breeder_id: u32,
    pub date_of_birth: String,
    pub puppy_ids: Vec<u32>,
}

impl Litter {
    pub fn new(id: u32, sire_id: u32, dam_id: u32, breeder_id: u32, date_of_birth: String) -> Self {
        Litter {
            id,
            sire_id,
            dam_id,
            breeder_id,
            date_of_birth,
            puppy_ids: Vec::new(),
        }
    }

    pub fn add_puppy(&mut self, puppy_id: u32) {
        self.puppy_ids.push(puppy_id);
    }
}
