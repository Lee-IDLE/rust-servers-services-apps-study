use actix_files as fs;
use actix_web::{error, web::{self, Data}, App, Error, HttpResponse, HttpServer, Result};
use std::env;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_|
        "127.0.0.1:8080".to_string());
    println!("Listening on: {}, open browser and visit have a try!", addr);
    HttpServer::new(|| {
        // 새로운 Tera 인스턴스를 만든다. Tera 템플릿의 위치는
        // /static/iter1/ 디렉터리로 지정하자. 앞에서 tera 템플릿 태그
        // {{name}}를 포함하고 있는 index.html을 여기에 두었다.
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter1/**/*")).unwrap();

        // Tera 인스턴스를 애플리케이션의 디펜던시로 주입한다. 
        // 모든 라우트 핸들러 안에서 Tera에 접근할 수 있게 된다.
        App::new()
        .app_data(Data::new(tera))
        .service(fs::Files::new("/static", "./static").show_files_listing())
        .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(addr)?
    .run()
    .await
}

async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    // index 핸들러에 전달된 인수의 일부분으로, Tera 인스턴스에 접근한다.
    // 새로운 Tera Context 객체를 만든다. 이 객체를 사용해 웹페이지에 데이터를 접근한다.
    let mut ctx = tera::Context::new();
    // index 핸들러 안에서 name 변수에 값을 할당한다.
    ctx.insert("name", "Bob");

    let s = tmpl
        // 구성된 동적 웹페이지를 HTTP응답 바디의 일부로서 전달한다.
        // index 핸들러 함수에서 hTTP 응답을 반환한다. 
        .render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    // index 핸들러 함수에서 HTTP 응답을 반환한다.
    // 구성된 동적 웹 페이지를 HTTP 응답 바디의 일부로 전달한다.
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}