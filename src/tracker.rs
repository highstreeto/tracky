use colored::*;
use serde::{Deserialize, Serialize};
use std::{fmt, fs, io::prelude::*, path::PathBuf, time};

#[derive(Serialize, Deserialize, Default)]
pub struct TimeTracker {
    projects: Vec<Project>,
}

impl TimeTracker {
    pub fn new() -> TimeTracker {
        Default::default()
    }

    fn default_path() -> PathBuf {
        if let Some(mut file) = dirs::home_dir() {
            file.push("tracky.json");
            file
        } else {
            PathBuf::from("tracky.json")
        }
    }

    pub fn projects(&self) -> impl Iterator<Item = &Project> {
        self.projects.iter()
    }

    pub fn load() -> Result<TimeTracker, String> {
        let path = TimeTracker::default_path();
        let mut file = fs::File::open(&path).map_err(|err| err.to_string())?;
        println!(
            "  Loading from {}...",
            path.to_str().expect("Not a valid Unicode path!")
        );

        let mut json = String::new();
        file.read_to_string(&mut json)
            .map_err(|err| err.to_string())?;
        Ok(serde_json::from_str(&json).map_err(|err| err.to_string())?)
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }

    pub fn find_project_mut(&mut self, name: &str) -> Option<&mut Project> {
        self.projects
            .iter_mut()
            .find(|proj| proj.name() == name)
    }

    pub fn save(&self) -> Result<(), String> {
        let path = TimeTracker::default_path();

        let json = serde_json::to_string(self).map_err(|err| err.to_string())?;
        let mut file = fs::File::create(&path).map_err(|err| err.to_string())?;
        file.write_all(json.as_bytes())
            .map_err(|err| err.to_string())?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    name: String,
    tasks: Vec<Task>,
}

impl Project {
    pub fn new(name: &str) -> Project {
        Project {
            name: String::from(name),
            tasks: vec![]
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter()
    }

    pub fn add_task(&mut self, entry: Task) {
        self.tasks.push(entry);
    }

    pub fn finish(&mut self, activity: &str) -> Option<&Task> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.activity == activity);
        match task {
            Some(entry) => {
                entry.finish();
                Some(entry)
            }
            None => None,
        }
    }

    pub fn finish_last(&mut self) -> Option<&Task> {
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

#[derive(Serialize, Deserialize)]
pub struct Task {
    activity: String,
    start: time::SystemTime,
    end: Option<time::SystemTime>,
    duration: Option<time::Duration>,
}

impl Task {
    pub fn new(activity: &str) -> Task {
        Task {
            activity: String::from(activity),
            start: time::SystemTime::now(),
            end: None,
            duration: None,
        }
    }

    pub fn finish(&mut self) {
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