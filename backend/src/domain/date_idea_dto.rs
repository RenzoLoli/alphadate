use serde::{Deserialize, Serialize};

use super::{DateIdea, IdObject};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct DateIdeaDto {
    pub id: Option<IdObject>,
    pub idea: String,
    pub description: String,
}

impl From<&DateIdeaDto> for DateIdea {
    fn from(dto: &DateIdeaDto) -> Self {
        let mut n_idea = DateIdea::new(&dto.idea, &dto.description);

        n_idea.set_id(&dto.id.clone().unwrap_or_default().id());

        n_idea
    }
}
