use std::sync::Arc;

use crate::{database::Connection, domain::EDateIdeaTag};

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
        self.find_by_where("tag_id", tag_id).await
    }

    pub async fn find_by_date_idea_id(&self, date_idea_id: &str) -> Vec<EDateIdeaTag> {
        self.find_by_where("date_idea_id", date_idea_id).await
    }
}
