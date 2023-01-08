use crate::data::career::{Career, CareerImporter};

struct Importer {
    json_file_name: String,
}

impl Importer {
    pub fn new(json_file_name: String) -> Self { Self { json_file_name } }
}

impl CareerImporter for Importer {
    fn import(&self) -> Career {
        Career {
            projects: todo!(),
            experiences: todo!(),
        }
    }
}
