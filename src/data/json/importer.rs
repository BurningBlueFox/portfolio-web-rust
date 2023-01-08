use std::{fs::File, io::BufReader};

use serde_json::Value;

use crate::data::career::{Career, CareerImporter, Experience, Project};

use super::{KEY_CAREER_EXPERIENCE, KEY_CAREER_PROJECTS};

pub struct Importer {
    json_file_name: String,
}

impl Importer {
    pub fn new(json_file_name: String) -> Self {
        Self { json_file_name }
    }

    fn parse(&self) -> Value {
        let file = File::open(&self.json_file_name)
            .ok()
            .expect("Error opening Json file");
        let reader = BufReader::new(file);

        serde_json::from_reader(reader)
            .ok()
            .expect("Error parsing Json")
    }

    fn extract_projects(&self, value: &Value) -> Vec<Project> {
        let projects: Vec<Project> = Vec::new();
        return projects;
    }

    fn extract_experiences(&self, value: &Value, projects: &Vec<Project>) -> Vec<Experience> {
        let experiences: Vec<Experience> = Vec::new();
        return experiences;
    }
}

impl CareerImporter for Importer {
    fn import(&self) -> Career {
        let json = self.parse();
        let projects = self.extract_projects(&json[KEY_CAREER_PROJECTS]);
        let experiences = self.extract_experiences(&json[KEY_CAREER_EXPERIENCE], &projects);

        Career {
            projects,
            experiences,
        }
    }
}
