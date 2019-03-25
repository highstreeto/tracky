use serde::{Deserialize, Serialize};
use std::time;

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

    pub fn all_tasks(&self) -> impl Iterator<Item = &Task> {
        self.started
            .iter()
            .map(|task| task as &Task)
            .chain(self.finished.iter().map(|task| task as &Task))
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

pub trait Task {
    fn activity(&self) -> &str;

    fn state(&self) -> TaskState;
}

pub enum TaskState {
    Started,
    Finished
}

#[derive(Serialize, Deserialize)]
pub struct StartedTask {
    activity: String,
    start: time::SystemTime,
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

impl Task for StartedTask {
    fn activity(&self) -> &str {
        &self.activity
    }

    fn state(&self) -> TaskState {
        TaskState::Started
    }
}

#[derive(Serialize, Deserialize)]
pub struct FinishedTask {
    activity: String,
    start: time::SystemTime,
    end: time::SystemTime,
    duration: time::Duration,
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

impl Task for FinishedTask {
    fn activity(&self) -> &str {
        &self.activity
    }

    fn state(&self) -> TaskState {
        TaskState::Finished
    }
}