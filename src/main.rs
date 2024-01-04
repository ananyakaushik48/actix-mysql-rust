use actix_web::{
    error,get,middleware,post,web,App,Error,HttpRequest, HttpResponse,HttpServer, Result,
};
use std::env;

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
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    // database connection
    // migration
}