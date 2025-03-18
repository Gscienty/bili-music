use axum::{Json, extract::Path, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::bili_api::{
    basic_info,
    stream_info::{self, StreamAudioInfo},
};

#[derive(Serialize, Deserialize)]
pub struct GetVideoInfoPathVariables {
    bvid: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetVideoInfoResponse {
    title: String,
    audios: Vec<StreamAudioInfo>,
}

pub async fn get_audio_info(Path(params): Path<GetVideoInfoPathVariables>) -> impl IntoResponse {
    let (title, cids) = basic_info::get_video_info(&params.bvid).await;
    let audios = stream_info::audio_info(&params.bvid, cids[0]).await;

    Json(GetVideoInfoResponse { title, audios })
}
