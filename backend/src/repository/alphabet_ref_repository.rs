use std::sync::Arc;

use crate::{
    database::{Connection, DbHelper},
    domain::{EUser, Entity, RAlphabet},
};

use super::{BaseQuerySearch, BaseRepository};

#[derive(Default)]
pub struct AlphabetRefRepository {
    connection: Arc<Connection>,
}

impl AlphabetRefRepository {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }
}

impl BaseRepository<RAlphabet> for AlphabetRefRepository {
    fn get_connection(&self) -> &Connection {
        &self.connection
    }
}
impl BaseQuerySearch<RAlphabet> for AlphabetRefRepository {}

impl AlphabetRefRepository {
    pub async fn get_ref(&self, user_id: &str) -> Option<RAlphabet> {
        let query = format!(
            "
SELECT id, title,
(
SELECT id, letter, completed,
(
SELECT id, idea, description, (
        SELECT (
            SELECT * 
            FROM tags
            WHERE id = $parent.tag_id
        ) as tags
        FROM date_idea_tags
        WHERE date_idea_id = $parent.id
    ).tags[0] as tags
FROM date_ideas
WHERE id = $parent.date_idea_id
)[0] as date_idea
FROM user_dates
WHERE alphabet_id = $parent.id
) as user_dates
FROM alphabets
WHERE id = {}
",
            DbHelper::id_to_thing(RAlphabet::get_table_name(), user_id)
        );

        self.query_search(query).await.pop()
    }

    pub async fn find_user_id_refs(&self, user_id: &str) -> Vec<RAlphabet> {
        let query = format!(
            "
SELECT id, title,
(
SELECT id, letter, completed,
(
SELECT id, idea, description, (
        SELECT (
            SELECT * 
            FROM tags
            WHERE id = $parent.tag_id
        ) as tags
        FROM date_idea_tags
        WHERE date_idea_id = $parent.id
    ).tags[0] as tags
FROM date_ideas
WHERE id = $parent.date_idea_id
)[0] as date_idea
FROM user_dates
WHERE alphabet_id = $parent.id
) as user_dates
FROM alphabets
WHERE user_id = {}
",
            DbHelper::id_to_thing(EUser::get_table_name(), user_id)
        );

        self.query_search(query.to_owned()).await
    }
}
