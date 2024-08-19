use serde::{Deserialize, Serialize};

use super::{IdObject, Tag};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TagDto {
    pub id: Option<IdObject>,
    pub name: String,
}

impl From<&Tag> for TagDto {
    fn from(tag: &Tag) -> Self {
        TagDto {
            id: None,
            name: tag.name().to_string(),
        }
    }
}

impl From<&TagDto> for Tag {
    fn from(dto: &TagDto) -> Self {
        let mut n_tag = Tag::new(&dto.name);

        n_tag.set_id(&dto.id.clone().unwrap_or_default().id());

        n_tag
    }
}
