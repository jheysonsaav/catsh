// Copyright (C) Catsh authors. All right reserved.
pub mod colors;
pub mod env;
pub mod logs;
pub mod dirs;

#[derive(Debug)]
pub struct User {
    pub name: Option<String>,
    pub username: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
