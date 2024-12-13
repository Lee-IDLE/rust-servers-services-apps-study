use std::sync::Mutex;

pub struct AppState {
    // 공유된 이뮤터블 상태
    pub health_check_response: String, 
    // 공유된 뮤터블 상태
    pub visit_count: Mutex<u32>,
}

