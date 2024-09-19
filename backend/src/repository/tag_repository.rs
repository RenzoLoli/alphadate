use std::sync::Arc;

use crate::{
    database::{Connection, DbHelper, QueryBuilder},
    domain::{ETag, Entity},
};

use super::BaseRepository;

#[derive(Default)]
pub struct TagRepository {
    connection: Arc<Connection>,
}

impl TagRepository {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }
}

impl BaseRepository<ETag> for TagRepository {
    fn get_connection(&self) -> &Connection {
        &self.connection
    }
}

impl TagRepository {
    pub async fn find_by_name(&self, name: &str) -> Vec<ETag> {
        let table_name = ETag::get_table_name();
        let query = QueryBuilder::new(table_name)
            .q_select()
            .q_where_eq("name", &DbHelper::as_db_string(name))
            .get_query();

        self.query_search(query).await
    }
}
