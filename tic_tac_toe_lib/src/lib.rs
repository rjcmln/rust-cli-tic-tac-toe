mod game;
mod game_state;
mod setting_rules;
mod settings;
mod state_calculation;
mod types;

// NOTE 08: All communication with the library goes through JSON, to mimic Front to Back End communication

// Main library public type
pub use game::Game;

// JSON representation for Front End to fill controls
pub use setting_rules::get_game_settings_rules;
