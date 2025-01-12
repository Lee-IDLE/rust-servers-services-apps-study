use actix_web::{web, Error, HttpResponse, Result};
use crate::iter5::dbaccess::{get_user_record, post_new_user};
use serde_json::json;
use super::{error::EzyTutorError, state::AppState};

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
// Tera 템플릿, 애플리케이션 상태, 폼 데이터를 받는다.
pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    let username = params.username.clone();
    // db에 사용자가 이미 등록되어 있는지 확인
    let user = get_user_record(&app_state.db, username).await;
    let user_not_found: bool = user.is_err();

    // 데이터에서 사용자를 찾지 못하면 비밀번호 검증을 진행한다.
    if user_not_found {
        if params.password != params.confirmation {
            ctx.insert("error", "Password do not match");
            ctx.insert("current_username", &params.username);
            ctx.insert("current_password", "");
            ctx.insert("current_confirmation", "");
            ctx.insert("current_name", &params.name);
            ctx.insert("current_imageurl", &params.current_imageurl);
            ctx.insert("current_profile", &params.current_profile);

            s = tmpl
                .render("register.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
        } else {
            let new_tutor = json!({
                "tutor_name": &params.name,
                "tutor_pic_url": &params.imageurl,
                "tutor_profile": &params.profile
            });

            let awc_client = awc::Client::default();
            // tutor-web-service에 POST 요청을 보낸다.
            let mut resp = awc_client
                .post("http://localhost:3000/tutors/")
                .send_json(&new_tutor)
                .await
                .unwrap()
                .body()
                .await?;

            // 수신한 HTTP 응답 바디에는 강사 데이터가 bytes 포맷으로 포함되어 있다.
            let tutor_response: TutorResponse = serde_json::from_str(
                &std::str::from_utf8(&resp)?)?;

            s = format!("Congratulations. 
            You have been successfully registered with 
            EzyTutor and your tutor id is: {}. 
            To start using EzyTutor, please login with your credentials.",
            tutor_response.tutor_id);

            // 비밀번호 해싱 (argon2 라이브러리 사용)
            let salt = b"somerandomsalt";
            let config = Config::default();
            let hash = argon2::hash_encoded(params.password.clone(),
                salt, &config).unwrap();

            let user = User {
                username,
                tutor_id: Some(tutor_response.tutor_id),
                user_password: hash
            };
            let _tutor_created = post_new_user(&app_state.db, user).await?;
        }
    } else {
        ctx.insert("error", "User Id already exists");
        ctx.insert("current_username", &params.username);
        ctx.insert("current_password", "");
        ctx.insert("current_confirmation", "");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    };

    Ok(HttpResponse.Ok().content_type("text/html").body(s))
}
