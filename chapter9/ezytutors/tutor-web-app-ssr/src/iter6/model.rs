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