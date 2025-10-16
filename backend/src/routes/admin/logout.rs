use actix_web::HttpResponse;

use crate::session_state::TypedSession;

pub async fn log_out(session: TypedSession) -> HttpResponse {
    session.log_out();
    HttpResponse::Ok().finish()
}
