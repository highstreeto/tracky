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
                Some("task") => {
                    let project = args.next().ok_or("Project name needed!")?;
                    let activity = args.next().ok_or("New entry activity needed!")?;
                    let project = self
                        .find_project_mut(project)
                        .ok_or(format!("Project {} not known!", project))?;
                    let entry = Task::new(activity);
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
                        for entry in project.tasks() {
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
                            let task = project.finish(activity).ok_or("No tasks to finish!")?;
                            println!("Finished {}", task);
                        } else {
                            let latest = project.finish_last().ok_or("No tasks to finish!")?;
                            println!("Finished {}", latest);
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
                println!("    task <project> <activity>   Add a new task starting now ☕");
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
    tasks: Vec<Task>,
}

impl Project {
    fn new(name: &str) -> Project {
        Project {
            name: String::from(name),
            tasks: vec![],
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    fn rename(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    fn add_entry(&mut self, entry: Task) {
        self.tasks.push(entry);
    }

    fn finish(&mut self, activity: &str) -> Option<&Task> {
        let task = self.tasks
            .iter_mut()
            .filter(|task| task.activity == activity)
            .next();
        match task {
            Some(entry) => {
                entry.finish();
                Some(entry)
            }
            None => None,
        }
    }

    fn finish_last(&mut self) -> Option<&Task> {
        match self.tasks.last_mut() {
            Some(entry) => {
                entry.finish();
                Some(entry)
            }
            None => None,
        }
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct Task {
    activity: String,
    start: time::SystemTime,
    end: Option<time::SystemTime>,
    duration: Option<time::Duration>,
}

impl Task {
    fn new(activity: &str) -> Task {
        Task {
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

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.duration {
            Some(duration) => write!(
                f,
                "{} ⚡  took {}s",
                self.activity.green(),
                duration.as_secs()
            ),
            None => write!(f, "{} ☕ ", self.activity.yellow()),
        }
    }
}
