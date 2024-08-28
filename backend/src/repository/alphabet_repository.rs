use std::sync::Arc;

use crate::{
    database::{Connection, DbHelper, QueryBuilder},
    domain::EAlphabet,
};

use super::BaseRepository;

#[derive(Default)]
pub struct AlphabetRepository {
    connection: Arc<Connection>,
}

impl AlphabetRepository {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }
}

impl BaseRepository<EAlphabet> for AlphabetRepository {
    fn get_connection(&self) -> &Connection {
        &self.connection
    }
}

impl AlphabetRepository {
    pub async fn find_by_user_id(&self, user_id: &str) -> Vec<EAlphabet> {
        let query = QueryBuilder::new("alphabet")
            .q_select()
            .q_where_eq("user_id", &DbHelper::as_db_string(user_id))
            .get_query();

        self.query_search(query).await
    }
}
