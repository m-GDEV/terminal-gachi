// System imports
use std::{env, fs};

// Serde imports
use serde::{Deserialize, Serialize};

// Struct defs
#[derive(Serialize, Deserialize, Debug)]
struct Tamogachi {
    name: String,
    age: i32,
    level: i32,
    owner: String,
    personality: String,
    colour: String,
    breed: String,
    health: String,
    hunger: i32,
    sleeping: bool,
    sleep_left: i32,
    energy: i32,
    commands_run_per_day: Vec<CommandsPerDay>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CommandsPerDay {
    day: String,
    commands_run: i32,
}

fn read_tamogachi(path: &str) -> Tamogachi {
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

fn write_tamogachi(obj: Tamogachi, path: &str) {
    let json = serde_json::to_string(&obj).expect("Cannot convert struct to json!");

    fs::write(path, json).expect("Cannot save file!");
}

fn usage() {
    println!(
        "
        usage: ./tamogachi-pro  
        "
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for i in args {
        println!("{}", i);
    }

    usage();

    // let file_path = "./test-data.json";
    //
    // // Load data
    // let mut obj: Tamogachi = read_tamogachi(file_path);
    //
    // // Modify data
    // obj.name = String::from("Musa");
    //
    // // Save data
    // write_tamogachi(obj, file_path)
}
