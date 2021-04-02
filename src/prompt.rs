use super::utils::logs::{Log, LogLevel};
use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::dirs;

pub struct Prompt {}

impl Prompt {
    pub fn new(private: bool) {
        #[cfg(unix)]
        let history_file: String = format!(
            "{}/stellar_history",
            dirs::StellarDirs::load().config_dir().to_str().unwrap()
        );

        #[cfg(windows)]
        let history_file: String = format!(
            "{}\\stellar_history",
            dirs::stellarDirs::load().config_dir().to_str().unwrap()
        );

        let mut rl = Editor::<()>::new();

        if private == false {
            if rl.load_history(&history_file).is_err() {
                Log::new(LogLevel::Warning, 0, "No previous history.").show();
            }
        }

        loop {
            let readline = rl.readline("-| ");
            match readline {
                Ok(line) => {
                    if private == false {
                        rl.add_history_entry(line.as_str());
                    }
                    Log::new(LogLevel::Error, 1, "Command not found.").show();
                }
                Err(ReadlineError::Interrupted) => {
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    Log::new(LogLevel::Info, 130, "Bye!").show();
                    break;
                }
                Err(err) => {
                    Log::new(LogLevel::Error, 1, err.to_string().as_str());
                    break;
                }
            }
        }

        if private == false {
            rl.save_history(&history_file).unwrap();
        }
    }
}
