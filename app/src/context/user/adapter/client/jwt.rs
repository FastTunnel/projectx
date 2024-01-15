use async_trait::async_trait;

use domain::user::facade::IJwtFacade;

use crate::auth::Claims;
use crate::APP_STATE;

pub struct JwtFacade {}

impl JwtFacade {
    pub fn new() -> Self {
        JwtFacade {}
    }
}

#[async_trait]
impl IJwtFacade for JwtFacade {
    async fn generate_token(
        &self,
        user_id: &String,
        _username: &String,
    ) -> domain::error::Result<String> {
        let x = &unsafe {
            APP_STATE
                .keys
                .load(std::sync::atomic::Ordering::Relaxed)
                .as_ref()
                .unwrap()
        }
        .encoding;
        let claims = Claims::new(
            user_id.clone(),
            (chrono::Utc::now() + chrono::Duration::hours(5)).timestamp() as usize,
        );
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            x,
        )
        .map_err(|e| domain::error::DomainError::JwtError(e.to_string()))?;
        Ok(token)
    }
}
