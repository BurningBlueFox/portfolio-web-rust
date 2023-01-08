pub struct Career<'a> {
    pub projects: Vec<Project>,
    pub experiences: Vec<Experience<'a>>,
}

pub trait CareerImporter {
    fn import(&self) -> Career;
}

pub trait CareerExporter {
    fn export(&self, career: &Career) -> String;
}

pub struct Project {
    id: u32,
    name: String,
    date: String,
    links: Vec<ExternalLink>,
}

pub struct Experience<'a> {
    name: String,
    description: String,
    start_date: String,
    end_date: String,
    current_job: bool,
    projects: Vec<&'a Project>,
}

pub enum ExternalLink {
    Image { url: String },
    YoutubeVideo { url: String },
    Webpage { url: String, description: String },
}
