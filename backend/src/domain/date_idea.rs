use serde::{Deserialize, Serialize};

use super::DateIdeaUpdate;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct DateIdea {
    id: String,
    idea: String,
    description: String,
}

impl DateIdea {
    pub fn new(idea: &str, description: &str) -> Self {
        Self {
            id: "".to_owned(),
            idea: idea.to_string(),
            description: description.to_string(),
        }
    }

    // GETTERS
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn idea(&self) -> &str {
        &self.idea
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    // SETTERS
    pub fn set_id(&mut self, id: &str) {
        self.id = String::from(id);
    }

    pub fn set_idea(&mut self, idea: &str) {
        self.idea = String::from(idea);
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = String::from(description);
    }

    // DOMAIN RULES

    pub fn update(&mut self, options: &DateIdeaUpdate) {
        if let Some(idea) = options.idea.clone() {
            self.idea.clone_from(&idea);
        }

        if let Some(description) = options.description.clone() {
            self.description.clone_from(&description);
        }
    }
}
