use crate::{
    database::Connection,
    domain::{DateIdea, DateIdeaDto},
};

pub struct DateIdeaRepository {
    connection: Connection,
}

impl DateIdeaRepository {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    pub async fn get_all(&self) -> Vec<DateIdea> {
        let db = self.connection.db();

        let ideas: Vec<DateIdeaDto> = db.select("date_ideas").await.ok().unwrap_or(vec![]);

        ideas.iter().map(DateIdea::from).collect()
    }
}
