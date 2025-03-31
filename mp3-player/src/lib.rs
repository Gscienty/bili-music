mod bili_api;
#[allow(unused)]
mod demux;
mod errors;
#[allow(unused)]
mod parsing;

#[wasm_bindgen_futures::wasm_bindgen::prelude::wasm_bindgen]
pub async fn run(bvid: &str) {
    let audio_info = bili_api::audio_info(bvid).await.unwrap();
    let audio = &audio_info.audios[0];

    let mut stream = bili_api::fetch_m4s_chunk(&audio.base_url, audio.initialization)
        .await
        .unwrap();

    let _ = parsing::parse_box(&mut stream).await;
    let init_data = demux::init::TrackInitData::parse(&mut stream)
        .await
        .unwrap();

    let fragment_set = demux::fragment::FragmentSet::init(audio).await.unwrap();
    let fragment = fragment_set.get_fragment(0, &init_data).await.unwrap();

    crate::loginfo(format!("{fragment:?}"));

    //let buffer = Uint8Array::from(fragment.get_data()).buffer();

    // let stream = bili_api::fetch_m4s_chunk(&audio.base_url, audio.index_range)
    //     .await
    //     .unwrap()
    //     .bytes_stream()
    //     .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));
    // let mut stream = parsing::utils::BoxStream(tokio_util::io::StreamReader::new(stream));
    //
    // let result = parsing::enter::parse_box(&mut stream).await.unwrap();
    // if let mp4box::MP4Box::CompressedSegmentIndex(segment_indices) = &result {
    //     let range = segment_indices
    //         .get_range(audio.data_start_offset(), 1)
    //         .unwrap();
    //
    //     let stream = bili_api::fetch_m4s_chunk(&audio.base_url, range)
    //         .await
    //         .unwrap()
    //         .bytes_stream()
    //         .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));
    //     let mut stream = parsing::utils::BoxStream(tokio_util::io::StreamReader::new(stream));
    //
    //     let _ = parsing::enter::parse_box(&mut stream).await;
    //     let Ok(mp4box::MP4Box::MediaData(mdat)) = parsing::enter::parse_box(&mut stream).await
    //     else {
    //         return;
    //     };
    //     crate::loginfo(format!("{mdat:#?}"));
    // };
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
