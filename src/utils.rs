use crate::v1::models::users::JwtClaim;
use argon2::{password_hash::PasswordHash, password_hash::SaltString, Argon2, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{encode, Header,decode, EncodingKey, DecodingKey, Validation};
use std::env;
use rand;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn hash_password(password: &str) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut rand::thread_rng());
    let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
    hash.to_string()
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).unwrap();
    argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

pub fn generate_jwt_token(username: &String) -> Result<String, jsonwebtoken::errors::Error> {
        // Set expiration to a distant future time (e.g., 100 years from now)
        let expiration_time = SystemTime::now()
        .checked_add(std::time::Duration::from_secs(100 * 365 * 24 * 60 * 60)) // 100 years
        .unwrap_or_else(|| {
            // Fallback if addition overflows
            SystemTime::UNIX_EPOCH
        });

    let expiration_secs = expiration_time
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();


    let my_claims = JwtClaim{
        username: (&username).to_string(),
        exp: expiration_secs as usize
    };

    // Load the secret key from an environment variable
    let secret = env::var("JWT_SECRET_KEY").expect("SECRET_KEY not found in environment");
    let secret_bytes = secret.as_bytes();

    // Encode the JWT token with your claims
    let encoding_key = EncodingKey::from_secret(secret_bytes);
    let token = encode(&Header::default(),&my_claims,&encoding_key)?;
    Ok(token)
}

pub fn decode_jwt_token(token: &str) -> Result<JwtClaim, jsonwebtoken::errors::Error> {
    // Load the secret key from an environment variable or a secure location
    let secret = env::var("JWT_SECRET_KEY").expect("SECRET_KEY not found in environment");
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    // Decode the JWT token and validate its signature
    let token_data = decode::<JwtClaim>(token, &decoding_key, &Validation::default())?;
    
    // Extract and return the token's claims
    Ok(token_data.claims)
}