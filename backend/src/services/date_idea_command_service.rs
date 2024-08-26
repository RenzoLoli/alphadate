use std::sync::Arc;

use crate::{
    domain::{
        DateIdeaCreateCommand, DateIdeaDeleteCommand, DateIdeaUpdateCommand, EDateIdea,
        EDateIdeaTag,
    },
    repository::{BaseRepository, DateIdeaRepository, DateIdeaTagRepository},
};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct DateIdeaCommandService {
    date_idea_repository: Arc<DateIdeaRepository>,
    date_idea_tag_repository: Arc<DateIdeaTagRepository>,
}

impl DateIdeaCommandService {
    pub fn new(
        date_idea_repository: Arc<DateIdeaRepository>,
        date_idea_tag_repository: Arc<DateIdeaTagRepository>,
    ) -> Self {
        Self {
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
            .filter(|idea_tag| idea_tag.id.to_string() == command.id)
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
