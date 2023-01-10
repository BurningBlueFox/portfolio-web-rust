pub struct Career {
    pub projects: Vec<Project>,
    pub experiences: Vec<Experience>,
}

pub trait CareerImporter {
    fn import(&self) -> Career;
}

pub trait CareerExporter {
    fn export(&self, career: &Career) -> String;
}

pub struct Project {
    pub id: u64,
    pub name: String,
    pub date: String,
    pub links: Vec<ExternalLink>,
}

pub struct Experience {
    pub name: String,
    pub description: String,
    pub start_date: String,
    pub end_date: String,
    pub current_job: bool,
    pub projects: Vec<u64>,
}

pub enum ExternalLink {
    Image { url: String },
    YoutubeVideo { url: String },
    Webpage { url: String, description: String },
}
