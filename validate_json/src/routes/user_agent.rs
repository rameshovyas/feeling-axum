use axum::{TypedHeader, headers::UserAgent};
pub async fn user_agent(TypedHeader(user_agent) : TypedHeader<UserAgent>) -> String{
    user_agent.to_string()
}