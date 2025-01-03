use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, Tutor, UpdateTutor};
use sqlx::postgres::PgPool;

pub async fn get_all_tutors_db(pool: &PgPool) ->
    Result<Vec<Tutor>, EzyTutorError> {
        // SQL 구문 준비
        let tutor_rows = sqlx::query!(
            "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile 
            FROM ezy_tutor_c6")
            .fetch_all(pool)
            .await?;

        // 결과 추출
        let tutors: Vec<Tutor> = tutor_rows
            .iter()
            .map(|tutor_row| Tutor {
                tutor_id: tutor_row.tutor_id,
                tutor_name: tutor_row.tutor_name,
                tutor_pic_url: tutor_row.tutor_pic_url,
                tutor_profile: tutor_row.tutor_profile,
            })
            .collect();
        
        match tutors.len() {
            0 => Err(EzyTutorError::NotFound("No tutors found".into())),
            1 => Ok(tutors),
        }
    }

pub async fn get_tutor_details_db(pool: &PgPool, tutor_id: i32) -> Result<Tutor, EzyTutorError> {
    // SQL 구문 준비
    let tutor_row = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile 
        FROM ezy_tutor_c6 
        WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map(|tutor_row| Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name,
        tutor_pic_url: tutor_row.tutor_pic_url,
        tutor_profile: tutor_row.tutor_profile,
    })
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;

    Ok(tutor_row)
}

pub async fn post_new_tutor_db(pool: &PgPool, new_tutor: NewTutor) -> 
Result<Tutor, EzyTutorError> {
    let tutor_row = sqlx::query!(
        "insert into ezy_tutor_c6 (
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