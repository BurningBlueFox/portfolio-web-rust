use crate::data::career::{Career, CareerExporter};

pub struct Exporter {}

impl Exporter {
    pub fn new() -> Self { Self {  } }
}

impl CareerExporter for Exporter {
    fn export(&self, career: &Career) -> String {
        let json = serde_json::to_string(career).unwrap();
            return json;
    }
}
