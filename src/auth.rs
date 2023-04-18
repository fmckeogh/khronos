use {
    crate::{error::Error, AppState},
    axum::{
        async_trait,
        extract::{FromRef, FromRequestParts},
        http::{header::AUTHORIZATION, request::Parts},
    },
    chrono::{Duration, Utc},
    clap::ValueEnum,
    jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation},
    once_cell::sync::Lazy,
    serde::{Deserialize, Serialize},
};

/// Token validity duration
pub static TOKEN_DURATION: Lazy<Duration> = Lazy::new(|| Duration::days(31));

/// Authorization level granted by the token
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
pub enum AuthLevel {
    /// All endpoints
    Admin,
    /// Only add or update event
    User,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: i64,
    iss: String,
    level: AuthLevel,
}

impl Claims {
    fn new(level: AuthLevel) -> Self {
        let exp = (Utc::now() + *TOKEN_DURATION).timestamp();

        Self {
            exp,
            iss: "khronos".to_owned(),
            level,
        }
    }
}

/// Create a new JSON web token
pub fn create_token(
    key: &EncodingKey,
    level: AuthLevel,
) -> Result<String, jsonwebtoken::errors::Error> {
    jsonwebtoken::encode(&Header::default(), &Claims::new(level), key)
}

/// Verify an existing JSON web token, returning the user's auth level if valid
fn verify_token(token: &str, key: &DecodingKey) -> Result<AuthLevel, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.set_issuer(&["khronos"]);

    let token = jsonwebtoken::decode::<Claims>(token, key, &validation)?;

    Ok(token.claims.level)
}

async fn extract_token<S>(req: &Parts, state: &S) -> Result<AuthLevel, Error>
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    // get token from headers
    let token = if let Some(value) = req.headers.get(AUTHORIZATION) {
        // fix this
        value
            .to_str()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .to_owned()
    } else {
        return Err(Error::AuthRequired);
    };

    // get key from Axum state
    let key = AppState::from_ref(state).key;

    // verify JWT
    Ok(verify_token(&token, &key)?)
}

/// Extractor for JWT-authenticated users or admins
pub struct UserAuth;

#[async_trait]
impl<S> FromRequestParts<S> for UserAuth
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        extract_token(parts, state).await?;
        Ok(Self)
    }
}

/// Extractor for JWT-authenticated admins
pub struct AdminAuth;

#[async_trait]
impl<S> FromRequestParts<S> for AdminAuth
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match extract_token(parts, state).await? {
            AuthLevel::Admin => Ok(Self),
            got => Err(Error::InsufficientPrivilege {
                expected: AuthLevel::Admin,
                got,
            }),
        }
    }
}
