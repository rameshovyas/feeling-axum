use axum::Extension;
use sea_orm::{DatabaseConnection,Set, ActiveModelTrait};
use crate::database::users;
use chrono::{self, Utc};
pub async  fn create_user(Extension(database) : Extension<DatabaseConnection>){
    
    let new_user = users::ActiveModel {
        email :  Set("ramesh@vyas.com".to_owned()),
        password :Set("123456".to_owned()),
        name : Set("Ramesh Vyas".to_owned()),
        created_on : Set(Utc::now().naive_utc()),     
        ..Default::default()
    };

    let result = new_user.save(&database).await.unwrap();
    dbg!(result);
}