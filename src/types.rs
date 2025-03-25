use argon2::{Argon2, PasswordHasher};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl Login {
    pub fn encrypt() -> Login {
        todo!()
    }
}