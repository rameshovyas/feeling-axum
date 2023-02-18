use dotenvy::dotenv;
use dotenvy_macro::dotenv;

fn main() {
    dotenv().ok();
    let database_url = dotenv!("DATABASE_URL");
    print!("{}",database_url.to_string());
    
}
