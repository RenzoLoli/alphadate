use uuid::Uuid;

use crate::domain::{Tag, TagUpdate};

pub struct TagService;

static mut TAGS: Vec<Tag> = vec![];

impl TagService {
    pub fn get_all() -> Vec<Tag> {
        unsafe { TAGS.clone() }
    }

    pub fn find_by_id(id: &str) -> Option<Tag> {
        unsafe { TAGS.clone().into_iter().find(|tag: &'_ Tag| tag.id() == id) }
    }

    pub fn find_by_name(name: &str) -> Option<Tag> {
        unsafe {
            TAGS.clone()
                .into_iter()
                .find(|tag: &'_ Tag| tag.name() == name)
        }
    }

    pub fn create_tag(tag: &Tag) -> Result<Tag, String> {
        let mut n_tag = tag.clone();

        if TagService::find_by_name(&tag.name()).is_some() {
            return Err("Tag already exist".to_owned());
        }

        n_tag.set_id(Uuid::new_v4().to_string().as_str());

        unsafe { TAGS.push(n_tag.clone()) }
        Ok(n_tag)
    }

    pub fn update_tag(id: &str, options: &TagUpdate) -> Result<Tag, String> {
        let finded_tag = unsafe { TAGS.iter_mut().find(|tag: &&mut Tag| tag.id() == id) };

        let Some(tag) = finded_tag else {
            return Err("Tag not exist".to_owned());
        };

        tag.update(options);

        Ok(tag.clone())
    }

    pub fn delete_tag(id: &str) -> Result<Tag, String> {
        unsafe {
            TAGS.iter()
                .position(|tag: &Tag| tag.id() == id)
                .map(|pos| TAGS.remove(pos))
                .ok_or("tag not Exists".to_owned())
        }
    }
}
