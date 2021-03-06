use colored::*;
use tracky::{project::{Project, Task, TaskState}, TimeTracker};

#[derive(Debug, PartialEq)]
pub enum REPLAction {
    Continue,
    Quit,
}

pub fn handle_repl(tracker: &mut TimeTracker, line: &str) -> Result<REPLAction, String> {
    let mut args = line.split_whitespace();
    let cmd = args.next().ok_or_else(|| {
        format!(
            "Enter a command - use {} for a list of commands",
            "help".bold()
        )
    })?;

    match cmd {
        "add" => match args.next() {
            Some("project") => {
                let name = args.next().ok_or("New project name required!")?;
                let project = Project::new(name);
                println!("Added new project {}", project.name());
                tracker.add_project(project);
                Ok(REPLAction::Continue)
            }
            Some("task") => {
                let project = args.next().ok_or("Project name needed!")?;
                let activity = args.next().ok_or("New entry activity needed!")?;
                let project = tracker
                    .find_project_mut(project)
                    .ok_or_else(|| format!("Project {} not known!", project))?;
                let task = project.start_task(activity);
                println!("Started task {}", format_task(task));

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
                        .ok_or_else(|| format!("Project {} not known!", project))?;
                    println!("Tasks for project {}", project.name());
                    for task in project.all_tasks() {
                        println!(" - {}", format_task(task));
                    }
                }
                None => {
                    if tracker.projects().next().is_some() {
                        println!("All projects:");
                        for project in tracker.projects() {
                            println!(" - {}", project.name());
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
                        .ok_or_else(|| format!("Project {} not known!", project))?;
                    if let Some(activity) = args.next() {
                        let task = project.finish_task(activity).ok_or("No tasks to finish!")?;
                        println!("Finished {}", format_task(task));
                    } else {
                        let latest = project.finish_last_task().ok_or("No tasks to finish!")?;
                        println!("Finished {}", format_task(latest));
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
            println!("    task <project> <activity>    Add a new task starting now ☕");
            println!("  finish <project> [activity]    Finish activity of project or last activity");
            println!("  list                           List all projects");
            println!("  help                           Displays this help text");
            println!("  quit / exit                    Quit and save Tracky");
            Ok(REPLAction::Continue)
        }
        "quit" | "exit" => {
            Ok(REPLAction::Quit)
        }
        _ => Err(format!("Unknown command '{}'!", cmd)),
    }
}

fn format_task(task: &Task) -> String {
    match task.state() {
        TaskState::Started => format!("{} ☕ ", task.activity().yellow()),
        TaskState::Finished => format!("{} ⚡ ", task.activity().green())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help() {
        let mut tracker = TimeTracker::new();

        let action = handle_repl(&mut tracker, "help").unwrap();
        assert_eq!(REPLAction::Continue, action);
    }

    #[test]
    fn list() {
        let mut tracker = TimeTracker::new();

        let action = handle_repl(&mut tracker, "list").unwrap();
        assert_eq!(REPLAction::Continue, action);
    }

    #[test]
    fn list_tasks() {
        let mut tracker = TimeTracker::new();

        assert_eq!(
            "Project Test not known!",
            handle_repl(&mut tracker, "list Test").unwrap_err()
        );

        tracker.add_project(Project::new("Test"));

        let action = handle_repl(&mut tracker, "list Test").unwrap();
        assert_eq!(REPLAction::Continue, action);
    }

    #[test]
    fn list_tasks_unknown_project() {
        let mut tracker = TimeTracker::new();
        assert_eq!(
            "Project Test not known!",
            handle_repl(&mut tracker, "list Test").unwrap_err()
        );
    }

    #[test]
    fn unknown_cmd() {
        let mut tracker = TimeTracker::new();
        assert_eq!(
            "Unknown command 'unknown'!",
            handle_repl(&mut tracker, "unknown").unwrap_err()
        );
    }

    #[test]
    fn empty_cmd() {
        let mut tracker = TimeTracker::new();
        assert_eq!(
            format!(
                "Enter a command - use {} for a list of commands",
                "help".bold()
            ),
            handle_repl(&mut tracker, "").unwrap_err()
        );
    }

    #[test]
    fn quit_and_exit() {
        let mut tracker = TimeTracker::new();
        assert_eq!(REPLAction::Quit, handle_repl(&mut tracker, "quit").unwrap());
        assert_eq!(REPLAction::Quit, handle_repl(&mut tracker, "exit").unwrap());
    }
}
