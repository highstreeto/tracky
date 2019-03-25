use serde_json;
use serde::{Deserialize, Serialize};
use std::{fs, io::prelude::*, path::Path};
use project::Project;

pub mod project;

#[derive(Serialize, Deserialize, Default)]
pub struct TimeTracker {
    projects: Vec<Project>,
}

impl TimeTracker {
    pub fn new() -> TimeTracker {
        Default::default()
    }

    pub fn projects(&self) -> impl Iterator<Item = &Project> {
        self.projects.iter()
    }

    pub fn load(path: &Path) -> Result<TimeTracker, String> {
        let mut file = fs::File::open(&path).map_err(|err| err.to_string())?;
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

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let json = serde_json::to_string(self).map_err(|err| err.to_string())?;
        let mut file = fs::File::create(path).map_err(|err| err.to_string())?;
        file.write_all(json.as_bytes())
            .map_err(|err| err.to_string())?;
        Ok(())
    }
}