// Copyright (C) Catsh authors. All right reserved.
pub mod colors;
pub mod dirs;
pub mod env;
pub mod logs;

#[derive(Debug)]
pub struct User {
    pub name: Option<String>,
    pub username: String,
}
