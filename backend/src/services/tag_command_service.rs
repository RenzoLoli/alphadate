use std::sync::Arc;

use crate::{
    domain::{ETag, TagDeleteCommand, TagUpdateCommand},
    repository::{BaseRepository, DateIdeaTagRepository, TagRepository},
};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct TagCommandService {
    tag_repository: Arc<TagRepository>,
    date_idea_tag_repository: Arc<DateIdeaTagRepository>,
}

impl TagCommandService {
    pub fn new(
        tag_repository: Arc<TagRepository>,
        date_idea_tag_repository: Arc<DateIdeaTagRepository>,
    ) -> Self {
        Self {
            tag_repository,
            date_idea_tag_repository,
        }
    }
}

impl ServiceHandlerTrait<TagUpdateCommand, ETag> for TagCommandService {
    async fn handle(&self, input: TagUpdateCommand) -> Result<ETag, String> {
        let mut tag = match self.tag_repository.find_by_id(input.id.as_str()).await {
            Some(tag) => tag,
            None => return Err("Tag not found".to_owned()),
        };

        tag.update(input);

        match self.tag_repository.update(tag).await {
            Some(tag) => Ok(tag),
            None => Err("Tag cannot be updated".to_owned()),
        }
    }
}

impl ServiceHandlerTrait<TagDeleteCommand, ETag> for TagCommandService {
    async fn handle(&self, input: TagDeleteCommand) -> Result<ETag, String> {
        let _ = match self.tag_repository.find_by_id(input.id.as_str()).await {
            Some(tag) => tag.to_owned(),
            None => return Err("Tag not found".to_owned()),
        };

        let date_idea_tags = self
            .date_idea_tag_repository
            .find_by_tag_id(input.id.as_str())
            .await;

        let tag = match self.tag_repository.delete(input.id.as_str()).await {
            Some(tag) => Ok(tag.to_owned()),
            None => Err("Tag cannot be deleted".to_owned()),
        };

        for date_idea_tag in date_idea_tags {
            let id = date_idea_tag.id.to_string();
            self.date_idea_tag_repository
                .delete(id.as_str())
                .await
                .map(|input| {
                    log::info!("Reference Tag - Date Idea {} deleted", input.id.to_string());
                    Some(input)
                });
        }

        tag
    }
}
