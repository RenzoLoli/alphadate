#[derive(Debug)]
pub struct AlphabetUpdateCommand {
    pub id: String,
    pub title: Option<String>,
}

impl AlphabetUpdateCommand {
    pub fn need_changes(&self) -> bool {
        self.title.is_some()
    }
}
