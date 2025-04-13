use player::AudioPlayer;
use symphonia_core::{
    audio::{Channels, SampleBuffer},
    codecs::{CODEC_TYPE_AAC, CodecParameters, Decoder, DecoderOptions},
    formats::Packet,
};
use web_sys::{AudioBuffer, AudioBufferOptions, AudioContext, GainNode, GainOptions};

mod bili_api;
#[allow(unused)]
mod demux;
mod errors;
#[allow(unused)]
mod parsing;
mod player;
mod progress;

#[wasm_bindgen_futures::wasm_bindgen::prelude::wasm_bindgen]
pub async fn run(bvid: &str) {
    let audio_info = bili_api::audio_info(bvid).await.unwrap();
    let audio = &audio_info.audios[1];

    let mut player = AudioPlayer::new().await.unwrap();
    player.set_audio(audio).await.unwrap();
    player.set_loop(true);

    player.start().await;

    crate::loginfo("done".to_string());
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
