use std::time;

pub enum REPLAction {
    Continue,
    Quit,
}

pub struct Manager {
    projects: Vec<Project>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager { projects: vec![] }
    }

    fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }

    fn find_project(&self, name: &str) -> Option<&Project> {
        self.projects
            .iter()
            .filter(|proj| proj.name() == name)
            .next()
    }

    pub fn handle(&mut self, line: &str) -> Result<REPLAction, String> {
        let mut args = line.split_whitespace();
        let cmd = args.next().ok_or("Enter a command - use help for a list of commands")?;

        match cmd {
            "add" => {
                let name = args.next().ok_or("New project name required!")?;
                let project = Project::new(name);
                println!("Added new project {}", project);
                self.add_project(project);
                Ok(REPLAction::Continue)
            }
            "list" => {
                if !self.projects.is_empty() {
                    println!("All projects:");
                    for project in self.projects.iter() {
                        println!(" - {}", project);
                    }
                } else {
                    println!("No projects found - use add to change this!");
                }
                Ok(REPLAction::Continue)
            }
            "help" => {
                println!("Available commands:");
                println!("  add <name>    Add a new project");
                println!("  list          List all projects");
                println!("  help          Displays this help text");
                println!("  quit          To quit Tracky");
                Ok(REPLAction::Continue)
            }
            "quit" => Ok(REPLAction::Quit),
            _ => Err(format!("Unknwon command '{}'!", cmd)),
        }
    }
}

pub struct Project {
    name: String,
    entries: Vec<Entry>,
}

impl Project {
    fn new(name: &str) -> Project {
        Project {
            name: String::from(name),
            entries: vec![],
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn rename(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct Entry {
    start: time::SystemTime,
    end: time::SystemTime,
}
