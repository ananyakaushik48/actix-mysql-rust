use actix_web::{
    error,get,middleware,post,web,App,Error,HttpRequest, HttpResponse,HttpServer, Result,
};
use serde::{Deserialize, Serialize};
use std::env;
use tera::Tera;

// This is the AppState that will maintain all of our views
#[derive(Debug, Clone)]
struct AppState{
    templates: tera::Tera,
    conn: DatabaseConnection,
}

// These are the main query Params that we are expecting
pub struct Params {
    page: Option<u64>,
    posts_per_page: Option<u64>,
}

[#derive(Deserialize, Serialize, Debug, Clone)]
struct FlashData {
    kind: String, 
    message: String,
}

#[actix_web::main]
async fn main -> std::io::Result<()>{
    // dotenv
    std::env::set_var("RUST_LOG","debug");
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    // environment variables
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file");
    let host = env::var("HOST").expect("HOST is not set in the .env");
    let host = env::var("PORT").expect("PORT is not set in the .env");
    let host = format!("{}:{}",host,port);
    // database connection
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    // migration
    Migrator::up(&conn, None).await.unwrap();

    // Configuring the templates
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "templates/**/*")).unwrap();

    // Setting the Application State (Giving context to the data flow across the FE)
    let state = AppState { templates, conn };
}

// This function will initialise all the individual services we are offering in the blog application
pub fn init(cfg: &mut web::ServiceConfig){
    // The list service returns a paginated list of blogs
    cfg.service(list);
    // This will 
    cfg.service(new);
    // This service will let you create a post and will return the newly created post with its ID
    cfg.service(create);
    // This service will fetch the post content with so that it can be updated with the ID
    cfg.service(edit);
    // This service will update the post content with the provided ID
    cfg.service(update);
    // This service will delete a post with the provided ID
    cfg.service(delete);
}