use super::models::Course;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    // SQL 구문을 준비한다.
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time 
         FROM ezy_course_c4 
         WHERE tutor_id = $s",
         tutor_id
    )
    .fetch_all(pool)
    .await
    .unwrap();
    // 결과 추출
    course_rows
    .iter()
    .map(|course_row| Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    })
    .collect()
}

pub async fn get_course_details_db(pool: &PgPool, course_id: i32) -> Course {
    // SQL 구문 준비
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time 
         FROM ezy_course_c4
         WHERE tutor_id = $1 AND course_id = $2",
         tutor_id, course_id
    )
    .fetch_one(pool)
    .await
    .unwrap();
    // 결과 추출
    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    // SQL 구문 준비
    let course_row = sqlx::query!(
        "INSERT INTO ezy_course_c4 (tutor_id, course_id, course_name)
         VALUES ($1, $2, $3) returning
         tutor_id, course_id, course_name, posted_time",
         new_course.tutor_id, new_course.course_id, new_course.course_name
    )
    .fetch_one(pool)
    .await
    .unwrap();
    // posted_time은 기본값이 설정되어 있어서 따로 넣어주지 않아도 된다.

    // 결과 추출
    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    }
}