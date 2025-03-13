use axum::{extract::Path, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::bili_api::{basic_info, stream_info};

#[derive(Serialize, Deserialize)]
pub struct GetVideoInfoPathVariables {
    bvid: String,
}

pub async fn get_video_info(Path(params): Path<GetVideoInfoPathVariables>) -> impl IntoResponse {
    let (title, cids) = basic_info::get_video_info(&params.bvid).await;

    println!("{title:?} {cids:?}");

    let audios = stream_info::audio_info(&params.bvid, cids[0]).await;
    println!("{audios:?}");

    (axum::http::StatusCode::OK, "").into_response()
}
