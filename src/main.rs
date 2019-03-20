extern crate colored;

use colored::*;
use tracky::*;
use std::io::{self, Write};

fn main() {
    let command_prefix = "> ";
    let error_prefix = "⛔  error:".red().bold();
    let mut manager = TimeTracker::new();
    println!("Hello and Welcome to {}!", "Tracky".blue().bold());

    loop {
        let mut command = String::new();

        print!("{}", command_prefix);
        io::stdout()
            .flush()
            .expect("Could not write to stdout!"); // println would panic before

        let line = match io::stdin().read_line(&mut command) {
            Ok(_) => command.trim(),
            Err(msg) => {
                println!("{} Could not read command! {}", error_prefix, msg);
                break;
            }
        };
        
        match manager.handle(line) {
            Ok(REPLAction::Continue) => {},
            Ok(REPLAction::Quit) => {
                println!("Bye!");
                break;
            }
            Err(msg) => println!("{} {}", error_prefix, msg),
        }
    }
}
