mod routes;

use std::net::SocketAddr;

use crate::routes::create_routes;

pub async fn run() {
    let app = create_routes();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
