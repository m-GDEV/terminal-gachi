use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
// The numbers don't need to be signed but i thought
// I'd leave in the functionality in case i wanted
// to make them negative for some reason in the future
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
    // Level": 0,
    // "Owner": "Mario",
    // "Personality": "energetic",
    // "colour": "green",
    // "breed": "dinosaur",
    // "health": "great shape",
    // "hunger": 0,
    // "sleeping": false,
    // "sleep_left": 0,
    // "energy": 0,
    // "commandsRunPerDay": [
    //     "19122023": 0
    // ]
}

fn main() {
    let mut x = 50;

    println!("{}", x);
}
