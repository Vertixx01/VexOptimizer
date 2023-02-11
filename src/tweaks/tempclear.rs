extern crate colorful;

use colorful::Color;
use colorful::Colorful;

pub fn main() {
    for folders in std::fs::read_dir(std::env::temp_dir()).unwrap() {
        let path = folders.unwrap().path();
        std::fs::remove_dir_all(path.clone()).unwrap_or_else(|_| {println!("{} {}", "Failed to delete".color(Color::Red), path.file_name().unwrap().to_str().unwrap().color(Color::White))});
    }
    println!("{} {}", "Successfully deleted all files in temp folder".color(Color::Green), "Press enter to continue".color(Color::White));
}