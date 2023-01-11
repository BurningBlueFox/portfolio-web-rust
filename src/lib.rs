mod data;
mod routes;

use std::net::SocketAddr;

use crate::data::career::CareerExporter;
use crate::data::json::exporter::Exporter;
use crate::routes::create_routes;

use data::career::CareerImporter;
use data::json::importer::Importer;

pub fn set_data() -> String {
    let importer: Box<dyn CareerImporter> =
        Box::new(Importer::new("data/portfolio.json".to_owned()));
    let exporter: Box<dyn CareerExporter> = Box::new(Exporter::new());

    let c = importer.import();
    exporter.export(&c)
}

pub async fn run() {
    let app = create_routes();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Start Server");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
