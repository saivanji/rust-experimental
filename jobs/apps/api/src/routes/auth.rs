use crate::error::Result;
use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use actix_web::{route, HttpResponse};
use bcrypt::verify;
use serde::Deserialize;
use store::Store;

#[derive(Deserialize)]
pub struct SignInBody {
    username: String,
    password: String,
}

#[route("/sign-in", method = "POST")]
pub async fn sign_in(
    store: Data<Store>,
    body: Json<SignInBody>,
    session: Session,
) -> Result<HttpResponse> {
    let identity = store.get_user_hash(&body.username).await?;
    let invalid_res = HttpResponse::new(StatusCode::UNAUTHORIZED);

    let res = if let Some(identity) = identity {
        let (user_id, hash) = identity.entry();

        if verify(&body.password, &hash)? {
            session.set("user_id", user_id).unwrap();

            HttpResponse::Ok().into()
        } else {
            invalid_res
        }
    } else {
        invalid_res
    };

    Ok(res)
}
