extern crate colorful;

use colorful::Color;
use colorful::Colorful;

pub fn logo() {
    println!("      {}", "____   ____             ".gradient(Color::Green));
    println!("      {}", "\\   \\ /   /____ ___  ___".gradient(Color::Green));
    println!("      {}", " \\   Y   // __ \\  \\/  /".gradient(Color::Green));
    println!("      {}", "  \\     /\\  ___/ >    < ".gradient(Color::Green));
    println!("      {}", "   \\___/  \\___  >__/\\_ \\".gradient(Color::Green));
    println!("      {}", "              \\/      \\/".gradient(Color::Green));
}
