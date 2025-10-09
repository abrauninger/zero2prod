use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError, web};
use secrecy::Secret;
use sqlx::PgPool;

use crate::{
    authentication::{AuthError, Credentials, validate_credentials},
    session_state::TypedSession,
    utils::error_chain_fmt,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

#[tracing::instrument(skip(form, pool, session), fields(username=tracing::field::Empty, user_id=tracing::field::Empty))]
pub async fn login(
    form: web::Json<FormData>,
    pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, LoginError> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };

    tracing::Span::current().record("username", tracing::field::display(&credentials.username));

    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", tracing::field::display(&user_id));

            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(|e| LoginError::UnexpectedError(e.into()))?;

            Ok(HttpResponse::Ok().finish())
        }
        Err(e) => match e {
            AuthError::InvalidCredentials(_) => Err(LoginError::AuthError(e.into())),
            AuthError::UnexpectedError(_) => Err(LoginError::UnexpectedError(e.into())),
        },
    }
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),

    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl LoginError {
    fn response_builder(&self) -> HttpResponseBuilder {
        match self {
            LoginError::AuthError(_) => HttpResponse::BadRequest(),
            LoginError::UnexpectedError(_) => HttpResponse::InternalServerError(),
        }
    }
    fn error_id(&self) -> &str {
        match self {
            LoginError::AuthError(_) => "invalid_credentials",
            LoginError::UnexpectedError(_) => "internal",
        }
    }
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for LoginError {
    fn error_response(&self) -> HttpResponse {
        self.response_builder().json(serde_json::json!({
            "error_id": self.error_id()
        }))
    }
}
