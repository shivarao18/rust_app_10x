use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::response::Response;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error{
    LoginFail,

    TicketDeleteFailIdNotFound{id : u64},
}

impl IntoResponse for Error{
    fn into_response(self) -> Response{
        println!("->> {:<12} - Error: {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}