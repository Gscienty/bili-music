use axum::Router;
use info::get_video_info;

mod bili_api;
mod info;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/video_info/{bvid}", axum::routing::get(get_video_info));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
