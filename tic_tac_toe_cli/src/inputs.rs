use std::io;

pub fn user_entered_yes(message: &str) -> bool {
    loop {
        let text = message.to_owned();
        let answer = read_user_text((text + " (y/N)").as_str(), true);
        if answer.len() == 0 {
            return false;
        }
        if answer.to_lowercase() == "y" {
            return true;
        }
        if answer.to_lowercase() == "n" {
            return false;
        }
        println!("Wrong answer format, please use 'Y' or 'N'.");
    }
}

pub fn read_user_text(message: &str, remove_trailing_line_feed: bool) -> String {
    println!("{}", message);

    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_n) => {
            if !remove_trailing_line_feed
                || user_input.len() == 0
                || user_input.chars().last().unwrap() != '\n'
            {
                user_input
            } else {
                user_input.pop();
                user_input
            }
        }
        Err(_e) => "".to_string(),
    }
}
