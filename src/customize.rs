use std::io;

// Other imports
use crate::structure::Tamogachi;

fn change_name(obj: &mut Tamogachi) {
    let mut new_name = String::new();
    io::stdin()
        .read_line(&mut new_name)
        .expect("Cannot get user input!");
    obj.name = new_name;
}

fn create_tamogachi() {
    // make sure this one's name is not the same as
    // the current tamogachi or of other tamogachis already made
    // get name, age, owner, colour, breed from user
    // maybe not colour or breed since those depend on
    // match statements
    return ();
}

fn change_current_tamogachi() {
    // folder/file structure:
    // ~/.config/terminal-gachi.json || current tamogachi
    // ~/.config/Yoshi.json || Yoshi is a tamogachi not in use right now

    // so first we need the current tamogachi and then name of the
    // tamogachi to switch to
    // then move terminal-gachi.json to [current-tamogachi-name].json
    // and move [new-tamogachi-name].json to terminal-gachi.json
    return ();
}
