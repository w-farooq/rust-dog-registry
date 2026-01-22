pub struct CompetitionResult {
    pub id: u32,
    pub dog_id: u32,
    pub competition_name: String,
    pub date: String,
    pub score: f32,
    pub rank: Option<u32>,
    pub certification_awarded: Option<String>,
}

impl CompetitionResult {
    pub fn new(
        id: u32,
        dog_id: u32,
        competition_name: String,
        date: String,
        score: f32,
        rank: Option<u32>,
        certification_awarded: Option<String>,
    ) -> Self {
        CompetitionResult {
            id,
            dog_id,
            competition_name,
            date,
            score,
            rank,
            certification_awarded,
        }
    }
}