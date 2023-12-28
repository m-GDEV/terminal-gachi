// Modules made for this project
pub mod customize;
pub mod status;
pub mod structure;

// System imports
use std::{env, process};

// Other imports
// For local crate imports, they aren't strictly necessary
// but they allow you to access them directly. eg:
// Without import: structure::Tamogachi
// With import: Tamogachi
use crate::status::print_statusline;
use crate::structure::{read_tamogachi, write_tamogachi, Tamogachi};

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
