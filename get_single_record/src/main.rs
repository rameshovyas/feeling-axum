use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use db_seaorm::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = dotenv!("DATABASE_URL");
    print!("{}",database_url.to_string());
    run(database_url).await;
}
