pub mod tracker;

use std::env;
use tracker::{TimeTracker, Project, Task};
use colored::*;

pub enum REPLAction {
    Continue,
    Quit,
}

pub fn handle_repl(tracker: &mut TimeTracker, line: &str) -> Result<REPLAction, String> {
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
                    tracker.add_project(project);
                    Ok(REPLAction::Continue)
                }
                Some("task") => {
                    let project = args.next().ok_or("Project name needed!")?;
                    let activity = args.next().ok_or("New entry activity needed!")?;
                    let project = tracker
                        .find_project_mut(project)
                        .ok_or(format!("Project {} not known!", project))?;
                    let entry = Task::new(activity);
                    println!(
                        "Added and started new entry {} on project {}",
                        entry, project
                    );
                    project.add_task(entry);

                    Ok(REPLAction::Continue)
                }
                Some(sub) => Err(format!("Unknown sub-command '{}'!", sub)),
                None => Err("Sub-command needed!")?,
            },
            "list" => {
                match args.next() {
                    Some(project) => {
                        let project = tracker
                            .find_project_mut(project)
                            .ok_or(format!("Project {} not known!", project))?;
                        println!("Entries for project {}", project.name());
                        for entry in project.tasks() {
                            println!(" - {}", entry);
                        }
                    }
                    None => {
                        if !tracker.projects().next().is_some() {
                            println!("All projects:");
                            for project in tracker.projects() {
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
                        let project = tracker
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
            "save" => {
                tracker.save()?;
                Ok(REPLAction::Continue)
            }
            "help" => {
                println!(
                    "current dir: {}",
                    env::current_dir()
                        .map_err(|err| err.to_string())?
                        .iter()
                        .last()
                        .expect("No last path element")
                        .to_str()
                        .expect("No unicode path!") // TODO: Use CamelCase for str
                );

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
