// Copyright (C) Stellar authors. All right reserved.
use crate::utils::logs::{Log, LogLevel};
use directories::ProjectDirs;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct StellarDirs {
    config_dir: PathBuf,
    cache_dir: PathBuf,
    data_dir: PathBuf,
}

impl StellarDirs {
    pub fn load() -> Self {
        if let Some(stellar_dirs) =
            ProjectDirs::from("com", "stellar", "stellar")
        {
            Self {
                config_dir: stellar_dirs.config_dir().to_owned(),
                cache_dir: stellar_dirs.cache_dir().to_owned(),
                data_dir: stellar_dirs.data_dir().to_owned(),
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

    pub fn config_dir(&self) -> &Path {
        self.config_dir.as_path()
    }

    pub fn cache_dir(&self) -> &Path {
        self.cache_dir.as_path()
    }

    pub fn data_dir(&self) -> &Path {
        self.data_dir.as_path()
    }
}
