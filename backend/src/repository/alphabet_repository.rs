use std::sync::Arc;

use crate::{
    database::{Connection, DbHelper, QueryBuilder},
    domain::{EAlphabet, Entity},
};

use super::{BaseQuerySearch, BaseRepository, BaseTransactions};

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
impl BaseQuerySearch<EAlphabet> for AlphabetRepository {}
impl BaseTransactions<EAlphabet> for AlphabetRepository {}

impl AlphabetRepository {
    pub async fn find_by_user_id(&self, user_id: &str) -> Vec<EAlphabet> {
        let table_name = EAlphabet::get_table_name();
        let query = QueryBuilder::new(table_name)
            .q_select()
            .q_where_eq("user_id", &DbHelper::id_to_thing("users", user_id))
            .get_query();

        self.query_search(query).await
    }

    pub async fn find_by_title(&self, title: &str) -> Option<EAlphabet> {
        let table_name = EAlphabet::get_table_name();
        let query = QueryBuilder::new(table_name)
            .q_select()
            .q_where_eq("title", &DbHelper::as_db_string(title))
            .get_query();

        self.query_search(query).await.pop()
    }
}
