use colored::*;
use std::fmt;
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

    fn find_project_mut(&mut self, name: &str) -> Option<&mut Project> {
        self.projects
            .iter_mut()
            .filter(|proj| proj.name() == name)
            .next()
    }

    pub fn handle(&mut self, line: &str) -> Result<REPLAction, String> {
        let mut args = line.split_whitespace();
        let cmd = args.next().ok_or(format!(
            "Enter a command - use {} for a list of commands",
            "help".bold()
        ))?;

        match cmd {
            "add" => match args.next() {
                Some("project") => {
                    let name = args.next().ok_or("New project name required!")?;
                    let project = Project::new(name);
                    println!("Added new project {}", project);
                    self.add_project(project);
                    Ok(REPLAction::Continue)
                }
                Some("entry") => {
                    let project = args.next().ok_or("Project name needed!")?;
                    let activity = args.next().ok_or("New entry activity needed!")?;
                    let project = self
                        .find_project_mut(project)
                        .ok_or(format!("Project {} not known!", project))?;
                    let entry = Entry::new(activity);
                    println!(
                        "Added and started new entry {} on project {}",
                        entry, project
                    );
                    project.add_entry(entry);

                    Ok(REPLAction::Continue)
                }
                Some(sub) => Err(format!("Unknown sub-command '{}'!", sub)),
                None => Err("Sub-command needed!")?,
            },
            "list" => {
                match args.next() {
                    Some(project) => {
                        let project = self
                            .find_project_mut(project)
                            .ok_or(format!("Project {} not known!", project))?;
                        println!("Entries for project {}", project.name());
                        for entry in project.entries() {
                            println!(" - {}", entry);
                        }
                    }
                    None => {
                        if !self.projects.is_empty() {
                            println!("All projects:");
                            for project in self.projects.iter() {
                                println!(" - {}", project);
                            }
                        } else {
                            println!("No projects found - use {} to change this!", "add".bold());
                        }
                    }
                }

                Ok(REPLAction::Continue)
            }
            "finish" => {
                match args.next() {
                    Some(project) => {
                        let project = self
                            .find_project_mut(project)
                            .ok_or(format!("Project {} not known!", project))?;
                        if let Some(activity) = args.next() {
                            println!("TODO: Not implemented yet.");
                        } else {
                            if let Some(latest) = project.finish_last() {
                                println!("Finished {}", latest);
                            }
                        }
                    }
                    None => Err("Project name needed!")?,
                }
                Ok(REPLAction::Continue)
            }
            "help" => {
                println!("Available commands:");
                println!("  add                            Add ... to track");
                println!("    project <name>               Add a new project");
                println!("    entry <project> <activity>   Add a new entry starting now ☕");
                println!(
                    "  finish <project> [activity]    Finish activity of project or last acidity"
                );
                println!("  list                           List all projects");
                println!("  help                           Displays this help text");
                println!("  quit / exit                    Quit Tracky");
                Ok(REPLAction::Continue)
            }
            "quit" | "exit" => Ok(REPLAction::Quit),
            _ => Err(format!("Unknown command '{}'!", cmd)),
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

    fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    fn rename(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    fn finish_last(&mut self) -> Option<&Entry> {
        match self.entries.last_mut() {
            Some(entry) => {
                entry.finish();
                Some(entry)
            }
            None => None
        }
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct Entry {
    activity: String,
    start: time::SystemTime,
    end: Option<time::SystemTime>,
    duration: Option<time::Duration>,
}

impl Entry {
    fn new(activity: &str) -> Entry {
        Entry {
            activity: String::from(activity),
            start: time::SystemTime::now(),
            end: None,
            duration: None,
        }
    }

    fn finish(&mut self) {
        if self.end.is_none() {
            let end = time::SystemTime::now();
            self.duration = Some(
                end.duration_since(self.start)
                    .unwrap_or(time::Duration::from_secs(1)), // TODO: Not the best solution
            );
            self.end = Some(end);
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.duration {
            Some(duration) => write!(f, "{} ⚡  took {}s", self.activity.green(), duration.as_secs()),
            None => write!(f, "{} ☕ ", self.activity.yellow()),
        }
    }
}
