// System imports
use chrono::{Datelike, Local};
use std::{env, fs, io, process};

// Other imports
use colored::Colorize;
use serde::{Deserialize, Serialize};

// Struct defs
#[derive(Serialize, Deserialize, Debug)]
struct Tamogachi {
    name: String,
    birth: i32,
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

fn usage(program_name: &String) {
    eprintln!(
        "
USAGE:
\t{program_name} [OPTIONS]
OPTIONS:
\t-h | -help
\t-c | --create-tamogachi
\t-C | --change-name
\t-s | --switch-active-tamogachi
        "
    );
}

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

fn print_statusline(obj: &mut Tamogachi) {
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

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = &argv.len();

    // usage(&argv[0]);

    // println!("{}", &argv.len());

    let file_path = "./test-data.json";

    // Load data
    let mut obj: Tamogachi = read_tamogachi(file_path);

    // // Modify data
    // obj.name = String::from("Musa");

    // 3 Run conditions

    // print status line
    if argc == &1 {
        print_statusline(&mut obj);

        // println!("{:?}", obj);

        // invoke function that command line argument is
    } else if argc == &2 {
        if argv[1] == "-h" || argv[1] == "--help" {
            usage(&argv[0]);
        } else if argv[1] == "-c" || argv[1] == "--create-tamogachi" {
            // change nmae function
        } else if argv[1] == "-C" || argv[1] == "--change-name" {
            // change nmae function
        } else if argv[1] == "-s" || argv[1] == "--change-active-tamogachi" {
            // change nmae function
        } else {
            eprintln!("Invalid options provided!");
            usage(&argv[0]);
            process::exit(1);
        }
    } else {
        eprintln!("You can only run the program with one option!");
        usage(&argv[0]);
        process::exit(1);
    }

    // // Save data
    write_tamogachi(obj, file_path)
}
