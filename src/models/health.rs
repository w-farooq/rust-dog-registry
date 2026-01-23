pub struct HealthRecord {
    pub id: u32,
    pub dog_id: u32,
    pub record_type: HealthRecordType,
    pub date: String,
    pub description: String,
    pub vet_name: Option<String>,
}

pub enum HealthRecordType {
    Vaccination,
    VetVisit,
    HipScore,
    GeneticTest,
    Injury,
    Surgery,
}

impl HealthRecord {
    pub fn new(
        id: u32,
        dog_id: u32,
        record_type: HealthRecordType,
        date: String,
        description: String,
        vet_name: Option<String>,
    ) -> Self {
        HealthRecord {
            id,
            dog_id,
            record_type,
            date,
            description,
            vet_name,
        }
    }
}
