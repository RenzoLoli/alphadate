mod auth_controller;
mod user_controller;
mod token_controller;
mod tag_controller;
mod date_idea_controller;

pub use auth_controller::config as auth_config;
pub use user_controller::config as user_config;
pub use token_controller::config as token_config;
pub use tag_controller::config as tag_config;
pub use date_idea_controller::config as date_idea_config;
