// Modules made for this project
pub mod customize;
pub mod status;
pub mod structure;

use std::path::PathBuf;
// System imports
use std::{env, process};

// Other imports
// For local crate imports, they aren't strictly necessary
// but they allow you to access them directly. eg:
// Without import: structure::Tamogachi
// With import: Tamogachi
use crate::customize::{change_name, create_tamogachi};
use crate::status::statusline;
use crate::structure::{read_tamogachi, write_tamogachi, Tamogachi};

use dirs;

fn usage(program_name: &String) {
    eprintln!(
        "
USAGE:
\t{program_name} [OPTIONS]
OPTIONS:
\t-h | --help
\t-c | --create-tamogachi
\t-C | --change-name
\t-n | --no-command-run (this option is meant when running on shell startup)
\t-s | --switch-active-tamogachi
        "
    );
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = &argv.len();

    // usage(&argv[0]);

    // println!("{}", &argv.len());

    let config_location: String = String::from("/.config/terminal-gachi.json");
    let mut home = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    home.push_str(config_location.as_str());
    let file_path = home.as_str();
    // print!("full path {}", file_path);

    // Load data
    let mut obj: Tamogachi = read_tamogachi(file_path);

    // // Modify data
    // obj.name = String::from("Musa");

    // 3 Run conditions

    // print status line
    if argc == &1 {
        statusline(&mut obj, true, false);

        // println!("{:?}", obj);

        // invoke function that command line argument is
    } else if argc == &2 {
        if argv[1] == "-h" || argv[1] == "--help" {
            usage(&argv[0]);
        } else if argv[1] == "-c" || argv[1] == "--create-tamogachi" {
            // change nmae function
        } else if argv[1] == "-C" || argv[1] == "--change-name" {
            change_name(&mut obj);
        } else if argv[1] == "-s" || argv[1] == "--change-active-tamogachi" {
            // change nmae function
        } else if argv[1] == "-n" || argv[1] == "--no-command-run" {
            statusline(&mut obj, false, true)
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
