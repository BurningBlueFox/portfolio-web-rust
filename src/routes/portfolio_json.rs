use axum::Json;

use crate::set_data;

pub async fn portfolio_json() -> Json<String> {
    Json(set_data())
}
