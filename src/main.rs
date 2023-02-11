#![allow(non_snake_case)]

use tweaks::tempclear;

mod utils {
    pub mod logo;
    pub mod options;
    pub mod input;
}

mod tweaks {
    pub mod tempclear;
}

fn main() {
    clearscreen::clear().expect("failed to clear screen");
    utils::logo::logo();
    utils::options::createOptions(vec![
        utils::options::OptionConfig {
            index: 1,
            text: "Clear Temp Folder".to_string(),
            description: "Clears your temp folder, saves alot of space and might help with lag.".to_string(),
            run: || tempclear::main(),
        },
        utils::options::OptionConfig {
            index: 2,
            text: "Test2".to_string(),
            description: "Test2".to_string(),
            run: || println!("Running option 2"),
        }
    ]);
}
