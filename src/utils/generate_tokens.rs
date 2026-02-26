use crate::utils::hashing_handler::hashing_handler;
use crate::utils::load_config::AppConfig;
use chrono::{Duration, Utc};
use jsonwebtoken::errors::Error as JwtError;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i64,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    // pub is_admin: Option<bool>,
    // pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct Tokens {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub one_time_password_token: Option<String>,
    pub auth_cookie: Option<String>,
}

pub async fn generate_tokens(
    token_type: &str,
    user: User,
    config: &AppConfig,
) -> Result<Tokens, JwtError> {
    let auth = config
        .auth
        .as_ref()
        .expect("AUTH CONFIGURATION IS MISSING!");

    let jwt_secret = &auth.jwt_secret;
    let access_expiry = auth.jwt_access_expiration_time_in_hours;
    let session_expiry = auth.jwt_refresh_expiration_time_in_hours;
    let otp_expiry = auth.jwt_one_time_password_lifetime_in_minutes;

    let access_token_expiration = Utc::now()
        .checked_add_signed(Duration::hours(access_expiry as i64))
        .unwrap()
        .timestamp() as usize;

    let refresh_token_expiration = Utc::now()
        .checked_add_signed(Duration::hours(session_expiry as i64))
        .unwrap()
        .timestamp() as usize;

    let otp_token_expiration = Utc::now()
        .checked_add_signed(Duration::minutes(otp_expiry as i64))
        .unwrap()
        .timestamp() as usize;

    match token_type {
        "auth" => {
            let access_claims = Claims {
                id: user.id,
                email: user.email.clone(),
                exp: access_token_expiration,
                iat: Utc::now().timestamp_millis() as usize,
            };

            let access_token = encode(
                &Header::default(),
                &access_claims,
                &EncodingKey::from_secret(jwt_secret.as_bytes()),
            )?;

            let refresh_claims = Claims {
                id: user.id,
                email: user.email.clone(),
                exp: refresh_token_expiration,
                iat: Utc::now().timestamp_millis() as usize,
            };

            let refresh_token = encode(
                &Header::default(),
                &refresh_claims,
                &EncodingKey::from_secret(jwt_secret.as_bytes()),
            )?;

            let auth_cookie_part_a = match hashing_handler(user.email.as_str()).await {
                Ok(hash) => hash.to_string(),
                Err(e) => e.to_string(),
            };

            let auth_cookie_part_b = match hashing_handler(&jwt_secret).await {
                Ok(hash) => hash.to_string(),
                Err(e) => e.to_string(),
            };

            let auth_cookie = format!(
                "rusty_chat____{ }____{ }",
                auth_cookie_part_a, auth_cookie_part_b
            );

            Ok(Tokens {
                access_token: Some(access_token),
                refresh_token: Some(refresh_token),
                one_time_password_token: None,
                auth_cookie: Some(auth_cookie),
            })
        }

        "one_time_password" => {
            let otp_claims = Claims {
                id: user.id,
                email: user.email.clone(),
                exp: otp_token_expiration,
                iat: Utc::now().timestamp_millis() as usize,
            };

            let otp_token = encode(
                &Header::default(),
                &otp_claims,
                &EncodingKey::from_secret(jwt_secret.as_bytes()),
            )?;

            Ok(Tokens {
                access_token: None,
                refresh_token: None,
                one_time_password_token: Some(otp_token),
                auth_cookie: None,
            })
        }

        _ => Ok(Tokens {
            access_token: None,
            refresh_token: None,
            one_time_password_token: None,
            auth_cookie: None,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load_config::{
        AppSection, AuthSection, ClientIntegrationsSection, ObservabilitySection,
    };

    fn mock_config() -> AppConfig {
        AppConfig {
            app: AppSection {
                name: "test_app".to_string(),
                environment: Some("test".to_string()),
            },
            client_integrations: ClientIntegrationsSection {
                allow_access_middleware: true,
                allow_sessions_middleware: true,
                allow_logging_middleware: true,
                allow_request_timeout_middleware: true,
                allow_admin_routes_protector_middleware: true,
            },
            observability: ObservabilitySection {
                enable_tracing: false,
                enable_metrics: false,
            },
            server: None,
            database: None,
            auth: Some(AuthSection {
                jwt_secret: "test_secret".to_string(),
                jwt_access_expiration_time_in_hours: 1,
                jwt_refresh_expiration_time_in_hours: 24,
                jwt_one_time_password_lifetime_in_minutes: 5,
            }),
        }
    }

    #[tokio::test]
    async fn test_generate_tokens_auth() {
        let config = mock_config();
        let user = User {
            id: 1,
            email: "test@example.com".to_string(),
        };

        let result = generate_tokens("auth", user, &config).await;
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.access_token.is_some());
        assert!(tokens.refresh_token.is_some());
        assert!(tokens.auth_cookie.is_some());
        assert!(tokens.one_time_password_token.is_none());
    }

    #[tokio::test]
    async fn test_generate_tokens_otp() {
        let config = mock_config();
        let user = User {
            id: 1,
            email: "test@example.com".to_string(),
        };

        let result = generate_tokens("one_time_password", user, &config).await;
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.access_token.is_none());
        assert!(tokens.refresh_token.is_none());
        assert!(tokens.auth_cookie.is_none());
        assert!(tokens.one_time_password_token.is_some());
    }

    #[tokio::test]
    async fn test_generate_tokens_invalid_type() {
        let config = mock_config();
        let user = User {
            id: 1,
            email: "test@example.com".to_string(),
        };

        let result = generate_tokens("invalid", user, &config).await;
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(tokens.access_token.is_none());
        assert!(tokens.refresh_token.is_none());
        assert!(tokens.auth_cookie.is_none());
        assert!(tokens.one_time_password_token.is_none());
    }
}
