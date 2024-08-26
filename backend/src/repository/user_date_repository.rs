use std::sync::Arc;

use crate::{database::Connection, domain::EUserDate};

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
