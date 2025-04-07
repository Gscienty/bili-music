use symphonia_core::{
    audio::{AudioBuffer, Channels},
    codecs::{CODEC_TYPE_AAC, CodecParameters, Decoder, DecoderOptions},
    formats::Packet,
};

mod bili_api;
#[allow(unused)]
mod demux;
mod errors;
#[allow(unused)]
mod parsing;

#[wasm_bindgen_futures::wasm_bindgen::prelude::wasm_bindgen]
pub async fn run(bvid: &str) {
    let audio_info = bili_api::audio_info(bvid).await.unwrap();
    let audio = &audio_info.audios[1];

    let header = demux::header::MP4Metadata::parse(audio).await.unwrap();
    let range = &header.get_references()[0];
    let fragment = demux::fragment::MP4Fragment::parse(&header, audio, range.data_range)
        .await
        .unwrap();

    let Some((sample, data)) = fragment.get_sample(1) else {
        panic!("");
    };
    let packet = Packet::new_from_slice(0, 0, sample.duration as u64, data);

    let mut decoder = symphonia_codec_aac::AacDecoder::try_new(
        CodecParameters::new()
            .for_codec(CODEC_TYPE_AAC)
            .with_sample_rate(48000)
            .with_channels(Channels::FRONT_LEFT | Channels::FRONT_RIGHT),
        &DecoderOptions::default(),
    )
    .unwrap();

    let result = decoder.decode(&packet).unwrap();
    let mut buffer: AudioBuffer<u8> = AudioBuffer::unused();
    result.convert(&mut buffer);
    crate::loginfo(format!("{:?}", buffer.capacity()));

    // let _ = parsing::parse_box(&mut stream).await;
    // let init_data = demux::init::TrackInitData::parse(&mut stream)
    //     .await
    //     .unwrap();
    //
    // let fragment_set = demux::fragment::FragmentSet::init(audio).await.unwrap();
    // let fragment = fragment_set
    //     .get_track_fragment(0, &init_data)
    //     .await
    //     .unwrap();

    // crate::loginfo(format!("{fragment:?}"));

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
