use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError, web};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::{
    authentication::{AuthError, Credentials, UserId, validate_credentials},
    routes::admin::dashboard::get_username,
    utils::error_chain_fmt,
};

#[derive(serde::Deserialize)]
pub struct ChangePasswordData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    form: web::Json<ChangePasswordData>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, ChangePasswordError> {
    let user_id = user_id.into_inner();

    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        return Err(ChangePasswordError::PasswordCheckFailed);
    }

    let username = get_username(*user_id, &pool)
        .await
        .map_err(ChangePasswordError::UnexpectedError)?;

    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };

    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => Err(ChangePasswordError::CurrentPasswordIncorrect),
            AuthError::UnexpectedError(_) => Err(ChangePasswordError::UnexpectedError(e.into())),
        };
    }

    let password_minimum_length = 12;

    if form.0.new_password.expose_secret().len() < password_minimum_length {
        return Err(ChangePasswordError::NewPasswordTooShort(
            password_minimum_length,
        ));
    }

    crate::authentication::change_password(*user_id, form.0.new_password, &pool)
        .await
        .map_err(ChangePasswordError::UnexpectedError)?;

    Ok(HttpResponse::Ok().finish())
}

#[derive(thiserror::Error)]
pub enum ChangePasswordError {
    #[error("Password check failed: New passwords do not match")]
    PasswordCheckFailed,

    #[error("Current password is incorrect")]
    CurrentPasswordIncorrect,

    #[error(
        "The new password you entered is too short. The new password must be at least {0} characters long."
    )]
    NewPasswordTooShort(usize),

    #[error("Unexpected error")]
    UnexpectedError(#[from] anyhow::Error),
}

impl ChangePasswordError {
    fn response_builder(&self) -> HttpResponseBuilder {
        match self {
            ChangePasswordError::PasswordCheckFailed => HttpResponse::BadRequest(),
            ChangePasswordError::CurrentPasswordIncorrect => HttpResponse::BadRequest(),
            ChangePasswordError::NewPasswordTooShort(_) => HttpResponse::BadRequest(),
            ChangePasswordError::UnexpectedError(_) => HttpResponse::InternalServerError(),
        }
    }
    fn error_id(&self) -> &str {
        match self {
            ChangePasswordError::PasswordCheckFailed => "password_check_failed",
            ChangePasswordError::CurrentPasswordIncorrect => "current_password_incorrect",
            ChangePasswordError::NewPasswordTooShort(_) => "new_password_too_short",
            ChangePasswordError::UnexpectedError(_) => "internal_error",
        }
    }
}

impl std::fmt::Debug for ChangePasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// TODO: De-dupe with SubscribeError etc.
impl ResponseError for ChangePasswordError {
    fn error_response(&self) -> HttpResponse {
        self.response_builder().json(serde_json::json!({
            "error_id": self.error_id()
        }))
    }
}
