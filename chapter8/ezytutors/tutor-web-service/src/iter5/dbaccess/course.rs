use crate::models::course::{CreateCourse, UpdateCourse, Course};
use crate::errors::EzyTutorError;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Result<Vec<Course>, EzyTutorError> {
    // SQL 구문을 준비한다.
    let course_rows = sqlx::query_as!(
        Course,
        "SELECT * 
        FROM ezy_course_c7 
        WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_all(pool)
    .await?;

    Ok(course_rows)
}

pub async fn get_course_details_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Result<Course, EzyTutorError> {
    // SQL 구문 준비
    let course_row = sqlx::query_as!(
        Course,
        "SELECT * 
         FROM ezy_course_c7 
         WHERE tutor_id = $1 AND course_id = $2",
         tutor_id, course_id
    )
    .fetch_optional(pool)
    .await?;

    // 결과 추출
    if let Some(course) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: CreateCourse) -> Result<Course, EzyTutorError> {
    // SQL 구문 준비
    let course_row = sqlx::query_as!(
        Course,
        "INSERT INTO ezy_course_c7 (
            tutor_id, course_name, course_description, course_duration,
            course_level, course_format, course_language, course_structure,
            course_price)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning
            tutor_id, course_id, course_name, course_description,
            course_duration, course_level, course_format, course_language,
            course_structure, course_price, posted_time",
         new_course.tutor_id, new_course.course_name, 
         new_course.course_description,
         new_course.course_duration, new_course.course_level,
         new_course.course_format, new_course.course_language,
         new_course.course_structure, new_course.course_price
    )
    .fetch_one(pool)
    .await?; // ?를 사용해 에러나면 바로 결과 반환(EzyTutorError 반환)
    // posted_time은 기본값이 설정되어 있어서 따로 넣어주지 않아도 된다.

    // 결과 추출
    Ok(course_row)
}

pub async fn delete_course_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<String, EzyTutorError> {
    // SQL 구문을 준비한다.
    let course_row = sqlx::query!(
        "DELETE FROM ezy_course_c7 
        WHERE tutor_id = $1 
        AND course_id = $2",
        tutor_id,
        course_id
    )
    .execute(pool)
    .await?;

    Ok(format!("Deleted {:?} record", course_row))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
    update_course: UpdateCourse
) -> Result<Course, EzyTutorError> {
    // 현재 레코드를 얻는다.
    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * 
        FROM ezy_course_c7 
        WHERE tutor_id = $1 AND course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| EzyTutorError::NotFound("Course id not found".into()))?;

    // 업데이트를 위한 매개변수를 만든다.
    let name: String = if let Some(name) = update_course.course_name {
        name
    } else {
        current_course_row.course_name
    };
    let description: String = if let Some(desc) = update_course.course_description {
        desc
    } else {
        current_course_row.course_description.unwrap_or_default()
    };
    let format: String = if let Some(format) = update_course.course_format {
        format
    } else {
        current_course_row.course_format.unwrap_or_default()
    };
    let structure: String = if let Some(structure) = update_course.course_structure {
        structure
    } else {
        current_course_row.course_structure.unwrap_or_default()
    };
    let duration: String = if let Some(duration) = update_course.course_duration {
        duration
    } else {
        current_course_row.course_duration.unwrap_or_default()
    };
    let level: String = if let Some(level) = update_course.course_level {
        level
    } else {
        current_course_row.course_level.unwrap_or_default()
    };
    let language: String = if let Some(language) = update_course.course_language {
        language
    } else {
        current_course_row.course_language.unwrap_or_default()
    };
    let price: i32 = if let Some(price) = update_course.course_price {
        price
    } else {
        current_course_row.course_price.unwrap_or_default()
    };

    // SQL 구문을 준비한다.
    let course_row = sqlx::query_as!(
        Course,
        "UPDATE ezy_course_c7 
        SET course_name = $1, course_description = $2, course_format = $3,
        course_structure = $4, course_duration = $5, course_price = $6,
        course_language = $7, course_level = $8
        WHERE tutor_id = $9 
        AND course_id = $10 returning 
        tutor_id, course_id, course_name,
        course_description, course_duration, course_level,
        course_format, course_language, course_structure,
        course_price, posted_time
        ",
        name, description, format, structure, duration, price,
        language, level, tutor_id, course_id
    )
    .fetch_one(pool)
    .await;

    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}

