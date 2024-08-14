use uuid::Uuid;

use crate::domain::{DateIdea, DateIdeaUpdate, Tag};

use super::TagService;

pub struct DateIdeaService;

static mut DATE_IDEAS: Vec<DateIdea> = vec![];

impl DateIdeaService {
    pub fn get_all() -> Vec<DateIdea> {
        unsafe { DATE_IDEAS.clone() }
    }

    pub fn find_by_id(id: &str) -> Option<DateIdea> {
        unsafe {
            DATE_IDEAS
                .clone()
                .into_iter()
                .find(|date_idea: &'_ DateIdea| date_idea.id() == id)
        }
    }

    pub fn find_by_name(idea: &str) -> Option<DateIdea> {
        unsafe {
            DATE_IDEAS
                .clone()
                .into_iter()
                .find(|date_idea: &'_ DateIdea| date_idea.idea() == idea)
        }
    }

    pub fn create_date_idea(date_idea: &DateIdea) -> Result<DateIdea, String> {
        let mut n_date_idea = date_idea.clone();

        if DateIdeaService::find_by_name(&date_idea.idea()).is_some() {
            return Err("Date Idea already exist".to_owned());
        }

        n_date_idea.set_id(Uuid::new_v4().to_string().as_str());

        unsafe { DATE_IDEAS.push(n_date_idea.clone()) }
        Ok(n_date_idea)
    }

    pub fn update_date_idea(id: &str, options: &DateIdeaUpdate) -> Result<DateIdea, String> {
        let finded_date_idea = unsafe {
            DATE_IDEAS
                .iter_mut()
                .find(|date_idea: &&mut DateIdea| date_idea.id() == id)
        };

        let Some(date_idea) = finded_date_idea else {
            return Err("Date Idea not exist".to_owned());
        };

        date_idea.update(options);

        Ok(date_idea.clone())
    }

    pub fn delete_date_idea(id: &str) -> Result<DateIdea, String> {
        unsafe {
            DATE_IDEAS
                .iter()
                .position(|date_idea: &DateIdea| date_idea.id() == id)
                .map(|pos| DATE_IDEAS.remove(pos))
                .ok_or("Date Idea not Exists".to_owned())
        }
    }
}
