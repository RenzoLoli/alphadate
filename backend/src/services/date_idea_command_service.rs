use std::sync::Arc;

use crate::{
    domain::{
        DateIdeaAddTagCommand, DateIdeaCreateCommand, DateIdeaDeleteCommand,
        DateIdeaRemoveTagCommand, DateIdeaUpdateCommand, EDateIdea, EDateIdeaTag,
    },
    repository::{BaseRepository, DateIdeaRepository, DateIdeaTagRepository, TagRepository},
};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct DateIdeaCommandService {
    tag_repository: Arc<TagRepository>,
    date_idea_repository: Arc<DateIdeaRepository>,
    date_idea_tag_repository: Arc<DateIdeaTagRepository>,
}

impl DateIdeaCommandService {
    pub fn new(
        date_idea_repository: Arc<DateIdeaRepository>,
        date_idea_tag_repository: Arc<DateIdeaTagRepository>,
        tag_repository: Arc<TagRepository>,
    ) -> Self {
        Self {
            tag_repository,
            date_idea_repository,
            date_idea_tag_repository,
        }
    }
}

impl ServiceHandlerTrait<DateIdeaUpdateCommand, EDateIdea> for DateIdeaCommandService {
    async fn handle(&self, command: DateIdeaUpdateCommand) -> Result<EDateIdea, String> {
        let mut date_idea = match self
            .date_idea_repository
            .find_by_id(command.id.as_str())
            .await
        {
            Some(date_idea) => date_idea,
            None => return Err("Date Idea not found".to_owned()),
        };

        date_idea.update(command);

        match self.date_idea_repository.update(date_idea).await {
            Some(date_idea) => Ok(date_idea),
            None => Err("Date Idea cannot be updated".to_owned()),
        }
    }
}

impl ServiceHandlerTrait<DateIdeaDeleteCommand, EDateIdea> for DateIdeaCommandService {
    async fn handle(&self, command: DateIdeaDeleteCommand) -> Result<EDateIdea, String> {
        let date_idea = match self.date_idea_repository.find_by_id(&command.id).await {
            Some(date_idea) => date_idea,
            None => return Err("Date Idea not found".to_owned()),
        };

        let date_idea_tags = self.date_idea_tag_repository.get_all().await;

        let date_idea_tag_refs = date_idea_tags
            .into_iter()
            .filter(|idea_tag| idea_tag.date_idea_id == date_idea.id)
            .collect::<Vec<EDateIdeaTag>>();

        for date_idea_tag_ref in date_idea_tag_refs {
            self.date_idea_tag_repository
                .delete(&date_idea_tag_ref.id.to_string())
                .await;
        }

        let _ = match self.date_idea_repository.delete(&command.id).await {
            Some(date_idea) => Ok(date_idea),
            None => Err("Date Idea cannot be deleted".to_owned()),
        };

        Ok(date_idea)
    }
}

impl ServiceHandlerTrait<DateIdeaCreateCommand, EDateIdea> for DateIdeaCommandService {
    async fn handle(&self, command: DateIdeaCreateCommand) -> Result<EDateIdea, String> {
        let finded_idea = self.date_idea_repository.find_by_idea(&command.idea).await;

        if !finded_idea.is_empty() {
            return Err("Idea is already exists".to_owned());
        }

        let date_idea_ent = EDateIdea::from(command);

        let date_idea = self.date_idea_repository.create(date_idea_ent).await;

        match date_idea {
            Some(date_idea) => Ok(date_idea),
            None => Err("Date Idea cannot be created".to_owned()),
        }
    }
}

impl ServiceHandlerTrait<DateIdeaAddTagCommand, EDateIdea> for DateIdeaCommandService {
    async fn handle(&self, command: DateIdeaAddTagCommand) -> Result<EDateIdea, String> {
        let date_idea = match self
            .date_idea_repository
            .find_by_id(&command.date_idea_id)
            .await
        {
            Some(date_idea) => date_idea,
            None => return Err("Date Idea not found".to_owned()),
        };

        let _ = match self.tag_repository.find_by_id(&command.tag_id).await {
            Some(tag) => tag,
            None => return Err("Tag not found".to_owned()),
        };

        let date_idea_tag = match self
            .date_idea_tag_repository
            .find_by_date_idea_and_tag_id(&command.date_idea_id, &command.tag_id)
            .await
        {
            Some(_) => return Err("Reference already exists".to_owned()),
            None => EDateIdeaTag::from(command),
        };

        match self.date_idea_tag_repository.create(date_idea_tag).await {
            Some(_) => Ok(date_idea),
            None => Err("Reference cannot be created".to_owned()),
        }
    }
}

impl ServiceHandlerTrait<DateIdeaRemoveTagCommand, EDateIdea> for DateIdeaCommandService {
    async fn handle(&self, command: DateIdeaRemoveTagCommand) -> Result<EDateIdea, String> {
        let date_idea = match self
            .date_idea_repository
            .find_by_id(&command.date_idea_id)
            .await
        {
            Some(date_idea) => date_idea,
            None => return Err("Date Idea not found".to_owned()),
        };

        let _ = match self.tag_repository.find_by_id(&command.tag_id).await {
            Some(tag) => tag,
            None => return Err("Tag not found".to_owned()),
        };

        let date_idea_tag = match self
            .date_idea_tag_repository
            .find_by_date_idea_and_tag_id(&command.date_idea_id, &command.tag_id)
            .await
        {
            Some(date_idea_tag) => date_idea_tag,
            None => return Err("Reference not found".to_owned()),
        };

        match self
            .date_idea_tag_repository
            .delete(&date_idea_tag.id.to_string())
            .await
        {
            Some(_) => Ok(date_idea),
            None => Err("Reference cannot be deleted".to_owned()),
        }
    }
}
