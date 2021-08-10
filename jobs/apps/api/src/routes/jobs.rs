use crate::error::Result;
use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Query};
use actix_web::{route, HttpResponse};
use serde::Deserialize;
use store::Store;

#[derive(Deserialize)]
pub struct JobsParams {
    limit: i64,
    offset: i64,
}

#[route("/jobs", method = "GET")]
pub async fn jobs_list(
    store: Data<Store>,
    params: Query<JobsParams>,
    session: Session,
) -> Result<HttpResponse> {
    if session.get::<i32>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::new(StatusCode::UNAUTHORIZED));
    }

    let jobs = store.list_jobs(params.limit, params.offset).await?;

    Ok(HttpResponse::Ok().json(jobs))
}
