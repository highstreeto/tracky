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
        self.projects.iter_mut().find(|proj| proj.name() == name)
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
    started: Vec<StartedTask>,
    finished: Vec<FinishedTask>,
}

impl Project {
    pub fn new(name: &str) -> Project {
        Project {
            name: String::from(name),
            started: vec![],
            finished: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn all_tasks(&self) -> impl Iterator<Item = &fmt::Display> {
        self.started
            .iter()
            .map(|task| task as &fmt::Display)
            .chain(self.finished.iter().map(|task| task as &fmt::Display))
    }

    pub fn start_task(&mut self, activity: &str) -> &StartedTask {
        let task = StartedTask {
            activity: String::from(activity),
            start: time::SystemTime::now(),
        };
        self.started.push(task);
        self.started.last().unwrap()
    }

    pub fn finish_task(&mut self, activity: &str) -> Option<&FinishedTask> {
        let entry = self
            .started
            .iter()
            .enumerate()
            .find(|(_, task)| task.activity == activity);
        match entry {
            Some((idx, _)) => {
                let task = self.started.remove(idx);
                let task = task.finish();
                self.finished.push(task);
                Some(self.finished.last().unwrap())
            }
            None => None,
        }
    }

    pub fn finish_last_task(&mut self) -> Option<&FinishedTask> {
        if !self.started.is_empty() {
            Some(self.finish_task_at(self.started.len() - 1))
        } else {
            None
        }
    }

    fn finish_task_at(&mut self, idx: usize) -> &FinishedTask {
        let task = self.started.remove(idx);
        let task = task.finish();
        self.finished.push(task);
        self.finished.last().unwrap()
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Serialize, Deserialize)]
pub struct StartedTask {
    activity: String,
    start: time::SystemTime,
}

#[derive(Serialize, Deserialize)]
pub struct FinishedTask {
    activity: String,
    start: time::SystemTime,
    end: time::SystemTime,
    duration: time::Duration,
}

impl StartedTask {
    pub fn new(activity: &str) -> StartedTask {
        let activity = String::from(activity);
        StartedTask {
            activity,
            start: time::SystemTime::now(),
        }
    }

    fn finish(self) -> FinishedTask {
        FinishedTask::new(self)
    }
}

impl fmt::Display for StartedTask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ☕ ", self.activity.yellow())
    }
}

impl FinishedTask {
    fn new(task: StartedTask) -> FinishedTask {
        let end = time::SystemTime::now();
        let duration = end
            .duration_since(task.start)
            .unwrap_or(time::Duration::from_secs(1)); // TODO: Not the best solution

        FinishedTask {
            activity: task.activity,
            start: task.start,
            end,
            duration,
        }
    }
}

impl fmt::Display for FinishedTask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ⚡  took {}s",
            self.activity.green(),
            self.duration.as_secs()
        )
    }
}
