use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::errors::EzyTutorError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub tutor_id: i32,
    pub course_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
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

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}
/*
impl From<web::Json<CreateCourse>> for CreateCourse {
    fn from(new_course: web::Json<CreateCourse>) -> Self {
        CreateCourse {
            tutor_id: new_course.tutor_id,
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_structure: new_course.course_structure.clone(),
            course_duration: new_course.course_duration.clone(),
            course_price: new_course.course_price,
            course_language: new_course.course_language.clone(),
            course_level: new_course.course_level.clone(),
        }
    }
}
*/

impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = EzyTutorError;

    fn try_from(new_course: web::Json<CreateCourse>) -> Result<CreateCourse, EzyTutorError> {
        Ok(CreateCourse{
            tutor_id: new_course.tutor_id,
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_structure: new_course.course_structure.clone(),
            course_duration: new_course.course_duration.clone(),
            course_price: new_course.course_price,
            course_language: new_course.course_language.clone(),
            course_level: new_course.course_level.clone(),
        })
    }
}

pub struct UpdateCourse {
    pub course_name: Option<String>,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<String>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(update_course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            course_name: update_course.course_name.clone(),
            course_description: update_course.course_description.clone(),
            course_format: update_course.course_format.clone(),
            course_structure: update_course.course_structure.clone(),
            course_duration: update_course.course_duration.clone(),
            course_price: update_course.course_price.clone(),
            course_language: update_course.course_language.clone(),
            course_level: update_course.course_level.clone(),
        }
    }
}