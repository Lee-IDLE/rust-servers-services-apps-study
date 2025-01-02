use crate::dbaccess::tutor::*;
use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, UpdateTutor};
use crate::state::AppState;

use actix_web::{web, HttpResponse};

pub async fn get_all_tutors(app_state: web::Data<AppState>) -> 
    Result<HttpResponse, EzyTutorError> {
    get_all_tutors_db(&app_state.db)
    .await
    .map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn get_tutor_details(
    appState: web::Data<AppState>,
    web::Path(tutor_id): web::Path<i32>
) -> Result<HttpResponse, EzyTutorError> {
    get_tutor_details_db(&appState.db, tutor_id)
    .await
    .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn post_new_tutor(
    new_tutor: web::Json<NewTutor>,
    app_state: web::Data<AppState>
) -> Result<HttpResponse, EzyTutorError> {
    post_new_tutor(&app_state.db, NewTutor::from(new_tutor))
    .await
    .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    web::Path(tutor_id): web::Path<i32>,
    update_tutor: web::Json<UpdateTutor>
) -> Result<HttpResponse, EzyTutorError> {
    update_tutor_details_db(&app_state.db, tutor_id,
        UpdateTutor::from(update_tutor))
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn delete_tutor(
    app_state: web::Data<AppState>,
    web::Path(tutor_id): web::Path<i32>
) -> Result<HttpResponse, EzyTutorError> {
    delete_tutor_db(&app_state.db, tutor_id)
    .await
    .map(|tutor| HttpResponse::Ok().json(tutor))
}