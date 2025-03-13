use crate::errors;

pub async fn video_info(bvid: &str) -> Result<String, errors::Error> {
    let resp = reqwest_wasm::Client::default()
        .request(
            reqwest_wasm::Method::GET,
            "https://api.bilibili.com/x/web-interface/view",
        )
        .query(&[("bvid", bvid)])
        .fetch_mode_no_cors()
        .send()
        .await?;

    let text = resp.text().await?;

    Ok(text)
}
