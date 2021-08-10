use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use bcrypt::BcryptError;
use derive_more::Display;
use store::StoreError;

#[derive(Debug, Display)]
pub struct WebError {
    status_code: StatusCode,
}

impl std::error::Error for WebError {}

impl ResponseError for WebError {
    // fn error_response(&self) -> Response<Body> {}
    fn status_code(&self) -> StatusCode {
        self.status_code
    }
}

impl From<StoreError> for WebError {
    fn from(_err: StoreError) -> Self {
        Self {
            status_code: StatusCode::from_u16(500).unwrap(),
        }
    }
}

impl From<BcryptError> for WebError {
    fn from(_err: BcryptError) -> Self {
        Self {
            status_code: StatusCode::from_u16(500).unwrap(),
        }
    }
}

// impl From<actix_web::dev::HttpResponseBuilder> for WebError {
//     fn from(builder: actix_web::dev::HttpResponseBuilder) -> WebError {}
// }

pub type Result<T> = std::result::Result<T, WebError>;
