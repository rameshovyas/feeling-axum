use axum::Extension;
use super::SharedData;

pub async fn middleware_data(Extension(shared_data):Extension<SharedData>) ->String {
    shared_data.data
}