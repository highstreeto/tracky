pub enum REPLAction {
    Continue,
    Quit,
}

pub fn handle_command(cmd: &str) -> Result<REPLAction, String> {
    match cmd {
        cmd if cmd.starts_with("add") => {
            for part in cmd.split_whitespace() {
                println!("{}", part);
            }
            Ok(REPLAction::Continue)
        },
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