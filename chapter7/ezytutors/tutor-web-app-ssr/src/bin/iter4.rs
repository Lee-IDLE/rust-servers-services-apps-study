use actix_files as fs;
use actix_web::{error::{self, ErrorInternalServerError}, web::{self, Data}, App, Error, HttpResponse, HttpServer, Result};
use awc::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tera::Tera;

#[derive(Serialize, Deserialize)]
pub struct Tutor {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

async fn handle_get_tutors(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    // 웹서비스와 통신하기 위한 ACtix Web HTTP 클라이언트를 만든다.
    let client = Client::default();

    // unwrap 키워드를 사용해 네트워크 응답 결과를 추출했다. 
    // 튜터 웹 애플리케이션을 작성할 때, 에러들을 프로덕션에 준하는 방법으로 처리할 것이다.
    // unwrap은 형재 프로세스를 종료시키므로 프로덕션에서의 사용에 적합하지 않지만,
    // 소프트웨어 개발 초기 단계를 단순화해준다.
    let response = client
        .get("http://localhost:3000/tutors/")
        .send()
        .await
        .unwrap()
        .body()
        .await
        .unwrap();

    let str_list = std::str::from_utf8(&response.as_ref()).unwrap();
    let tutor_list: Vec<Tutor> = serde_json::from_str(str_list).unwrap();
    let mut ctx = tera::Context::new();
    
    ctx.insert("tutors", &tutor_list);
    
    let rendered_html = tmpl
        .render("list.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered_html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on : 127.0.0.1:8080");
    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter4/**/*")).unwrap();

        App::new()
            .app_data(Data::new(tera))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/tutors").route(web::get().to(handle_get_tutors)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}