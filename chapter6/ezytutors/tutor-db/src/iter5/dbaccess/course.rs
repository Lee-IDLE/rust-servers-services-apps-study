use crate::models::course::{CreateCourse, UpdateCourse, Course};
use crate::errors::EzyTutorError;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Result<Vec<Course>, EzyTutorError> {
    // SQL 구문을 준비한다.
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time 
         FROM ezy_course_c4 
         WHERE tutor_id = $1",
         tutor_id
    )
    .fetch_all(pool)
    .await?;
    // 결과 추출
    let courses: Vec<Course> = course_rows
    .iter()
    .map(|course_row| Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: course_row.posted_time.map(|pt| chrono::NaiveDateTime::from(pt)),
    })
    .collect();

    match courses.len() {
        0 => {
            Err(EzyTutorError::NotFound("Courses not found for tutor".into()))
        },
        _ => Ok(courses)
    }
}

pub async fn get_course_details_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Result<Course, EzyTutorError> {
    // SQL 구문 준비
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time 
         FROM ezy_course_c4
         WHERE tutor_id = $1 AND course_id = $2",
         tutor_id, course_id
    )
    .fetch_one(pool)
    .await;

    // 결과 추출
    if let Ok(course_row) = course_row { 
        Ok(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap()))
        })
    }else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: CreateCourse) -> Result<Course, EzyTutorError> {
    // SQL 구문 준비
    let course_row = sqlx::query!(
        "INSERT INTO ezy_course_c4 (tutor_id, course_id, course_name)
         VALUES ($1, $2, $3) returning
         tutor_id, course_id, course_name, posted_time",
         new_course.tutor_id, new_course.course_id, new_course.course_name
    )
    .fetch_one(pool)
    .await?; // ?를 사용해 에러나면 바로 결과 반환(EzyTutorError 반환)
    // posted_time은 기본값이 설정되어 있어서 따로 넣어주지 않아도 된다.

    // 결과 추출
    Ok(Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    })
}