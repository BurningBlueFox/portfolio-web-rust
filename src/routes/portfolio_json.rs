use axum::Json;

pub async fn portfolio_json() -> Json<&'static str> {
    Json("")
}
