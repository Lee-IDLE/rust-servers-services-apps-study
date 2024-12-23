use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub tutor_id: i32,
    pub course_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

/**
 * From 트레이트는 HTTP POST 요청(새로운 강의)과 함꼐 보내진 
 * 데이터 페이로드를 추출해서 러스트 Course 데이터 구조채로 변환하기 위해 구현함
 */
impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}