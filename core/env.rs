// Copyright (C) Catsh authors. All right reserved.
use std::{env, path::PathBuf};

use crate::{logs::Log, User};

#[derive(Debug)]
pub struct Env {
    name: String,
    value: String,
    user: Option<User>,
}

impl Env {
    pub fn set_var(name: &str, value: &str) -> Result<(), Log> {
        // Set env on operative system
        env::set_var(name, value);

        // Set env on catsh
        Self {
            name: String::from(name),
            value: String::from("value"),
            user: None,
        };

        Ok(())
    }
}
