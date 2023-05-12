/// 去除字符串前方多余空格
#[allow(dead_code)]
pub fn remove_space(s: &str) -> String {
    s.trim_start_matches(' ').trim_end_matches(' ').to_string()
}

use chrono::Local;

#[allow(dead_code)]
pub fn timestamp() -> i64 {
    let time = Local::now();
    time.timestamp()
}
