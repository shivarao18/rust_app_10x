use serde::Deserialize;
use tokio::net::TcpListener;
use axum::{Router, routing::get, response::Html, response::IntoResponse, extract::Query, extract::Path};
#[tokio::main]
async fn main(){
    let routes_all: Router = Router::new()
        .merge(routes_hello());
        .fallback_service(routes_static()); 

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	println!("->> LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloResponse {
    name : Option<String>,
}

async fn handler_hello(Query(params): Query<HelloResponse>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

	let name = params.name.as_deref().unwrap_or("World!");
	Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
	println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

	Html(format!("Hello2 <strong>{name}</strong>"))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}