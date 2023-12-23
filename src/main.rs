// System imports
use std::env;
use std::fs;

// Serde imports
use serde::{Deserialize, Serialize};
use serde_json::Result;

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

fn main() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data: String =
        fs::read_to_string("./test-data.json").expect("Should have been able to read the file");

    let s: &str = data.as_str();
    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Tamogachi = serde_json::from_str(s)?;

    // Do things just like with any other Rust data structure.
    println!("{:?}", p.name);

    Ok(())
}
