#[derive(Debug)]
pub struct GetRandomDateIdeaQuery {
    pub alphabet_id: String,
    pub exclude_active: bool,
}
