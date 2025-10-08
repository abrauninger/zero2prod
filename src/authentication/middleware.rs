use std::ops::Deref;

use actix_web::{
    FromRequest, HttpMessage, HttpResponse,
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};
use uuid::Uuid;

use crate::{session_state::TypedSession, utils::e500};

#[derive(Clone)]
pub struct UserId(Uuid);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for UserId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await
    }?;

    match session.get_user_id().map_err(e500)? {
        Some(user_id) => {
            req.extensions_mut().insert(UserId(user_id));
            next.call(req).await
        }
        None => {
            let (request, _payload) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(serde_json::json!({ "error_id": "not_logged_in" }))
                .map_into_boxed_body();
            Ok(ServiceResponse::new(request, response))
        }
    }
}
