use chrono::Local;

pub fn log(l: String) {
    println!("{}", l)
}

pub fn log_with_time(l: String) {
    let now = Local::now().to_rfc3339();
    log(format!("{} {}", now, l));
}