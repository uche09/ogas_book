use anyhow::Context;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::{self, TryRng};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub iss: String, // issuer
    pub aud: String, // what part of the system is it for?
    pub sub: Uuid,   // User ID
    pub sid: Uuid,
    pub exp: u64,
}

pub struct TokenService;

impl TokenService {
    pub fn create_token<T: Serialize>(
        claims: &T,
        enc_key: &[u8],
    ) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::new(Algorithm::HS256),
            claims,
            &EncodingKey::from_secret(enc_key),
        ) // pin the specific algorithm used in jwt header
    }

    pub fn verify_token<T: DeserializeOwned>(
        token: &str,
        dec_key: &[u8],
        iss: &str,
        aud: &str,
    ) -> Result<T, ApiError> {
        // Do not trust the incoming jwt header, explicitely set ours with our expected value to override it
        let mut val = Validation::new(Algorithm::HS256);
        val.set_audience(&[aud]);
        val.set_issuer(&[iss]);

        decode::<T>(token, &DecodingKey::from_secret(dec_key), &val)
            .map(|d| d.claims)
            .map_err(|_| ApiError::Unauthorized("Failed Token verification".into()))
    }

    // Using a high-entopy (highly randomized) cryptographically secure random string.
    pub fn create_refresh_token() -> Result<String, anyhow::Error> {
        let mut bytes = [0u8; 32];
        rand::rngs::SysRng
            .try_fill_bytes(&mut bytes)
            .context("Error generating random refresh token")?;

        let tokens = URL_SAFE_NO_PAD.encode(bytes);
        Ok(hex::encode(Sha256::digest(tokens.as_bytes())))
    }
}

#[cfg(test)]
mod tests {
    use super::{TokenService, UserClaims};
    use crate::error::ApiError;
    use uuid::Uuid;

    const SECRET: &[u8] = b"test-secret";

    fn claims() -> UserClaims {
        UserClaims {
            iss: "ogasbook-auth".into(),
            aud: "ogasbook-user".into(),
            sub: Uuid::new_v4(),
            sid: Uuid::new_v4(),
            exp: (chrono::Utc::now().timestamp() + 60) as u64,
        }
    }

    #[test]
    fn verifies_token_with_expected_issuer_and_audience() {
        let token = TokenService::create_token(&claims(), SECRET).unwrap();
        let verified = TokenService::verify_token::<UserClaims>(
            &token,
            SECRET,
            "ogasbook-auth",
            "ogasbook-user",
        );

        assert!(verified.is_ok());
    }

    #[test]
    fn rejects_token_with_unexpected_issuer() {
        let token = TokenService::create_token(&claims(), SECRET).unwrap();
        let result = TokenService::verify_token::<UserClaims>(
            &token,
            SECRET,
            "unexpected-issuer",
            "ogasbook-user",
        );

        assert!(matches!(result, Err(ApiError::Unauthorized(_))));
    }
}
