use std::{fs::File, io::BufReader};

use serde_json::Value;

use crate::data::career::{Career, CareerImporter, Experience, ExternalLink, Project};

use super::*;

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
        let mut projects: Vec<Project> = Vec::new();

        for item in value.as_array().unwrap() {
            let proj = Project {
                id: item[KEY_ID].as_u64().unwrap(),
                name: item[KEY_NAME].to_string(),
                date: item[KEY_PROJECT_DATE].to_string(),
                links: self.extract_project_links(item),
            };

            projects.push(proj);
        }

        return projects;
    }

    fn extract_experiences(&self, value: &Value, projects: &Vec<Project>) -> Vec<Experience> {
        let mut experiences: Vec<Experience> = Vec::new();

        for item in value.as_array().unwrap() {
            let exp = Experience {
                name: todo!(),
                description: todo!(),
                start_date: todo!(),
                end_date: todo!(),
                current_job: todo!(),
                projects: todo!(),
            };

            experiences.push(exp);
        }

        return experiences;
    }

    fn extract_project_links(&self, item: &Value) -> Vec<ExternalLink> {
        let mut links: Vec<ExternalLink> = Vec::new();

        return links;
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
