// Copyright (C) Catsh authors. All right reserved.
use crate::logs::{Log, LogLevel};
use directories::ProjectDirs;
use std::{fs::create_dir_all, path::PathBuf};

#[derive(Debug)]
pub struct CatshDirs {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub data_dir: PathBuf,
}

impl CatshDirs {
    pub fn load() -> Self {
        if let Some(catsh_dirs) = ProjectDirs::from("com", "catsh", "catsh") {
            Self {
                config_dir: catsh_dirs.config_dir().to_owned(),
                cache_dir: catsh_dirs.cache_dir().to_owned(),
                data_dir: catsh_dirs.data_dir().to_owned(),
            }
        } else {
            Self {
                config_dir: PathBuf::new(),
                cache_dir: PathBuf::new(),
                data_dir: PathBuf::new(),
            }
        }
    }

    pub fn verify(self) -> Self {
        if self.config_dir.exists() == false {
            match create_dir_all(self.config_dir.to_owned()) {
                Ok(_) => {
                    let _ = Log::new(LogLevel::Ok, 0, "Created directory");
                }
                Err(_) => {
                    Log::new(LogLevel::Error, 1, "Error for create directory")
                        .show();
                }
            }
        } else {
            Log::new(LogLevel::Ok, 0, "The directory exist");
        }

        if self.cache_dir.exists() == false {
            match create_dir_all(self.cache_dir.to_owned()) {
                Ok(_) => {
                    let _ = Log::new(LogLevel::Ok, 0, "Created directory");
                }
                Err(_) => {
                    Log::new(LogLevel::Error, 1, "Error for create directory")
                        .show();
                }
            }
        } else {
            Log::new(LogLevel::Ok, 0, "The directory exist");
        }

        if self.data_dir.exists() == false {
            match create_dir_all(self.data_dir.to_owned()) {
                Ok(_) => {
                    let _ = Log::new(LogLevel::Ok, 0, "Created directory");
                }
                Err(_) => {
                    Log::new(LogLevel::Error, 1, "Error for create directory")
                        .show();
                }
            }
        } else {
            Log::new(LogLevel::Ok, 0, "The directory exist");
        }
        self
    }
}
