use crate::state::AppState;
use crate::dbaccess::course::*;
use crate::errors::EzyTutorError;
use crate::models::course::{CreateCourse, UpdateCourse, Course};

use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>
) -> Result<HttpResponse, EzyTutorError> {
    // web::Path는 HTTP 요청의 경로에서 타입이 정의된 정보를 추출하는 추출자다.
    // courses/{tutor-id}로 정의되어 있고 실제 요청이 localhost:3000/courses/1로 유입되면
    // tutor-id를 1이라는 값에 매핑한다.
    // let tuple = params.0;
    let tutor_id: i32 = params.into_inner();
    get_courses_for_tutor_db(&app_state.db, tutor_id)
    .await
    .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>
) -> Result<HttpResponse, EzyTutorError> {
    // routes에서 라우트를 /{tutor_id}/{course_id}로 정의했기 때문에 아래와 같이 함
    let (tutor_id, course_id) = params.into_inner();
    get_course_details_db(&app_state.db, tutor_id, course_id)
    .await
    .map(|course| {HttpResponse::Ok().json(course)})   
}

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>
) -> Result<HttpResponse, EzyTutorError> {
    post_new_course_db(&app_state.db, new_course.into_inner())
    .await
    .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, tutor_id, course_id)
    .await
    .map(|course| HttpResponse::Ok().json(course))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();
    update_course_details_db(&app_state.db, tutor_id, course_id, update_course.into())
    .await
    .map(|course| HttpResponse::Ok().json(course))
}
#[cfg(test)]
mod test {
    // test 돌리면 테스트 코드 course_id: 3이 들어가므로 delete 시켜줄 필요가 있다.
    // docker exec -it rust_postgres psql -U truuser -d ezytutors
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        
        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = get_course_details(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore] // 테스트 안 하고 무시함
    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let new_course_msg = Course {
            course_id: 3,
            tutor_id: 1,
            course_name: "Third course".into(),
            posted_time: Some(NaiveDate::from_ymd_opt(2020, 12, 18).and_then(|date| date.and_hms_opt(05, 40, 00)).unwrap()) ,
        };
        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(course_param, app_state).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }
}