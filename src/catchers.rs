use rocket::Request;
use rocket::serde::json::{Value, json};

use crate::guards::auth::authentication_guard::Token;

#[catch(401)]
pub async fn unauthorized(req: &Request<'_>) -> Value {

    let guard = req.guard::<Token>().await;

    match guard.failed() {
        Some(e) => {
            e.1
        },
        None => {
            json!({
                "status": "error",
                "message": "Unauthorized"
            })
        }
    }

}