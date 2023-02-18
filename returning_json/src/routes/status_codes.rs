use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn status_codes() ->impl IntoResponse {
    (
        StatusCode::CREATED,
        "This is 201 response and not 200".into_response()
    )
}