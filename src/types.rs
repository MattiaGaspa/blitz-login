use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChangePassword {
    pub username: String,
    pub old_password: String,
    pub new_password: String,
}

impl Credentials {
    pub fn hash(&self) -> Credentials {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(self.password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        Credentials {
            username: self.username.clone(),
            password: password_hash,
        }
    }
}

impl ChangePassword {
    pub fn hash(&self) -> ChangePassword {
        let salt = SaltString::generate(&mut OsRng);
        let old_password_hash = Argon2::default()
            .hash_password(self.old_password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        let new_password_hash = Argon2::default()
            .hash_password(self.new_password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        ChangePassword {
            username: self.username.clone(),
            old_password: old_password_hash,
            new_password: new_password_hash,
        }
    }
}