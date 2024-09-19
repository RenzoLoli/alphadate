mod alphabet_controller;
mod auth_controller;
mod date_idea_controller;
pub mod resources;
mod tag_controller;
mod token_controller;
mod user_controller;

pub use alphabet_controller::config as alphabet_config;
pub use auth_controller::config as auth_config;
pub use date_idea_controller::config as date_idea_config;
pub use tag_controller::config as tag_config;
pub use token_controller::config as token_config;
pub use user_controller::config as user_config;
