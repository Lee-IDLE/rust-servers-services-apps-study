use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, Tutor, UpdateTutor};
use sqlx::postgres::PgPool;

pub async fn get_all_tutors_db(pool: &PgPool) ->
    Result<Vec<Tutor>, EzyTutorError> {
        // SQL 구문 준비
        let tutor_rows = sqlx::query!(
            "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile 
            FROM ezy_tutor_c7")
            .fetch_all(pool)
            .await?;

        // 결과 추출
        let tutors: Vec<Tutor> = tutor_rows
            .iter()
            .map(|tutor_row| Tutor {
                tutor_id: tutor_row.tutor_id,
                tutor_name: tutor_row.tutor_name.clone(),
                tutor_pic_url: tutor_row.tutor_pic_url.clone(),
                tutor_profile: tutor_row.tutor_profile.clone(),
            })
            .collect();
        
        match tutors.len() {
            0 => Err(EzyTutorError::NotFound("No tutors found".into())),
            _ => Ok(tutors),
        }
    }

pub async fn get_tutor_details_db(pool: &PgPool, tutor_id: i32) -> Result<Tutor, EzyTutorError> {
    // SQL 구문 준비
    let tutor_row = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile 
        FROM ezy_tutor_c7 
        WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map(|tutor_row| Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name.clone(),
        tutor_pic_url: tutor_row.tutor_pic_url.clone(),
        tutor_profile: tutor_row.tutor_profile.clone(),
    })
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;

    Ok(tutor_row)
}

pub async fn post_new_tutor_db(pool: &PgPool, new_tutor: NewTutor) -> 
Result<Tutor, EzyTutorError> {
    let tutor_row = sqlx::query!(
        "insert into ezy_tutor_c7 (
        tutor_name, tutor_pic_url, tutor_profile
        ) values ($1, $2, $3)
        returning tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        new_tutor.tutor_name, new_tutor.tutor_pic_url, new_tutor.tutor_profile
    )
    .fetch_one(pool)
    .await?;

    // 결과를 꺼낸다.
    Ok(Tutor{
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name,
        tutor_pic_url: tutor_row.tutor_pic_url,
        tutor_profile: tutor_row.tutor_profile
    })
}

pub async fn update_tutor_details_db(pool: &PgPool, tutor_id: i32, change_tutor: UpdateTutor) ->
Result<Tutor, EzyTutorError> {
    let tutor_row = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile 
        FROM ezy_tutor_c7
        WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;

    let new_tutor_record = Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: if let Some(name) = change_tutor.tutor_name {
            name
        } else {
            tutor_row.tutor_name
        },
        tutor_pic_url: if let Some(pic_url) = change_tutor.tutor_pic_url {
            pic_url
        } else {
            tutor_row.tutor_pic_url
        },
        tutor_profile: if let Some(profile) = change_tutor.tutor_profile {
            profile
        } else {
            tutor_row.tutor_profile
        }
    };

    // SQL 구문을 준비한다.
    let tutor_updated_row = sqlx::query!(
        "UPDATE ezy_tutor_c7
        SET tutor_name = $1, tutor_pic_url = $2, tutor_profile = $3
        WHERE tutor_id = $4 returning
        tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        new_tutor_record.tutor_name,
        new_tutor_record.tutor_pic_url,
        new_tutor_record.tutor_profile,
        new_tutor_record.tutor_id,
    )
    .fetch_one(pool)
    .await
    .map(|tutor_row| Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name,
        tutor_pic_url: tutor_row.tutor_pic_url,
        tutor_profile: tutor_row.tutor_profile,
    })
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?; // ?를 사용해 에러나면 바로 결과인다.

    Ok(tutor_updated_row)
}

pub async fn delete_tutor_db(pool: &PgPool, tutor_id: i32) ->
Result<String, EzyTutorError> {
    // SQL 구문을 준비하는다.
    let tutor_row = sqlx::query!(
        "DELETE FROM ezy_tutor_c7
        WHERE tutor_id = $1",
        tutor_id
    )
    .execute(pool)
    .await
    .map_err(|_err| EzyTutorError::DBError("Unable to delete tutor".into()))?;

    Ok(format!("Deleted {:?} record", tutor_row))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, ResponseError};
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_tutors_success() {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = sqlx::postgres::PgPool::connect(&database_url).await.unwrap();
        let tutors = get_all_tutors_db(&db_pool).await.unwrap();

        match tutors.len() {
            0 => panic!("No tutors found"),
            _ => assert_eq!(tutors.len() > 0, true),
        }
    }
}