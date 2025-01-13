use crate::iter6::errors::EzyTutorError;
use crate::iter6::model::*;
use sqlx::postgres::PgPool;

// 결과를 반환한다
pub async fn get_user_record(pool: &PgPool, username: String) 
-> Result<User, EzyTutorError> {
    // SQL 쿼리를 준비한다.
    let user_row = sqlx::query_as!(
        User,
        "SELECT * 
        FROM ezyweb_user 
        WHERE username = $1",
        username
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = user_row {
        Ok(user) 
    } else {
        // &str을 String 형으로 변환하기 위해 into() 사용
        Err(EzyTutorError::NotFound("User name not found".into()))
    }
}

pub async fn post_new_user(pool: &PgPool, new_user: User) 
-> Result<User, EzyTutorError> {
    let user_row = sqlx::query_as!(
        User,
        "INSERT INTO ezyweb_user (
            username, tutor_id, user_password
        ) VALUES (
            $1, $2, $3
        ) returning username, tutor_id, user_password
        ",
        new_user.username,
        new_user.tutor_id,
        new_user.user_password
    )
    .fetch_one(pool)
    .await?;

    Ok(user_row)
}



