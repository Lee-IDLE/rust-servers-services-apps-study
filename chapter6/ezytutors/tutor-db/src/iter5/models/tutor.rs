use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tutor {
    tutor_id: i32,
    tutor_name: String,
    tutor_pic_url: String,
    tutor_profile: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewTutor {
    tutor_name: String,
    tutor_pic_url: String,
    tutor_profile: String
}

impl From<web::Json<NewTutor>> for NewTutor {
    fn from(new_tutor: web::Json<NewTutor>) -> Self {
        NewTutor {
            tutor_name: new_tutor.tutor_name.clone(),
            tutor_pic_url: new_tutor.tutor_pic_url.clone(),
            tutor_profile: new_tutor.tutor_profile.clone()
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTutor {
    tutor_name: Option<String>,
    tutor_pic_url: Option<String>,
    tutor_profile: Option<String>
}

impl From<web::Json<UpdateTutor>> for UpdateTutor {
    fn from(update_tutor: web::Json<UpdateTutor>) -> Self {
        UpdateTutor {
            tutor_name: update_tutor.tutor_name.clone(),
            tutor_pic_url: update_tutor.tutor_pic_url.clone(),
            tutor_profile: update_tutor.tutor_profile.clone()
        }
    }
}