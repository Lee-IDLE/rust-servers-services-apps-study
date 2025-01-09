use actix_web::{Error, HttpResponse, Result};

// 사용자에게 등록 폼을 표시하는 핸들러 함수
pub async fn show_register_form() -> Result<HttpResponse, Error> {
    let msg = "Hello, you are in the registration page";
    Ok(HttpResponse::Ok().body(msg))
}

// 등록 요청을 처리하는 핸들러 함수
pub async fn handle_register() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(""))
}

