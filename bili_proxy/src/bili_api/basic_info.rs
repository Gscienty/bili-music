use sonic_rs::{JsonContainerTrait, JsonValueTrait};

pub async fn get_video_info(bvid: &str) -> (String, Vec<u64>) {
    let resp = reqwest::Client::default()
        .request(
            reqwest::Method::GET,
            "https://api.bilibili.com/x/web-interface/view",
        )
        .query(&[("bvid", bvid)])
        .send()
        .await
        .unwrap();

    let content = sonic_rs::from_str::<sonic_rs::Value>(&resp.text().await.unwrap()).unwrap();

    let title = content
        .pointer(&["data", "title"])
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let cid = content
        .pointer(&["data", "pages"])
        .and_then(|v| v.as_array())
        .map(|array| {
            array
                .iter()
                .map(|elem| elem.get("cid").map(|cid| cid.as_u64()).unwrap_or_default())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
        .iter()
        .filter_map(|&v| v)
        .collect::<Vec<_>>();

    (title, cid)
}
