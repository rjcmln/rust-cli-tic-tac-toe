use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GameSettingRule {
    pub name: String,
    pub allowed_values: String,
    pub default_value: String,
}

pub fn deserialize_game_settings_rules(s: &str) -> Vec<GameSettingRule> {
    serde_json::from_str(s).unwrap()
}
