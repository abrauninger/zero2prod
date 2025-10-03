use actix_web::{HttpResponse, body::to_bytes, http::StatusCode};
use sqlx::PgPool;

use crate::authentication::UserId;

use super::IdempotencyKey;

pub async fn get_saved_response(
    pool: &PgPool,
    idempotency_key: &IdempotencyKey,
    user_id: UserId,
) -> Result<Option<HttpResponse>, anyhow::Error> {
    let saved_response = sqlx::query!(
        r#"
        SELECT
            response_status_code,
            response_headers as "response_headers: Vec<HeaderPairRecord>",
            response_body
        FROM idempotency
        WHERE
            user_id = $1 AND
            idempotency_key = $2
        "#,
        *user_id,
        idempotency_key.as_ref()
    )
    .fetch_optional(pool)
    .await?;

    if let Some(r) = saved_response {
        let status_code = StatusCode::from_u16(r.response_status_code.try_into()?)?;
        let mut response = HttpResponse::build(status_code);
        for HeaderPairRecord { name, value } in r.response_headers {
            response.append_header((name, value));
        }
        Ok(Some(response.body(r.response_body)))
    } else {
        Ok(None)
    }
}

pub async fn save_response(
    pool: &PgPool,
    idempotency_key: &IdempotencyKey,
    user_id: UserId,
    http_response: &HttpResponse,
) -> Result<(), anyhow::Error> {
    let status_code = http_response.status().as_u16() as i16;
    let headers = {
        let mut h = Vec::with_capacity(http_response.headers.len());
        for (name, value) in http_response.headers().iter() {
            let name = name.as_str().to_owned();
            let value = value.as_bytes().to_owned();
            h.push(HeaderPairRecord { name, value });
        }
        h
    };
    let body = to_bytes(http_response.body()).await.unwrap();
    todo!()
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "header_pair")]
struct HeaderPairRecord {
    name: String,
    value: Vec<u8>,
}
