use std::sync::Arc;

use crate::database::{DbHelper, QueryBuilder};
use crate::{database::Connection, domain::EUser};

use crate::repository::BaseRepository;

#[derive(Default)]
pub struct UserRepository {
    connection: Arc<Connection>,
}

impl UserRepository {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }
}

impl BaseRepository<EUser> for UserRepository {
    fn get_connection(&self) -> &Connection {
        &self.connection
    }
}

impl UserRepository {
    pub async fn find_by_email(&self, email: &str) -> Option<EUser> {
        let query = QueryBuilder::new("users")
            .q_select()
            .q_where_eq("email", &DbHelper::as_db_string(email))
            .get_query();
        let users = self.query_search(query).await;
        users.first().map(|user| user.to_owned())
    }
}
