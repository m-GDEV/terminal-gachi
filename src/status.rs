// Foreign imports
use chrono::{Datelike, Local};
use colored::Colorize;
use std::collections::hash_map::Entry;

// Local imports
use crate::structure::{CommandsPerDay, Tamogachi};

fn return_colored_text(color: &str, text: &String) -> colored::ColoredString {
    // Only allowed colors are the default colors in the 'colored' crate
    match color {
        "black" => text.black(),
        "red" => text.red(),
        "green" => text.green(),
        "yellow" => text.yellow(),
        "blue" => text.blue(),
        "magenta" => text.magenta(),
        "cyan" => text.cyan(),
        "white" => text.white(),
        _ => return text.white(),
    }
}

fn return_emoji_from_breed(breed: &str) -> &str {
    // ChatGPT came up with these lol
    match breed {
        "cat" => return &"🐱",
        "dog" => return &"🐶",
        "bunny" => return &"🐰",
        "dinosaur" => return &"🦕",
        "dragon" => return &"🐉",
        "unicorn" => return &"🦄",
        "penguin" => return &"🐧",
        "fox" => return &"🦊",
        "owl" => return &"🦉",
        "robot" => return &"🤖",
        _ => return breed,
    }
}

pub fn statusline(obj: &mut Tamogachi, modify: bool, print: bool) {
    let mut sleeping_status: &str = "";
    if obj.sleeping == true {
        sleeping_status = "😴";
    }

    let now = Local::now();
    let age = now.year() - obj.birth;
    let formatted_date = format!("{}{}{}", now.day(), now.month(), now.year());
    let mut commands_run_today = if modify == true { 1 } else { 0 };

    // Cool code provided by Phind, I don't fully get how it works
    match obj.commands_run_per_day.entry(formatted_date) {
        Entry::Occupied(mut entry) => {
            *entry.get_mut() += 1;
            commands_run_today = *entry.get_mut();
        }
        Entry::Vacant(entry) => {
            entry.insert(commands_run_today);
        }
    }

    if print {
        println!(
            // "Yoshi (23) is a LVL 0 dinosaur, energetic, and is owned by Mario\t0",
            "{}{} {} ({}) {} {}, {}{} {}\t\t⚡ {}",
            return_emoji_from_breed(&obj.breed),
            sleeping_status,
            return_colored_text(&obj.colour, &obj.name).bold(),
            age.to_string().truecolor(173, 173, 173).bold(),
            "is".truecolor(138, 43, 226).bold(),
            format!("LVL {}", obj.level.to_string()).yellow().bold(),
            obj.personality.truecolor(248, 131, 121).bold(),
            ", and owned by".truecolor(138, 43, 226).bold(),
            obj.owner.truecolor(65, 105, 225).bold(),
            commands_run_today
                .to_string()
                .truecolor(255, 0, 0)
                .bold()
                .italic()
        );
    }

    // println!("{:?}", obj.commands_run_per_day[0].day);
    // 🦖 Musa (23) 🎉, Mario's energetic dinosaur. Green, great shape 💪, no hunger 🍔, not sleeping 😴, 0 energy ⚡. Ran 0 commands on 19/12/2023 📅.
}
