use std::sync::Arc;

use crate::{
    domain::{ETag, GetAllTagsQuery, GetTagByIdQuery},
    repository::{BaseTransactions, TagRepository},
};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct TagQueryService {
    tag_repository: Arc<TagRepository>,
}

impl TagQueryService {
    pub fn new(tag_repository: Arc<TagRepository>) -> Self {
        Self { tag_repository }
    }
}

impl ServiceHandlerTrait<GetAllTagsQuery, Vec<ETag>> for TagQueryService {
    async fn _handle(&self, _query: GetAllTagsQuery) -> Result<Vec<ETag>, String> {
        Ok(self.tag_repository.get_all().await)
    }
}

impl ServiceHandlerTrait<GetTagByIdQuery, ETag> for TagQueryService {
    async fn _handle(&self, query: GetTagByIdQuery) -> Result<ETag, String> {
        let finded = self.tag_repository.find_by_id(&query.id).await;

        match finded {
            Some(tag) => Ok(tag),
            None => Err("Tag not found".to_owned()),
        }
    }
}
