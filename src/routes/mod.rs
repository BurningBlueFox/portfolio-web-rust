mod portfolio_json;

use crate::routes::portfolio_json::portfolio_json;
use axum::{body::Body, routing::get, Router};

pub fn create_routes() -> Router<(), Body> {
    Router::new().route("/", get(portfolio_json))
}
