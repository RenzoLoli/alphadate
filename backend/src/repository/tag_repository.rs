use std::sync::Arc;

use crate::{database::Connection, domain::ETag};

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
