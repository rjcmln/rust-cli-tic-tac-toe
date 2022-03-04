use crate::types::board_dimension::{
    get_all_as_vec_str as get_all_board_dimensions, get_default as get_default_board_dimension,
};
use crate::types::player_mark::{
    get_all_as_vec_str as get_all_player_marks, get_default as get_default_player_mark,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SettingRule {
    pub name: String,
    pub allowed_values: String,
    pub default_value: String,
}

// NOTE 05: JSON representation for Front End to fill controls
pub fn get_game_settings_rules() -> String {
    let rules = vec![
        SettingRule {
            name: String::from("First Player"),
            allowed_values: get_all_player_marks().join(","),
            default_value: get_default_player_mark().to_string(),
        },
        SettingRule {
            name: String::from("Board Size"),
            allowed_values: get_all_board_dimensions().join(","),
            default_value: get_default_board_dimension().get_value().to_string(),
        },
    ];

    serde_json::to_string(&rules).unwrap()
}

#[test]
fn test_get_game_settings_rules() {
    let expected = serde_json::to_string(&vec![
        SettingRule {
            name: String::from("First Player"),
            allowed_values: String::from("X,O"),
            default_value: String::from("X"),
        },
        SettingRule {
            name: String::from("Board Size"),
            allowed_values: String::from("3,4,5"),
            default_value: String::from("3"),
        },
    ])
    .unwrap();

    assert_eq!(expected, get_game_settings_rules());
}
