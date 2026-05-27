use chrono::Local;
use std::sync::LazyLock;

static LOG_LEVEL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("LOG")
        .unwrap_or_else(|_| "log".to_string())
        .to_lowercase()
});

pub struct Logger {
    prefix: String,
}

impl Logger {
    pub fn new(prefix: impl Into<String>) -> Self {
        Logger {
            prefix: prefix.into(),
        }
    }

    pub fn log(&self, message: &str) {
        println!("{} {message}", self.prefix_with_date());
    }

    pub fn debug(&self, message: &str) {
        if LOG_LEVEL.as_str() == "debug" {
            println!("{} [DEBUG] {message}", self.prefix_with_date());
        }
    }

    pub fn error(&self, message: &str) {
        eprintln!("{} [ERROR] {message}", self.prefix_with_date());
    }

    fn prefix_with_date(&self) -> String {
        format!(
            "[{}] {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            self.prefix
        )
    }
}
