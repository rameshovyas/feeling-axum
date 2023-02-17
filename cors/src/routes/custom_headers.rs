use axum::http::HeaderMap;

pub async fn custom_headers(headers : HeaderMap) -> String{
   let message_value = headers.get("x-message").unwrap();
   let message = message_value.to_str().unwrap().to_owned();
   message
}