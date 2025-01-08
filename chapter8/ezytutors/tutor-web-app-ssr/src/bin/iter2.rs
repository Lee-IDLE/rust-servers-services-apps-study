use actix_web::{error, web::{self, Data}, App, Error, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::env;
use tera::Tera;

// 애플리케이션 상태에 Tera 템플릿을 저장한다.
// index 핸들러 함수는 / 라우트로 HTTP 요청이 들어왔을 때 호출된다. 
// 이것은 사용자가 이름을 입력할 수 있는 폼을 표시한다.
async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    // 새로운 Tera 콘텍스트 객체와 함께 form.html을 렌더링한다. form.html 파일에는 어떤 템플릿 변수도 포함되지 않으므로
    // 콘텍스트에 아무런 데이터를 삽입하지 않는다.
    let s = tmpl
    .render("form.html", &tera::Context::new())
    .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// 직렬화할 수 있는 구조체인 Tutor를 사용해 폼에서 얻을 데이터를 나타낸다.
// 이것은 커스텀 데이터 구조체이며, 원하는 대로 구조체를 정의할 수 있다.
#[derive(Serialize, Deserialize)]
pub struct Tutor {
    name: String
}

// 이 두번째 핸들러 함수는 사용자가 강사 이름을 입력하고 submit 버튼을 누르면 호출된다.
async fn handle_post_tutor(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<Tutor>
) -> Result<HttpResponse, Error> {
    // 사용자가 제출한 폼 데이터(즉, 강사 이름)를 핸들러 함수에서 접근할 수 있다.
    // 이는 Actix의 web::Form<T> 추출자를 통해 이루어진다.
    let mut ctx = tera::Context::new();
    ctx.insert("name", &params.name);
    ctx.insert("text", "Welcome!");

    let s = tmpl
        .render("user.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on : 127.0.0.1:8080");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter2/**/*")).unwrap();

        App::new()
            .app_data(Data::new(tera))
            .configure(app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
        .service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/tutors").route(web::post().to(handle_post_tutor)))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::header::{CONTENT_TYPE, HeaderValue};
    use actix_web::http::StatusCode;
    use actix_web::web::Form;

    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::test::{self, TestRequest};

    // 이 애너테이션 이후의 함수가 Actix 런타임에 의해 실행돼야 하는 테스트 함수임을 Actix런타임에게 알린다.
    #[actix_rt::test] 
    async fn handle_post_1_unit_test() {
        let params = Form(Tutor {
            name: "Terry".to_string(),
        });
        let tera = tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter2/**/*")).unwrap();
        
        let webdata_tera = web::Data::new(tera);
        let resp = handle_post_tutor(webdata_tera, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.headers().get(CONTENT_TYPE).unwrap(), HeaderValue::from_static("text/html"));
    }

    #[actix_rt::test]
    async fn handle_post_1_integration_test() {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter2/**/*")).unwrap();
        
        /** 
         * init_service()를 사용해 테스팅을 위한 Actix Service를 만든다. 
         * 이 서브시에 HTTP 메시지를 보내서 웹서버에 요청을 보내는 웹 클라이언트를 시뮬레이션할 수 있다.
         * 일반적인 앱 빌더를 매개변수로 받으므로, 일반적인 Actix 웹 애플리케이션에서 했던 것처럼
         * Tera 인스턴스와 애플리케이션 라우트를 전달할 수 있다.
         * */ 
        let app = test::init_service(
            App::new().app_data(Data::new(tera)).configure(app_config)).await;

        /**
         * HTTP 요청 메시지는 TesrReqeust::post()를 사용해 구성된다.
         * 이를 사용해 테스트 서버에 일반적인 POST 요청을 보낸다.
         */
        let req = test::TestRequest::post()
            .uri("/tutors")
            .set_form(&Tutor {
                name: "Terry".to_string()
            })
            .to_request();
        // to_request()는 TestRequest::post() 빌더에 전달된 매개변수를 정규 포맷의 HTTP 요청 메시지로 변환한다.

        // 테스트 서버는 HTTP 요청 메시지와 함께 호출된다.
        let resp: ServiceResponse = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.headers().get(CONTENT_TYPE).unwrap(),
        HeaderValue::from_static("text/html"));
    }
}