pub type Result<T> = core::result::Result<T, Error>;

#[derive(debug)]
pub enum Errro{
    LoginFail,
}

impl IntoResponse for Error{
    fn into_response(self) -> Response{
        println!("->> {:<12} - Error: {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}