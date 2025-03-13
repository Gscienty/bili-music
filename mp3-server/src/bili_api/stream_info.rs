use sonic_rs::{JsonContainerTrait, JsonValueTrait};

#[derive(Debug, Clone)]
pub struct StreamAudioInfo {
    pub base_url: String,
    pub bandwidth: usize,
    pub mime_type: String,
    pub codecs: String,
    pub initialization: (usize, usize),
    pub index_range: (usize, usize),
}

impl StreamAudioInfo {
    fn new(audio: &sonic_rs::Value) -> Self {
        let base_url = audio
            .get("base_url")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let bandwidth = audio
            .get("bandwidth")
            .and_then(|v| v.as_u64())
            .unwrap_or_default() as usize;

        let mime_type = audio
            .get("mime_type")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let codecs = audio
            .get("codecs")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let initialization = audio
            .pointer(&["segment_base", "initialization"])
            .and_then(|v| v.as_str())
            .and_then(|v| {
                let pat = v.splitn(2, '-').collect::<Vec<&str>>();
                if pat.len() != 2 {
                    None
                } else {
                    let begin = pat[0].parse::<usize>().unwrap_or_default();
                    let end = pat[1].parse::<usize>().unwrap_or_default();

                    Some((begin, end))
                }
            })
            .unwrap_or_default();

        let index_range = audio
            .pointer(&["segment_base", "index_range"])
            .and_then(|v| v.as_str())
            .and_then(|v| {
                let pat = v.splitn(2, '-').collect::<Vec<&str>>();
                if pat.len() != 2 {
                    None
                } else {
                    let begin = pat[0].parse::<usize>().unwrap_or_default();
                    let end = pat[1].parse::<usize>().unwrap_or_default();

                    Some((begin, end))
                }
            })
            .unwrap_or_default();

        Self {
            base_url,
            bandwidth,
            mime_type,
            codecs,
            index_range,
            initialization,
        }
    }
}

pub async fn audio_info(bvid: &str, cid: u64) -> Vec<StreamAudioInfo> {
    let resp = reqwest::Client::default()
        .request(
            reqwest::Method::GET,
            "https://api.bilibili.com/x/player/wbi/playurl",
        )
        .query(&[
            ("bvid", bvid),
            ("cid", &cid.to_string()),
            ("fnver", "0"),
            ("fnval", "4048"),
            ("fourk", "1"),
        ])
        .header(reqwest::header::USER_AGENT, "Mozilla")
        .header(reqwest::header::REFERER, "https://www.bilibili.com")
        .send()
        .await
        .unwrap();

    let content = sonic_rs::from_str::<sonic_rs::Value>(&resp.text().await.unwrap()).unwrap();
    let Some(dash) = content.pointer(&["data", "dash"]) else {
        return Vec::new();
    };
    let Some(audios) = dash.get("audio").and_then(|v| v.as_array()) else {
        return Vec::new();
    };

    audios.iter().map(StreamAudioInfo::new).collect()
}
