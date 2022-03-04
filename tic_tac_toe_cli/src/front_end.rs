use crate::game_settings_rules::{deserialize_game_settings_rules, GameSettingRule};
use crate::game_state::deserialize_game_state;
use crate::inputs::{read_user_text, user_entered_yes};
use crate::outputs::{print_board, print_result};
use tic_tac_toe_lib::get_game_settings_rules;
use tic_tac_toe_lib::Game;

pub fn start_game() {
    println!("--- Tic Tac Toe CLI Application ---");

    let game_settings_rules = deserialize_game_settings_rules(get_game_settings_rules().as_str());

    let mut previous_game_settings: Option<String> = None;

    loop {
        let mut game = init_game(&game_settings_rules, &mut previous_game_settings);

        loop {
            let state = deserialize_game_state(game.get_state().as_str());
            print_board(&state);

            if state.finished {
                print_result(&state.result);
                break;
            }
            play_move(&mut game, state.next_on_move.as_str());
        }

        if user_entered_yes("Play again?") == false {
            break;
        }
    }
}

fn init_game(
    game_settings_rules: &Vec<GameSettingRule>,
    previous_game_settings: &mut Option<String>,
) -> Game {
    loop {
        let ask_for_new_game_settings =
            previous_game_settings.is_none() || user_entered_yes("Use old Game Settings?") == false;

        if ask_for_new_game_settings {
            println!("Please enter the game settings. Rules are:");
            for rule in game_settings_rules {
                println!(
                    "  {}: Allowed Values: {}. Default Value: {}",
                    rule.name, rule.allowed_values, rule.default_value
                );
            }
            let mut user_text = read_user_text("  Please separate values with comma, for example: 'X,3'\n  Hit Enter for default values",true);

            if user_text.len() == 0 {
                let rule_first_player = game_settings_rules
                    .iter()
                    .find(|&rule| rule.name == String::from("First Player"))
                    .unwrap();
                let rule_board_size = game_settings_rules
                    .iter()
                    .find(|&rule| rule.name == String::from("Board Size"))
                    .unwrap();
                user_text = rule_first_player.default_value.clone()
                    + ","
                    + rule_board_size.default_value.as_str();
            }

            match Game::new(user_text.as_str()) {
                Ok(game) => {
                    *previous_game_settings = Some(String::from(user_text));
                    return game;
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        } else {
            let s = previous_game_settings.as_ref().unwrap().as_str();
            return Game::new(s).unwrap();
        }
    }
}

fn play_move(game: &mut Game, player_mark: &str) {
    loop {
        let message = String::from("Player ") + player_mark + " please enter the move";
        let user_text = read_user_text(message.as_str(), true);
        match game.play_move(user_text.as_str()) {
            Ok(()) => return,
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
