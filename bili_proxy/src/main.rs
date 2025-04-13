use axum::Router;
use info::get_audio_info;
use tower_http::services::{ServeDir, ServeFile};

mod bili_api;
mod info;
mod m4s_chunk;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/audio_info/{bvid}", axum::routing::get(get_audio_info))
        .route("/audio_chunk", axum::routing::post(m4s_chunk::m4s_chunk))
        .route_service("/", ServeFile::new("../player/index.html"))
        .nest_service("/pkg", ServeDir::new("../player/pkg"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
