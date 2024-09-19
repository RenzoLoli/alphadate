use std::sync::Arc;

use crate::{
    database::{Connection, DbHelper, QueryBuilder},
    domain::{EDateIdea, Entity},
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
        let table_name = EDateIdea::get_table_name();
        let query = QueryBuilder::new(table_name)
            .q_select()
            .q_where_eq("idea", &DbHelper::as_db_string(idea))
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
