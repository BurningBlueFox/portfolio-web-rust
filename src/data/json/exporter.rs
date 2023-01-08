use crate::data::career::{Career, CareerExporter};

struct Exporter {}

impl CareerExporter for Exporter {
    fn export(&self, career: &Career) -> String {
        String::from("Test")
    }
}
