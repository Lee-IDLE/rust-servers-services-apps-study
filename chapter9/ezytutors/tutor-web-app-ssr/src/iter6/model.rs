use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorRegisterForm {
    pub username: String,
    pub password: String,
    pub confirmation: String,
    pub name: String,
    pub imageurl: String,
    pub profile: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorResponse {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String
}
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub tutor_id: Option<i32>,
    pub user_password: String,
}

// 강사들이 로그인할 수 있는 폼
#[derive(Serialize, Deserialize, Debug)]
pub struct TutorSigninForm {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCourse {
    pub course_name: String,
    pub course_description: String,
    pub course_format: String,
    pub course_duration: String,
    pub course_structure: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCourseResponse {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: String,
    pub course_format: String,
    pub course_duration: String,
    pub course_structure: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
    pub posted_time: String
}

impl From<web::Json<NewCourseResponse>> for NewCourseResponse {
    fn from(new_course: web::Json<NewCourseResponse>) -> Self {
        NewCourseResponse {
            course_id: new_course.course_id,
            tutor_id: new_course.tutor_id,
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_duration: new_course.course_duration.clone(),
            course_structure: new_course.course_structure.clone(),
            course_price: new_course.course_price,
            course_language: new_course.course_language.clone(),
            course_level: new_course.course_level.clone(),
            posted_time: new_course.posted_time.clone()
        }
    }
}

// region: HTTP PUT Update
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCourse {
    pub course_name: Option<String>,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_duration: Option<String>,
    pub course_structure: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCourseResponse {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: String,
    pub course_format: String,
    pub course_duration: String,
    pub course_structure: String,
    pub course_price: i32,
    pub course_language: String,
    pub course_level: String,
    pub posted_time: String
}

impl From<web::Json<UpdateCourseResponse>> for UpdateCourseResponse {
    fn from(new_course: web::Json<UpdateCourseResponse>) -> Self {
        UpdateCourseResponse {
            course_id: new_course.course_id,
            tutor_id: new_course.tutor_id,
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_duration: new_course.course_duration.clone(),
            course_structure: new_course.course_structure.clone(),
            course_price: new_course.course_price,
            course_language: new_course.course_language.clone(),
            course_level: new_course.course_level.clone(),
            posted_time: new_course.posted_time.clone()
        }
    }
}
// endregion: HTTP PUT Update