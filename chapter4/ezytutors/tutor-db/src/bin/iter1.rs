use dotenv::dotenv;
use std::env;
use std::io;
use sqlx::postgres::PgPool;
use chrono::NaiveDateTime;

// 워크스페이스에서 실행하고 싶으면
// cargo run --bin iter -p tutor-db 라고 하면 된다.

#[derive(Debug)]
pub struct Course {
    course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

// cargo run --bin iter1
// 위 명령어 입력시 $PROJECT_ROOT/src/bin 디렉터리의 
// iter1.rs안에 있는 main 함수를 실행한다.
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 환경 변수를 메모리에 로드
    dotenv().ok();
    // 환경 변수 DATABASE_URL의 값을 꺼낸다.
    // 이 값은 셀 프롬프트 또는 .env 파일을 사용해 정의한다.
    let database_url = env::var("DATABASE_URL").expect(
        "DATABASE_URL is not set in .env file");
    // sqlx를 사용해 데이터베이스 커넥션 풀을 만든다.
    // Actix Web 프레임워크가 생성한 여러 스레드 사이의 
    // 여러 데이터베이스 커넥션을 효과적으로 관리할 수 있다.
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    // 실행할 쿼리를 정의한다.
    let course_rows = sqlx::query!(
        r#"SELECT course_id, tutor_id, course_name, posted_time FROM ezy_course_c4 where course_id = $1"#, 
        1
    )
    .fetch_all(&db_pool) // 데이터베이스 커넥션 풀의 참조를 전달해서 테이블의 모든 행을 가져온다.
    .await
    .unwrap();

    let mut courses_list = vec![];
    for course_row in course_rows {
        courses_list.push(Course{
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name,
            posted_time: Some(
                chrono::NaiveDateTime::from(course_row.posted_time.unwrap())
            ),
        })
    }

    println!("Courses = {:?}", courses_list);
    Ok(())
}