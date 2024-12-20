use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../iter2/handlers.rs"]
mod handlers;
#[path = "../iter2/models.rs"]
mod models;
#[path = "../iter2/routes.rs"]
mod routes;
#[path = "../iter2/state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let databse_url = env::var("DATABASE_URL").expect(
        "DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&databse_url).await.unwrap();
    let shared_data = web::Data::new(AppState {
        health_check_handler: "I'm good. You've aleardy asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool
    });
    // 앱을 구성하고 라우트를 구성한다
    let app = move || {
        App::new()
        .app_data(shared_data.clone())
        .configure(general_routes)
        .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
