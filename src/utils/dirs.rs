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

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_configdir_load() {
        let dirs = StellarDirs::load();

        // ConfigDir
        #[cfg(target_os = "linux")]
        assert_eq!(
            dirs.config_dir.to_str().unwrap(),
            format!("{}/.config/stellar", env::var("HOME").unwrap()).as_str()
        );

        #[cfg(target_os = "windows")]
        assert_eq!(
            dirs.config_dir.to_str().unwrap(),
            format!(
                "{}\\stellar\\stellar\\config",
                env::var("APPDATA").unwrap()
            )
            .as_str()
        );

        #[cfg(target_os = "macos")]
        assert_eq!(
            dirs.config_dir.to_str().unwrap(),
            format!(
                "{}/Library/Application Support/com.stellar.stellar",
                env::var("HOME").unwrap()
            )
            .as_str()
        );
    }

    #[test]
    fn test_datadir_load() {
        let dirs = StellarDirs::load();

        // DataDir
        #[cfg(target_os = "linux")]
        assert_eq!(
            dirs.data_dir.to_str().unwrap(),
            format!("{}/.local/share/stellar", env::var("HOME").unwrap())
                .as_str()
        );

        #[cfg(target_os = "windows")]
        assert_eq!(
            dirs.data_dir.to_str().unwrap(),
            format!("{}\\stellar\\stellar\\data", env::var("APPDATA").unwrap())
                .as_str()
        );

        #[cfg(target_os = "macos")]
        assert_eq!(
            dirs.data_dir.to_str().unwrap(),
            format!(
                "{}/Library/Application Support/com.stellar.stellar",
                env::var("HOME").unwrap()
            )
            .as_str()
        );
    }

    #[test]
    fn test_cachedir_load() {
        let dirs = StellarDirs::load();

        // CacheDir
        #[cfg(target_os = "linux")]
        assert_eq!(
            dirs.cache_dir.to_str().unwrap(),
            format!("{}/.cache/stellar", env::var("HOME").unwrap()).as_str()
        );

        #[cfg(target_os = "windows")]
        assert_eq!(
            dirs.cache_dir.to_str().unwrap(),
            format!(
                "{}\\stellar\\stellar\\cache",
                env::var("APPDATA").unwrap()
            )
            .as_str()
        );

        #[cfg(target_os = "macos")]
        assert_eq!(
            dirs.cache_dir.to_str().unwrap(),
            format!(
                "{}/Library/Caches/com.stellar.stellar",
                env::var("HOME").unwrap()
            )
            .as_str()
        );
    }
}
