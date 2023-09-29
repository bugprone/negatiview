use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub user_id: Uuid,
    pub token_uuid: Uuid,
    pub expires_in: Option<i64>,
    pub access_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub token_uuid: String,
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
}

pub fn generate_token(
    user_id: Uuid,
    ttl: i64,
    private_key: String,
) -> Result<TokenData, jsonwebtoken::errors::Error> {
    let private_key_in_bytes = general_purpose::STANDARD.decode(private_key).unwrap();
    let decoded_private_key = String::from_utf8(private_key_in_bytes).unwrap();

    let now = chrono::Utc::now();
    let mut data = TokenData {
        user_id,
        token_uuid: Uuid::new_v4(),
        expires_in: Some((now + chrono::Duration::minutes(ttl)).timestamp()),
        access_token: None,
    };

    let claims = TokenClaims {
        token_uuid: data.token_uuid.to_string(),
        sub: data.user_id.to_string(),
        exp: data.expires_in.unwrap(),
        iat: now.timestamp(),
        nbf: now.timestamp(),
    };

    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_rsa_pem(decoded_private_key.as_bytes())?,
    )?;
    data.access_token = Some(token);
    Ok(data)
}

pub fn verify_token(
    public_key: String,
    token: &str,
) -> Result<TokenData, jsonwebtoken::errors::Error> {
    let public_key_in_bytes = general_purpose::STANDARD.decode(public_key).unwrap();
    let decoded_public_key = String::from_utf8(public_key_in_bytes).unwrap();

    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

    let decoded = jsonwebtoken::decode::<TokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_rsa_pem(decoded_public_key.as_bytes())?,
        &validation
    )?;

    let user_id = uuid::Uuid::parse_str(decoded.claims.sub.as_str()).unwrap();
    let token_uuid = Uuid::parse_str(decoded.claims.token_uuid.as_str()).unwrap();

    Ok(TokenData {
        user_id,
        token_uuid,
        expires_in: None,
        access_token: None,
    })
}
