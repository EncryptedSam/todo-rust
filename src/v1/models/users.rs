use crate::schema::users;
use diesel::prelude::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Queryable, Insertable, Deserialize, Serialize, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: Option<std::time::SystemTime>, // Make sure to handle timestamps properly
}

#[derive(Queryable,Selectable, Serialize)]
#[diesel(table_name = users)]
pub struct Emp {
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct SignInData {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Validate, Insertable)]
#[diesel(table_name = users)]
pub struct  NewUser{
    #[validate(custom = "validate_username")]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom = "validate_password")]
    pub password: String,
    #[validate(custom = "validate_role")]
    pub role: String,
}

fn validate_username(username: &str) -> Result<(), validator::ValidationError> {
    if username.len() >= 3  && username.chars().all(|c| c.is_ascii_alphanumeric()) {
        Ok(())
    } else {
        let validation_error = validator::ValidationError::new("Username must contain only letters or numbers");
        Err(validation_error)
    }
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let mut upper_case = 0;
    let mut lower_case = 0;
    let mut digits = 0;
    let mut special_chars = 0;

    for c in password.chars() {
        if c.is_uppercase() {
            upper_case += 1;
        } else if c.is_lowercase() {
            lower_case += 1;
        } else if c.is_digit(10) {
            digits += 1;
        } else if "!@#$&*".contains(c) {
            special_chars += 1;
        }
    }

    if password.len() >= 8 && upper_case >= 2 && lower_case >= 3 && digits >= 2 && special_chars >= 1 {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid password format. Password should be at least 8 characters long, contain at least 2 uppercase letters, 3 lowercase letters, 2 numerals, and 1 special character (!@#$&*)."))
    }
}

fn validate_role(role: &str) -> Result<(), ValidationError> {
    match role {
        "admin" | "staff" | "client" => Ok(()),
        _ => Err(ValidationError::new("Invalid role. Only 'admin', 'staff', or 'client' roles are allowed.")),
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct JwtClaim {
    pub username: String,
    pub exp: usize
}
