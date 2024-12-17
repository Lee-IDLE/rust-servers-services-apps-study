use super::model::*;
use super::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);

    *visit_count += 1;
    HttpResponse::Ok().json(&response)
} // 스코프 벗어나면서 visit_count의 lock 자동 해제

/**
 * 애플리케이션 상태에 저장된 강의 컬렉션에 대한 스기 권한을 얻는다.
 * 유입되는 요청에서 데이터 페이로드를 추출한다.
 * 해당 강사의 기존 강의 숫자를 계산하고, 거기에 1을 더해서 새롱누 강의 id를 생성한다.
 * 새로운 강의 인스턴스를 만든다.
 * 새로운 강의를 AppSatet의 강의 컬렉션에 추가한다.
 * 이 함수에 대한 테스트 스크립트를 작성하자. 이 ㅡ크립트는 자동화 테스트에서 사용할 수 있다.
 */
pub async fn new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<Course>,
) -> HttpResponse {
    println!("Received new course");
    let course_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .count();
    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some(course_count_for_user as i32 + 1),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Added course")
}

/**
 * 1. AppState로부터 강의들을 얻는다.
 * 2. 요청된 tutor_id와 일치하는 강의들을 필터링한다.
 * 3. 강의 리스트를 반환한다.
 */
pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>
) -> HttpResponse{
    let tutor_id: i32 = params.into_inner();

    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == tutor_id)
        .collect::<Vec<Course>>();

    if filtered_courses.len() > 0 {
        HttpResponse::Ok().json(filtered_courses)
    } else {
        HttpResponse::Ok().json("No courses found for tutor".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_web::test]
    async fn post_course_test() {
        // 요청 데이터 페이로드, 새로운 강의 데이터 객체 생성
        let course = web::Json(Course {
            tutor_id: 1,
            course_name: "Hello, this is test course".into(),
            course_id: None,
            posted_time: None,
        });
        // 애플리케이션 상태를 나타내는 web::Data<T> 객체 생성
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        // new_course 핸들러 함수 호출하여 객체 생성
        let resp = new_course(app_state, course).await;
        // resp 객체의 상태가 Ok 값인지 확인
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_all_courses_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![])
        });
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}