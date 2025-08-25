use crate::{Error, Result};
use serde::Deserialize;
use axum::Json;
use serde_json::{json, Value};
use axum::routing::post;
use axum::Router;
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
struct LoginPayLoad{
    username: String,
    pwd: String,
}

pub fn routes() -> Router{
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload : Json<LoginPayLoad>) -> Result<Json<Value>>{
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "testuser" || payload.pwd != "testpwd"{
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)

}
