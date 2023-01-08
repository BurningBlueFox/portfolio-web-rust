mod data;
mod routes;

use std::net::SocketAddr;

use crate::routes::create_routes;

use data::career::CareerImporter;
use data::json::importer::Importer;

pub async fn run() {
    let app = create_routes();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let importer: Box<dyn CareerImporter> = Box::new(Importer::new("portfolio.json".to_owned()));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
