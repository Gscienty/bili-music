use crate::errors;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamAudioInfo {
    pub base_url: String,
    pub bandwidth: usize,
    pub mime_type: String,
    pub codecs: String,
    pub initialization: (usize, usize),
    pub index_range: (usize, usize),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVideoInfoResponse {
    pub title: String,
    pub audios: Vec<StreamAudioInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct M4SChunkRequest {
    url: String,
    range_start: usize,
    range_end: usize,
}

pub async fn audio_info(bvid: &str) -> Result<GetVideoInfoResponse, errors::Error> {
    let resp = reqwest_wasm::Client::default()
        .request(
            reqwest_wasm::Method::GET,
            format!("http://localhost:8000/audio_info/{bvid}"),
        )
        .send()
        .await?;

    let text = resp.text().await?;

    serde_json::from_str(&text).map_err(|err| errors::Error::IOError(err.to_string()))
}

pub async fn fetch_m4s_chunk(
    url: &str,
    range: (usize, usize),
) -> Result<reqwest_wasm::Response, errors::Error> {
    let resp = reqwest_wasm::Client::default()
        .request(
            reqwest_wasm::Method::POST,
            "http://localhost:8000/audio_chunk",
        )
        .header(reqwest_wasm::header::CONTENT_TYPE, "application/json")
        .body(
            serde_json::to_string(&M4SChunkRequest {
                url: url.to_owned(),
                range_start: range.0,
                range_end: range.1,
            })
            .unwrap(),
        )
        .send()
        .await?;

    Ok(resp)
}
