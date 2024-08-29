use std::sync::Arc;

use crate::{
    database::{Connection, DbHelper, QueryBuilder},
    domain::{EUserDate, Entity},
};

use super::BaseRepository;

#[derive(Default)]
pub struct UserDateRepository {
    connection: Arc<Connection>,
}

impl UserDateRepository {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }
}

impl BaseRepository<EUserDate> for UserDateRepository {
    fn get_connection(&self) -> &Connection {
        &self.connection
    }
}

impl UserDateRepository {
    pub async fn find_by_alphabet_id(&self, alphabet_id: &str) -> Vec<EUserDate> {
        let table_name = EUserDate::get_table_name();
        let query = QueryBuilder::new(table_name)
            .q_select()
            .q_where_eq(
                "alphabet_id",
                &DbHelper::id_to_thing(table_name, alphabet_id),
            )
            .get_query();

        self.query_search(query).await
    }

    pub async fn find_by_alphabet_and_date_idea_id(
        &self,
        alphabet_id: &str,
        date_idea_id: &str,
    ) -> Option<EUserDate> {
        let table_name = EUserDate::get_table_name();
        let query = QueryBuilder::new(table_name)
            .q_select()
            .q_where_eq(
                "alphabet_id",
                &DbHelper::id_to_thing(table_name, alphabet_id),
            )
            .q_and_eq(
                "date_idea_id",
                &DbHelper::id_to_thing(table_name, date_idea_id),
            )
            .get_query();

        self.query_search(query).await.pop()
    }

    pub async fn find_by_alphabet_id_and_letter(&self, letter: char) -> Vec<EUserDate> {
        let table_name = EUserDate::get_table_name();
        let query = QueryBuilder::new(table_name)
            .q_select()
            .q_where_eq(
                "alphabet_id",
                &DbHelper::id_to_thing(table_name, table_name),
            )
            .q_and_eq("letter", &DbHelper::as_db_string(&letter.to_string()))
            .get_query();

        self.query_search(query).await
    }
}
