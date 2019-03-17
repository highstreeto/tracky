extern crate colored;

use colored::*;
use std::io::{self, Write};

fn main() {
    let command_prefix = "> ";
    let error_prefix = "error:".red().bold();

    println!("Hello and Welcome to {}!", "Tracky".blue().bold());

    loop {
        let mut command = String::new();

        print!("{}", command_prefix);
        io::stdout()
            .flush()
            .expect("Could not write to stdout!"); // println would panic before

        let command = match io::stdin().read_line(&mut command) {
            Ok(_) => command.trim(),
            Err(msg) => {
                println!("{} Could not read command! {}", error_prefix, msg);
                break;
            }
        };
        match handle_command(command) {
            Ok(REPLAction::Continue) => {},
            Ok(REPLAction::Quit) => {
                println!("quitting...");
                break;
            }
            Err(msg) => println!("{} {}", error_prefix, msg),
        }
    }
}

enum REPLAction {
    Continue,
    Quit,
}

fn handle_command(cmd: &str) -> Result<REPLAction, String> {
    match cmd {
        "help" => {
            println!("Available commands:");
            println!("  help    Displays this help text");
            println!("  quit    To quit Tracky");
            Ok(REPLAction::Continue)
        }
        "quit" => Ok(REPLAction::Quit),
        _ => Err(format!("Unknwon command '{}'!", cmd)),
    }
}
