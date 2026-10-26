mod content;
mod routes;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let state = routes::AppState {
        consultas: Default::default(),
    };

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/areas", get(routes::areas))
        .route("/estudio", get(routes::estudio))
        .route(
            "/contacto",
            get(routes::contacto_get).post(routes::contacto_post),
        )
        .route("/salud", get(routes::health))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
        .fallback(routes::not_found);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err| panic!("no se pudo escuchar en {addr}: {err}"));

    println!("{} · http://localhost:{port}", content::SITE_NAME);
    axum::serve(listener, app)
        .await
        .expect("error en el servidor");
}
