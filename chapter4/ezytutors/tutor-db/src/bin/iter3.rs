use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../iter3/db_access.rs"]
mod db_access;
#[path = "../iter3/handlers.rs"]
mod handlers;
#[path = "../iter3/models.rs"]
mod models;
#[path = "../iter3/routes.rs"]
mod routes;
#[path = "../iter3/state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count:Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone()) // App 상태를 애플리케이션 인스턴스에 주입
            .configure(general_routes) // 라우트 구성
            .configure(course_routes)
    };
    
    // HTTP Server 시작
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}