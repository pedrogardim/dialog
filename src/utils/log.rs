use colored::*;

enum LogLevel {
    Success,
    Info,
    Warning,
    Error,
    Debug,
    System,
}

impl LogLevel {
    fn prefix(&self) -> &'static str {
        match self {
            LogLevel::Success => "✔",
            LogLevel::Info => "ℹ",
            LogLevel::Warning => "⚠",
            LogLevel::Error => "✖",
            LogLevel::Debug => "⚒",
            LogLevel::System => "⚙",
        }
    }

    fn style_prefix(&self) -> ColoredString {
        let prefix = format!(" {} ", self.prefix());
        match self {
            LogLevel::Success => prefix.on_bright_green().bold(),
            LogLevel::Info => prefix.on_bright_blue().bold(),
            LogLevel::Warning => prefix.on_bright_yellow().bold(),
            LogLevel::Error => prefix.on_bright_red().bold(),
            LogLevel::Debug => prefix.on_bright_cyan().bold(),
            LogLevel::System => prefix.on_bright_black().bold(),
        }
    }

    fn style_message(&self, message: &str) -> ColoredString {
        match self {
            LogLevel::Success => message.bright_green().bold(),
            LogLevel::Info => message.blue().italic(),
            LogLevel::Warning => message.yellow().italic(),
            LogLevel::Error => message.bright_red().bold(),
            LogLevel::Debug => message.bright_cyan().bold(),
            LogLevel::System => message.bright_black().bold(),
        }
    }
}

fn log(message: &str, level: LogLevel) {
    let formatted_prefix = level.style_prefix();
    let formatted_message = level.style_message(message);
    println!("{} | {}", formatted_prefix, formatted_message);
}

pub fn info(message: &str) {
    log(message, LogLevel::Info);
}

pub fn success(message: &str) {
    log(message, LogLevel::Success);
}

pub fn error(message: &str) {
    log(message, LogLevel::Error);
}

pub fn warning(message: &str) {
    log(message, LogLevel::Warning);
}

pub fn debug(message: &str) {
    log(message, LogLevel::Debug);
}

pub fn system(message: &str) {
    log(message, LogLevel::System);
}
