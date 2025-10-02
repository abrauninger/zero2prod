use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

use crate::{routes::get_logged_in_user_id, session_state::TypedSession, utils::see_other};

pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    get_logged_in_user_id(&session).await?;
    session.log_out();
    FlashMessage::info("You have successfully logged out.").send();
    Ok(see_other("/login"))
}
