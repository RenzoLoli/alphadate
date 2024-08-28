use std::sync::Arc;

use crate::{
    database::{Connection, QueryBuilder},
    domain::EDateIdea,
};

use super::BaseRepository;

#[derive(Default)]
pub struct DateIdeaRepository {
    connection: Arc<Connection>,
}

impl DateIdeaRepository {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }
}

impl BaseRepository<EDateIdea> for DateIdeaRepository {
    fn get_connection(&self) -> &Connection {
        &self.connection
    }
}

impl DateIdeaRepository {
    pub async fn find_by_idea(&self, idea: &str) -> Vec<EDateIdea> {
        let query = QueryBuilder::new("date_ideas")
            .q_select()
            .q_where_eq("idea", idea)
            .get_query();

        self.query_search(query).await
    }
}

// "SELECT *,
//     (SELECT
//         (SELECT *
//         FROM tags
//         WHERE $parent.tag_id = id) as tags
//     FROM date_idea_tags
//     WHERE $parent.id = date_idea_id).tags[0] as tags
// FROM date_ideas
// ",
