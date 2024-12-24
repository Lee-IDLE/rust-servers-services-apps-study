use actix_web::{error::Error, web, App, HttpResponse, HttpServer};

async fn hello() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello there!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/hello", web::get().to(hello)))
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
