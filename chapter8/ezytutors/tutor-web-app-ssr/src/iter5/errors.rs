use actix_web::{error, http::StatusCode, HttpResponse, ResponseError, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

// 커스텀 에러 타입 정의
#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    TeraError(String)
}

// 응답을 사용자에게 반환한다.
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String
}

// 커스텀 에러 타입을 위한 러스트의 표준 에러 트레이트를 구현한다.
// 이를 통해 Actix를 사용해 커스텀 에러 타입을 HTTP 응답으로 변환할 수 있다.
impl std::error::Error for EzyTutorError {}

/**
 * 튜터 웹 애플리케이션에서 발생할 수 있는 다양한 에러타입에 대한 (사용자로의)
 * 에러 응답 메시지를 만든다.
 */ 
impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            EzyTutorError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            EzyTutorError::TeraError(msg) => {
                println!("Error in rendering the template {:?}", msg);
                msg.into()
            }
            EzyTutorError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

/**
 * Actix의 ResponseError 트레이트를 구현한다.
 * 이것은 EzyTutorError를 HTTP 응답으로 변환하는 방법을 지정한다.
 */
impl error::ResponseError for EzyTutorError  {
    fn status_code(&self) -> StatusCode {
        match self {
            EzyTutorError::DBError(_msg) |
            EzyTutorError::ActixError(_msg) |
            EzyTutorError::TeraError(_msg) => 
                StatusCode::INTERNAL_SERVER_ERROR,
            EzyTutorError::NotFound(_msg) => 
                StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

/**
 * 러스트 표준 라이브러리로부터 Display 트레이트를 구현한다.
 * 이는 EzyTutorError에 대해 에러를 출력할 수 있도록 하낟.
 */
impl fmt::Display for EzyTutorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

/**
 * Actix Web의 Error 트레이트를 구현한다. EzyTutorError는 이를 통해
 * ? 연산자를 사용해 전자를 후자로 변환할 수 있다.
 */
impl From<actix_web::error::Error> for EzyTutorError {
    fn from(err: actix_web::error::Error) -> Self {
        EzyTutorError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for EzyTutorError {
    fn from(err: SQLxError) -> Self {
        EzyTutorError::DBError(err.to_string())
    }
}