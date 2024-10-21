use std::sync::Arc;

use crate::{
    database::{Connection, DbHelper, QueryBuilder},
    domain::{EDateIdeaTag, Entity},
};

use super::BaseRepository;
use super::{BaseQuerySearch, BaseTransactions};

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
impl BaseQuerySearch<EDateIdeaTag> for DateIdeaTagRepository {}
impl BaseTransactions<EDateIdeaTag> for DateIdeaTagRepository {}

impl DateIdeaTagRepository {
    pub async fn find_by_date_idea_and_tag_id(
        &self,
        date_idea_id: &str,
        tag_id: &str,
    ) -> Option<EDateIdeaTag> {
        let table = EDateIdeaTag::get_table_name();
        let query = QueryBuilder::new(table)
            .q_select()
            .q_where_eq(
                "date_idea_id",
                &DbHelper::id_to_thing("date_ideas", date_idea_id),
            )
            .q_and_eq("tag_id", &DbHelper::id_to_thing("tags", tag_id))
            .get_query();

        self.query_search(query).await.pop()
    }

    pub async fn find_by_tag_id(&self, tag_id: &str) -> Vec<EDateIdeaTag> {
        let table = EDateIdeaTag::get_table_name();
        let query = QueryBuilder::new(table)
            .q_select()
            .q_where_eq("tag_id", &DbHelper::id_to_thing("tags", tag_id))
            .get_query();

        self.query_search(query).await
    }
}
