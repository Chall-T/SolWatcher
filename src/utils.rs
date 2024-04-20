use chrono::Local;

const LOG_LEVEL: &str = "LOG";

pub struct Logger {
    prefix: String,
    date_format: String,
}

impl Logger {
    // Constructor function to create a new Logger instance
    pub fn new(prefix: String) -> Self {
        Logger {
            prefix,
            date_format: String::from("%Y-%m-%d %H:%M:%S"),
        }
    }

    // Method to log a message with a prefix
    pub fn log(&self, message: String) -> String {
        let log = format!("{} {}", self.prefix_with_date(), message);
        println!("{}", log);
        log
    }
    pub fn debug(&self, message: String) -> String {
        let log = format!("{} [{}] {}", self.prefix_with_date(), "DEBUG", message);
        if LogLevel::new().is_debug() {
            println!("{}", log);
        }
        log
    }
    pub fn error(&self, message: String) -> String {
        let log = format!("{} [{}] {}", self.prefix_with_date(), "ERROR", message);
        println!("{}", log);

        log
    }
    fn prefix_with_date(&self) -> String {
        let date = Local::now();
        format!(
            "[{}] {}",
            date.format(self.date_format.as_str()),
            self.prefix
        )
    }
}

struct LogLevel<'a> {
    level: &'a str,
}
impl LogLevel<'_> {
    fn new() -> Self {
        let level = LOG_LEVEL;
        LogLevel { level }
    }
    fn is_debug(&self) -> bool {
        self.level.to_lowercase().eq("debug")
    }
}
