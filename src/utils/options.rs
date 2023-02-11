extern crate colorful;

use colorful::Color;
use colorful::Colorful;
use super::input::input;

#[derive(Clone)]
pub struct OptionConfig {
    pub index: i64,
    pub text: String,
    pub description: String,
    pub run: fn(),
}

pub fn createOptions(options: Vec<OptionConfig>) {
    for option in options.clone() {
        format_option(option.index, option.text);
    }
    println!();
    let input = input("   Select an option: ");
    let input = input.parse::<i64>().unwrap();
    for option in options {
        if option.index == input {
            (option.run)();
        }
    }
}

fn format_option(index: i64, text: String) {
    print!("     {}{}{} {} {}", "[".color(Color::Green), index.to_string().color(Color::White), "]".color(Color::Green), " - ".color(Color::White), text.color(Color::White));
    println!();
}