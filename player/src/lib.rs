use bili_api::AudioInfo;
use player::AudioPlayer;

mod bili_api;
#[allow(unused)]
mod demux;
mod errors;
#[allow(unused)]
mod parsing;
mod player;
mod progress;

#[wasm_bindgen_futures::wasm_bindgen::prelude::wasm_bindgen]
pub async fn run(bvid_json: &str) {
    let bvid_list: Vec<String> = serde_json::from_str(bvid_json).unwrap();
    loop {
        for bvid in bvid_list.iter() {
            let Some(audio) = get_audio(bvid).await else {
                continue;
            };

            let mut player = AudioPlayer::new().await.unwrap();
            player.set_audio(&audio).await.unwrap();

            player.start().await;
        }
    }
}

async fn get_audio(bvid: &str) -> Option<AudioInfo> {
    let audio_info = bili_api::audio_info(bvid).await.unwrap();
    for info in audio_info.audios.iter() {
        if info.codecs == "mp4a.40.2" {
            return Some(info.clone());
        }
    }

    None
}

pub fn loginfo(log: String) {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_with_str_1(&log)
        .unwrap();
}
