use std::sync::Arc;

use crate::{
    database::{Connection, DbHelper},
    domain::{EDateIdea, Entity, RDateIdeaTag},
};

use super::BaseQuerySearch;
use super::BaseRepository;

#[derive(Default)]
pub struct DateIdeaTagRefRepository {
    connection: Arc<Connection>,
}

impl DateIdeaTagRefRepository {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }
}

impl BaseRepository<RDateIdeaTag> for DateIdeaTagRefRepository {
    fn get_connection(&self) -> &Connection {
        &self.connection
    }
}
impl BaseQuerySearch<RDateIdeaTag> for DateIdeaTagRefRepository {}

impl DateIdeaTagRefRepository {
    pub async fn get_refs(&self) -> Vec<RDateIdeaTag> {
        let query = "SELECT *,
            (SELECT
                (SELECT *
                FROM tags
                WHERE $parent.tag_id = id) as tags
            FROM date_idea_tags
            WHERE $parent.id = date_idea_id).tags[0] as tags
        FROM date_ideas
        ";

        self.query_search(query.to_owned()).await
    }

    pub async fn find_ref(&self, id: &str) -> Option<RDateIdeaTag> {
        let query = format!(
            "SELECT *,
                (SELECT
                    (SELECT *
                    FROM tags
                    WHERE $parent.tag_id = id) as tags
                FROM date_idea_tags
                WHERE $parent.id = date_idea_id).tags[0] as tags
            FROM date_ideas
            WHERE id = {}
            ",
            DbHelper::id_to_thing(EDateIdea::get_table_name(), id)
        );

        self.query_search(query).await.pop()
    }
}
