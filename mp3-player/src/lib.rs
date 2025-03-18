use futures::TryStreamExt;
use parsing::ParseContainerBox;

mod bili_api;
mod errors;
#[allow(unused)]
mod parsing;

#[wasm_bindgen_futures::wasm_bindgen::prelude::wasm_bindgen]
pub async fn run(bvid: &str) {
    let audio_info = bili_api::audio_info(bvid).await.unwrap();
    let audio = &audio_info.audios[0];

    let stream = bili_api::fetch_m4s_chunk(&audio.base_url, audio.initialization)
        .await
        .unwrap()
        .bytes_stream()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));
    let mut stream = parsing::utils::BoxStream(tokio_util::io::StreamReader::new(stream));

    let _ = parsing::container::ContainerBox::<0>::parse(&mut stream, 0).await;
    let _ = parsing::container::ContainerBox::<0>::parse(&mut stream, 0).await;
    let _ = parsing::container::ContainerBox::<0>::parse(&mut stream, 0).await;
    let result = parsing::container::ContainerBox::<0>::parse(&mut stream, 0).await;
    loginfo(format!("{result:?}"));
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
