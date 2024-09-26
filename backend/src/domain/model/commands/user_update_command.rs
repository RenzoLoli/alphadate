#[derive(Debug)]
pub struct UserUpdateCommand {
    pub id: String,
    pub username: Option<String>,
    pub couplename: Option<String>,
    pub anniversary: Option<String>,
    pub photo: Option<String>,
}

impl UserUpdateCommand {
    pub fn need_changes(&self) -> bool {
        self.username.is_some()
            || self.couplename.is_some()
            || self.anniversary.is_some()
            || self.photo.is_some()
    }
}
