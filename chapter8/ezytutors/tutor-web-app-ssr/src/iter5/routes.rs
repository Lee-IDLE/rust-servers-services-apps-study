use crate::handler::{handle_register, show_register_form};
use actix_files as fs; // 정적 파일 제공
use actix_web::web;

// 서비스 구성을 생성하고 라우트와 관련된 핸들러를 지정한다.
pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(show_register_form)))
            .service(web::resource("/register").route(web::post().to(handle_register)))
    );
}