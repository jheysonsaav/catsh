// Copyright (C) Catsh authors. All right reserved.
use directories::ProjectDirs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum DirLabel {
    ConfigDir,
    CacheDir,
    DataDir,
    DataLocalDir,
    PreferenceDir,
    RuntimeDir,
}

pub fn get(label: DirLabel) -> PathBuf {
    if let Some(dirs) = ProjectDirs::from("com", "jheysonsaav", "catsh") {
        match label {
            DirLabel::ConfigDir => return dirs.config_dir().to_owned(),
            DirLabel::CacheDir => return dirs.cache_dir().to_owned(),
            DirLabel::DataDir => return dirs.data_dir().to_owned(),
            DirLabel::DataLocalDir => return dirs.data_local_dir().to_owned(),
            DirLabel::PreferenceDir => return dirs.preference_dir().to_owned(),
            DirLabel::RuntimeDir => {
                return dirs.runtime_dir().unwrap().to_owned()
            }
        }
    } else {
        return Path::new("catsh").to_owned();
    }
}
