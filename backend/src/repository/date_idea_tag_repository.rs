use std::sync::Arc;

use crate::{
    database::{Connection, DbHelper, QueryBuilder},
    domain::EDateIdeaTag,
};

use super::BaseRepository;

#[derive(Default)]
pub struct DateIdeaTagRepository {
    connection: Arc<Connection>,
}

impl DateIdeaTagRepository {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }
}

impl BaseRepository<EDateIdeaTag> for DateIdeaTagRepository {
    fn get_connection(&self) -> &Connection {
        &self.connection
    }
}

impl DateIdeaTagRepository {
    pub async fn find_by_tag_id(&self, tag_id: &str) -> Vec<EDateIdeaTag> {
        let query = QueryBuilder::new("date_idea_tags")
            .q_select()
            .q_where_eq("tag_id", &DbHelper::as_db_string(tag_id))
            .get_query();

        self.query_search(query).await
    }

    pub async fn find_by_date_idea_id(&self, date_idea_id: &str) -> Vec<EDateIdeaTag> {
        let query = QueryBuilder::new("date_idea_tags")
            .q_select()
            .q_where_eq("date_idea_id", &DbHelper::as_db_string(date_idea_id))
            .get_query();

        self.query_search(query).await
    }
}
