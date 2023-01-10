use std::{fs::File, io::BufReader};

use serde_json::Value;

use crate::data::career::{Career, CareerImporter, Experience, ExternalLink, Project};
use crate::data::career::ExternalLink::*;

use super::*;

pub struct Importer{
    json_file_name: String,
    projects: Vec<Project>
}

impl Importer{
    pub fn new(json_file_name: String) -> Self {
        Self {
            json_file_name,
            projects: Vec::new(),
        }
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

    fn extract_experiences(&self, value: &Value) -> Vec<Experience> {
        let mut experiences: Vec<Experience> = Vec::new();

        for item in value.as_array().unwrap() {
            let exp = Experience {
                name: item[KEY_NAME].to_string(),
                description: item[KEY_DESCRIPTION].to_string(),
                start_date: item[KEY_EXPERIENCE_START_DATE].to_string(),
                end_date: item[KEY_EXPERIENCE_END_DATE].to_string(),
                current_job: item[KEY_EXPERIENCE_CURRENT_JOB].as_bool().unwrap(),
                projects: self.extract_experience_projects(item),
            };

            experiences.push(exp);
        }

        return experiences;
    }

    fn extract_project_links(&self, item: &Value) -> Vec<ExternalLink> {
        let mut links: Vec<ExternalLink> = Vec::new();

        for entry in item[KEY_IMAGES].as_array().unwrap(){
            links.push(Image{url: entry.to_string()});
        }

        for entry in item[KEY_VIDEOS].as_array().unwrap(){
            links.push(YoutubeVideo{url: entry.to_string()});
        }

        for entry in item[KEY_LINKS].as_array().unwrap(){
            let url = entry[KEY_URL].to_string();
            let description = entry[KEY_DESCRIPTION].to_string();
            links.push(Webpage{url, description});
        }

        return links;
    }

    fn extract_experience_projects(&self, item: &Value) -> Vec<&Project> {
        let mut proj: Vec<&Project> = Vec::new();

        for entry in item[KEY_EXPERIENCE_PROJECTS].as_array().unwrap() {
            let id = entry.as_u64().unwrap();

            let proj_reference: &Project =
                &(&self.projects).into_iter().find(|x| x.id == id).unwrap();
            proj.push(proj_reference);
        }

        return proj;
    }
}

impl CareerImporter for Importer{
    fn import(&self) -> Career {
        let json = self.parse();
        let projects = self.extract_projects(&json[KEY_CAREER_PROJECTS]);
        let experiences = self.extract_experiences(&json[KEY_CAREER_EXPERIENCE]);

        Career {
            projects,
            experiences,
        }
    }
}
