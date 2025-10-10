use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError, http::header::LOCATION};

/// Return an opaque 500 while preserving the error's root cause for logging.
pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

/// Return a 400 with the user representation of the validation error as the body.
/// The error root cause is preserved for logging purposes.
pub fn e400<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorBadRequest(e)
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{e}\n")?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{cause}")?;
        current = cause.source();
    }
    Ok(())
}

/// A simple wrapper around anyhow::Error that implements ResponseError to return the `error_id`.
#[derive(thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl AppError {
    fn response_builder(&self) -> HttpResponseBuilder {
        match self {
            AppError::UnexpectedError(_) => HttpResponse::InternalServerError(),
        }
    }
    fn error_id(&self) -> &str {
        match self {
            AppError::UnexpectedError(_) => "internal_error",
        }
    }
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// TODO: De-dupe with SubscribeError etc.
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        self.response_builder().json(serde_json::json!({
            "error_id": self.error_id()
        }))
    }
}
