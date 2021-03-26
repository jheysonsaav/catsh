use crate::colors;

#[derive(Debug)]
pub enum LogLevel {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct Log {
    pub level: LogLevel,
    pub code: u16,
    pub message: String,
}

impl Log {
    pub fn new(
        level: LogLevel,
        code: u16,
        message: &str,
        print: Option<bool>,
    ) -> Self {
        if print == Some(false) || print == None {
            Self {
                level,
                code,
                message: String::from(message),
            }
        } else {
            let level_value = |color: bool| {
                if color == false {
                    let level_name = match level {
                        LogLevel::Error => String::from("Error"),
                        LogLevel::Warning => String::from("Warning"),
                    };

                    return level_name;
                } else {
                    let level_name = match level {
                        LogLevel::Error => {
                            colors::red_bold("Error").to_string()
                        }
                        LogLevel::Warning => {
                            colors::yellow_bold("Warning").to_string()
                        }
                    };

                    return level_name;
                }
            };
            println!("{} {}", format!("{}:", level_value(true)), message);
            Self {
                level,
                code,
                message: String::from(message),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_log() {
        let log_new = Log::new(LogLevel::Error, 1, "message", None);

        assert_eq!(log_new.code, 1);
        assert_eq!(log_new.message, "message");
        assert_ne!(log_new.message, String::from("message"));
    }
}
