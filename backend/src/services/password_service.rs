pub struct PasswordService;

impl PasswordService {
    pub fn validate(password: &String, saved: &String) -> bool {
        if password.is_empty() || saved.is_empty() {
            return false;
        }

        *password == PasswordService::decrypt(saved)
    }

    pub fn encrypt(password: &String) -> String {
        String::from(password)
    }

    fn decrypt(password: &String) -> String {
        String::from(password)
    }
}
