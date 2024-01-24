use axum::{
    http::StatusCode,
    response::{IntoResponse, Response}
};

#[derive(Debug)]
pub enum Error {
    Fail
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{self:?}");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

pub type Result<T> = core::result::Result<T, Error>;