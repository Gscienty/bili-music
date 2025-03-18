use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::bili_api::stream_info;

#[derive(Serialize, Deserialize)]
pub struct M4SChunkRequest {
    url: String,
    range_start: usize,
    range_end: usize,
}

pub async fn m4s_chunk(Json(request): Json<M4SChunkRequest>) -> impl IntoResponse {
    stream_info::get_m4s_file_chunk(&request.url, (request.range_start, request.range_end)).await
}
