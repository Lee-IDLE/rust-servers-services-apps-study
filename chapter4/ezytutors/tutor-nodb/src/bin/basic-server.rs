// 모듈 임포트
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// 라우트를 구성한다. 
// Actix 웹서버는 /health 경로로 유입되는 HTTP GET 요청을 health_check_handler()로 전달한다
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// 핸들러를 구성한다.
// 핸들러는 인사와 함께 HTTP 요청을 구현한다.
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

// HTTP 서버를 인스턴스화하고 실행한다.
#[actix_web::main]
async fn main() -> io::Result<()> {
    // app을 만들고 라우트를 구성한다.
    // Actix웹 애플리케이션 인스턴스를 만들고 설정된 경로에 등록한다.
    let app = move || App::new().configure(general_routes);
    
    // HTTP 서버를 시작한다.
    // 웹서버를 초기화하고, 애플리케이션을 로드하고, 이를 소켓에 바인딩한 뒤 서버를 실행한다.
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}