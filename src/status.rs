use chrono::{Datelike, Local};
use colored::Colorize;

// Other imports
use crate::structure::{CommandsPerDay, Tamogachi};

fn return_colored_text(color: &str, text: &String) -> colored::ColoredString {
    match color {
        "red" => return text.red(),
        "green" => return text.green(),
        _ => return text.white(),
    }
}

fn return_emoji_from_breed(breed: &str) -> &str {
    match breed {
        "dinosaur" => return &"🦖",
        _ => return breed,
    }
}

pub fn print_statusline(obj: &mut Tamogachi) {
    let mut sleeping_status: &str = "";
    if obj.sleeping == true {
        sleeping_status = "😴";
    }

    let now = Local::now();
    let formatted_date = format!("{}{}{}", now.day(), now.month(), now.year());
    let mut found_date = false;
    // initializing this at 1 means we don't have to change it
    // if we didn't find the date
    let mut commands_run_today = 1;
    // println!("{} {} {}", now.day(), now.month(), now.year());

    for i in &mut obj.commands_run_per_day {
        // println!("{:?}", i);
        if i.day == formatted_date {
            i.commands_run += 1;
            commands_run_today = i.commands_run;
            found_date = true;
            break;
        }
    }

    if !found_date {
        obj.commands_run_per_day.push(CommandsPerDay {
            day: formatted_date,
            commands_run: 1,
        })
    }

    let age = now.year() - obj.birth;

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

    // println!("{:?}", obj.commands_run_per_day[0].day);
    // 🦖 Musa (23) 🎉, Mario's energetic dinosaur. Green, great shape 💪, no hunger 🍔, not sleeping 😴, 0 energy ⚡. Ran 0 commands on 19/12/2023 📅.
}
