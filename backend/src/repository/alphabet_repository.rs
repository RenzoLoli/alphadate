use std::sync::Arc;

use crate::{database::Connection, domain::EAlphabet};

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
