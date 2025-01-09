use actix_web::{web, Error, HttpResponse, Result};

use super::error::EzyTutorError;

// 사용자에게 등록 폼을 표시하는 핸들러 함수
pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_username", "");
    ctx.insert("current_password", "");
    ctx.insert("current_confirmation", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");
    
    let s = tmpl
        .render("register.html", &ctx) // register.html 템플릿을 렌더링 한다.
        .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;

    // 와넌히 구성된 register.html 파일을 HTTP 응답의 일부분으로 반환한다.
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// 등록 요청을 처리하는 핸들러 함수
pub async fn handle_register() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(""))
}

