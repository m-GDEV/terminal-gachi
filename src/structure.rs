use serde::{Deserialize, Serialize};
use std::fs;

// Struct defs
#[derive(Serialize, Deserialize, Debug)]
pub struct Tamogachi {
    pub name: String,
    pub birth: i32,
    pub level: i32,
    pub owner: String,
    pub personality: String,
    pub colour: String,
    pub breed: String,
    pub health: String,
    pub hunger: i32,
    pub sleeping: bool,
    pub sleep_left: i32,
    pub energy: i32,
    pub commands_run_per_day: Vec<CommandsPerDay>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandsPerDay {
    pub day: String,
    pub commands_run: i32,
}

pub fn read_tamogachi(path: &str) -> Tamogachi {
    // The so both read_to_string and from_str can "throw exceptions"
    // Because of this i added a ".expect", this essentially
    // tells the program to try and execute that funtion
    // and if it can't then to "throw an exception"
    // and exit the program. This is called a "panic"

    // Also, the way expect works is that it handles
    // Result enums. These enums have two possibilities:
    // a generic type and an error. If what the
    // functions mentioned above return a Result
    // that has error "selected" it will print
    // the message and exit. Otherwise it will
    // just return the generic type.
    // This essentially allows for me to assume
    // what the functions return is the type i want

    // Read json from file
    let data: String = fs::read_to_string(path).expect("Error reading file!");

    // Convert data to &str since serde wants that
    let s: &str = data.as_str();

    // Parse the string of data into a Tamogachi object
    let obj: Tamogachi = serde_json::from_str(s).expect("Cannot convert file to struct!");
    return obj;
}

pub fn write_tamogachi(obj: Tamogachi, path: &str) {
    let json = serde_json::to_string(&obj).expect("Cannot convert struct to json!");

    fs::write(path, json).expect("Cannot save file!");
}
