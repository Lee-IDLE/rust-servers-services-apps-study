use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};

/**
 * 해당 핸들러가 몇 번 호출되었는지 추적하고, 이를 애플리케이션 상태로 기록한 뒤 
 * ($PROJECT_ROOT/src/iter2/state.rs), 방문 횟수를 HTTP 응답에 포함하여 반환한다.
 */
pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse{
    let health_check_response = &app_state.health_check_handler;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_course_for_tutor(
    _app_state: web::Data<AppState>,
    _params: web::Path<(i32,)>
) -> HttpResponse {
    HttpResponse::Ok().json("Success");
}

pub async fn get_course_details(
    _app_state: web::Json<AppState>,
    _params: web::Path<(i32, i32)>
) -> HttpResponse {
    HttpResponse::Ok().json("success");
}

pub async fn post_new_course(
    _new_course: web::Json<Course>,
    _app_state: web::Data<AppState>
) -> HttpResponse {
    HttpResponse::Ok().json("Success");
}