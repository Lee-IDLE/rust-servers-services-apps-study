use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse{
    error_message: String
}

impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            },
            EzyTutorError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            },
            EzyTutorError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            },
        }
    }
}

// EzyTutorError를 사용자에게 보낼 수 있는 문자열로 출력할 수 있도록 한다.
impl fmt::Display for EzyTutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl error::ResponseError for EzyTutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            EzyTutorError::DBError(_msg) | EzyTutorError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzyTutorError::NotFound(_msg) => StatusCode::NOT_FOUND
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}


// 물음표 연산자를 사용해 Actix Web 에러를 EzyTutorError로 변환할 수 있도록 한다.
impl From<actix_web::error::Error> for EzyTutorError {
    // actix_web::error::Error에러가 발생하면 아래 함수가 실행되어 EzyTutorError::ActixError로 변환된다.
    fn from(error: actix_web::error::Error) -> Self {
        EzyTutorError::ActixError(error.to_string())
    }
}

// 물음표 연산자를 사용해 데이터베이스 에러를 EzyTutorError로 변환할 수 있도록 한다.
impl From<SQLxError> for EzyTutorError {
    // SQLxError에러가 발생하면 아래 함수가 실행되어 EzyTutorError::DBError로 변환된다.
    fn from(error: SQLxError) -> Self {
        EzyTutorError::DBError(error.to_string())
    }
}