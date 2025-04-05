use std::ops::Range;

use symphonia_core::{
    audio::Channels,
    codecs::{CODEC_TYPE_AAC, CodecParameters, Decoder, DecoderOptions},
    formats::{FormatOptions, FormatReader},
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

    let header = demux::header::MP4Header::parse(audio).await.unwrap();
    let fragment = demux::fragment::MP4Fragment::parse(&header, audio, 0)
        .await
        .unwrap();

    let Some((_sample, data)) = fragment.get_sample(1) else {
        panic!("");
    };
    crate::loginfo(format!("{:?}", data));

    let adts_header = construct_adts_header(data);
    let data = [adts_header, data.to_owned()].concat();
    let data = std::io::Cursor::new(data);

    let source = symphonia_core::io::MediaSourceStream::new(Box::new(data), Default::default());
    let mut reader =
        symphonia_codec_aac::AdtsReader::try_new(source, &FormatOptions::default()).unwrap();
    let packet = reader.next_packet().unwrap();

    let mut decoder = symphonia_codec_aac::AacDecoder::try_new(
        CodecParameters::new()
            .for_codec(CODEC_TYPE_AAC)
            .with_sample_rate(48000)
            .with_channels(Channels::FRONT_LEFT | Channels::FRONT_RIGHT),
        &DecoderOptions::default(),
    )
    .unwrap();

    let _result = decoder.decode(&packet).unwrap();

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

fn construct_adts_header(sample: &[u8]) -> Vec<u8> {
    // B: Only support 0 (MPEG-4)
    // D: Only support 1 (without CRC)
    // byte7 and byte9 not included without CRC
    let adts_header_length = 7;

    //            AAAA_AAAA
    let byte0 = 0b1111_1111;

    //            AAAA_BCCD
    let byte1 = 0b1111_0001;

    //                EEFF_FFGH
    let mut byte2 = 0b0000_0000;
    // let object_type = match track.audio_profile() {
    //     Ok(mp4::AudioObjectType::AacMain) => 1,
    //     Ok(mp4::AudioObjectType::AacLowComplexity) => 2,
    //     Ok(mp4::AudioObjectType::AacScalableSampleRate) => 3,
    //     Ok(mp4::AudioObjectType::AacLongTermPrediction) => 4,
    //     Err(_) => return None,
    // };
    let object_type = 2;
    let adts_object_type = object_type - 1;
    byte2 = (byte2 << 2) | adts_object_type; // EE

    // let sample_freq_index = match track.sample_freq_index() {
    //     Ok(mp4::SampleFreqIndex::Freq96000) => 0,
    //     Ok(mp4::SampleFreqIndex::Freq88200) => 1,
    //     Ok(mp4::SampleFreqIndex::Freq64000) => 2,
    //     Ok(mp4::SampleFreqIndex::Freq48000) => 3,
    //     Ok(mp4::SampleFreqIndex::Freq44100) => 4,
    //     Ok(mp4::SampleFreqIndex::Freq32000) => 5,
    //     Ok(mp4::SampleFreqIndex::Freq24000) => 6,
    //     Ok(mp4::SampleFreqIndex::Freq22050) => 7,
    //     Ok(mp4::SampleFreqIndex::Freq16000) => 8,
    //     Ok(mp4::SampleFreqIndex::Freq12000) => 9,
    //     Ok(mp4::SampleFreqIndex::Freq11025) => 10,
    //     Ok(mp4::SampleFreqIndex::Freq8000) => 11,
    //     Ok(mp4::SampleFreqIndex::Freq7350) => 12,
    //     // 13-14 = reserved
    //     // 15 = explicit frequency (forbidden in adts)
    //     Err(_) => return None,
    // };
    let sample_freq_index = 3;
    byte2 = (byte2 << 4) | sample_freq_index; // FFFF
    byte2 = (byte2 << 1) | 0b1; // G

    // let channel_config = match track.channel_config() {
    //     // 0 = for when channel config is sent via an inband PCE
    //     Ok(mp4::ChannelConfig::Mono) => 1,
    //     Ok(mp4::ChannelConfig::Stereo) => 2,
    //     Ok(mp4::ChannelConfig::Three) => 3,
    //     Ok(mp4::ChannelConfig::Four) => 4,
    //     Ok(mp4::ChannelConfig::Five) => 5,
    //     Ok(mp4::ChannelConfig::FiveOne) => 6,
    //     Ok(mp4::ChannelConfig::SevenOne) => 7,
    //     // 8-15 = reserved
    //     Err(_) => return None,
    // };
    let channel_config = 2;
    byte2 = (byte2 << 1) | get_bits_u8(channel_config, 6..6); // H

    // HHIJ_KLMM
    let mut byte3 = 0b0000_0000;
    byte3 = (byte3 << 2) | get_bits_u8(channel_config, 7..8); // HH
    byte3 = (byte3 << 4) | 0b1111; // IJKL

    let frame_length = adts_header_length + sample.len() as u16;
    byte3 = (byte3 << 2) | get_bits(frame_length, 3..5) as u8; // MM

    // MMMM_MMMM
    let byte4 = get_bits(frame_length, 6..13) as u8;

    // MMMO_OOOO
    let mut byte5 = 0b0000_0000;
    byte5 = (byte5 << 3) | get_bits(frame_length, 14..16) as u8;
    byte5 = (byte5 << 5) | 0b11111; // OOOOO

    // OOOO_OOPP
    let mut byte6 = 0b0000_0000;
    byte6 = (byte6 << 6) | 0b111111; // OOOOOO
    byte6 <<= 2; // PP

    vec![byte0, byte1, byte2, byte3, byte4, byte5, byte6]
}

fn get_bits_u8(byte: u8, range: Range<u8>) -> u8 {
    let shaved_left = byte << (range.start - 1);
    let moved_back = shaved_left >> (range.start - 1);

    moved_back >> (8 - range.end)
}
fn get_bits(byte: u16, range: Range<u16>) -> u16 {
    let shaved_left = byte << (range.start - 1);
    let moved_back = shaved_left >> (range.start - 1);

    moved_back >> (16 - range.end)
}
