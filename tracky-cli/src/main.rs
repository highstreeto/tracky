use colored;
use colored::*;
use std::{io, io::Write, path::PathBuf};
use tracky::TimeTracker;
use tracky_cli::{handle_repl, REPLAction};

fn main() {
    let command_prefix = "> ";
    let error_prefix = "⛔  error:".red().bold();
    let path = default_path();
    let path_str = path.to_str().expect("Not a valid Unicode path!");

    // println!(
    //     "current dir: {}",
    //     env::current_dir()
    //         .map_err(|err| err.to_string())?
    //         .iter()
    //         .last()
    //         .expect("No last path element")
    //         .to_str()
    //         .expect("No unicode path!") // TODO: Use CamelCase for str
    // );

    println!("Hello and Welcome to {}!", "Tracky".blue().bold());
    println!("Loading from {}...", path_str);
    let mut tracker = TimeTracker::load(&path).unwrap_or_else(|_| {
        println!(" Creating empty tracker");
        TimeTracker::new()
    });

    loop {
        let mut command = String::new();

        print!("{}", command_prefix);
        io::stdout().flush().expect("Could not write to stdout!"); // println would panic before

        let line = match io::stdin().read_line(&mut command) {
            Ok(_) => command.trim(),
            Err(msg) => {
                println!("{} Could not read command! {}", error_prefix, msg);
                break;
            }
        };

        match handle_repl(&mut tracker, line) {
            Ok(REPLAction::Continue) => {}
            Ok(REPLAction::Quit) => {
                println!("Saving to {}...", path_str);
                tracker.save(&path).unwrap(); // TODO Decide what to do
                println!("Bye!");
                break;
            }
            Err(msg) => println!("{} {}", error_prefix, msg),
        }
    }
}

fn default_path() -> PathBuf {
    if let Some(mut file) = dirs::home_dir() {
        file.push("tracky.json");
        file
    } else {
        PathBuf::from("tracky.json")
    }
}
