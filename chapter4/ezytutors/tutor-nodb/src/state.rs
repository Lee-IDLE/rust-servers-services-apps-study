use std::sync::Mutex;
use super::model::Course;

pub struct AppState {
    // 공유된 이뮤터블 상태
    pub health_check_response: String, 
    // 공유된 뮤터블 상태
    pub visit_count: Mutex<u32>,
    // 강의들은 Vec 컬렉션으로 애플리케이션 상태에 저장, Mutex로 보호
    pub courses: Mutex<Vec<Course>>,
}

