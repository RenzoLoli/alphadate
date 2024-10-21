#[derive(Debug)]
pub struct DateIdeaUpdateCommand {
    pub id: String,
    pub idea: Option<String>,
    pub description: Option<String>,
}

impl DateIdeaUpdateCommand {
    pub fn need_changes(&self) -> bool {
        self.idea.is_some() || self.description.is_some()
    }
}
