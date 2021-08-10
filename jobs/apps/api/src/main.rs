mod error;
mod routes;

use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{http, App, HttpServer};
use std::env;
use store::Store;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let host = env::var("HOST").unwrap_or(String::from("127.0.0.1"));
    let port = env::var("PORT")?.parse()?;
    let addr = env::var("DATABASE_URL")?;
    let allowed_origin = env::var("ALLOWED_ORIGIN")?;
    let is_secure = env::var("SECURE_COOKIE")
        .unwrap_or(String::from("false"))
        .parse()?;

    let store = Store::new(addr).await?;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .data(store.clone())
            .wrap(cors)
            .wrap(
                CookieSession::private(&[0; 32])
                    .name("session")
                    .secure(is_secure),
            )
            .service(routes::jobs_list)
            .service(routes::sign_in)
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}
