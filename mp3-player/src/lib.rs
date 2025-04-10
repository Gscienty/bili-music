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
mod progress;

#[wasm_bindgen_futures::wasm_bindgen::prelude::wasm_bindgen]
pub async fn run(bvid: &str) {
    let audio_info = bili_api::audio_info(bvid).await.unwrap();
    let audio = &audio_info.audios[1];
    let mut decoder = symphonia_codec_aac::AacDecoder::try_new(
        CodecParameters::new()
            .for_codec(CODEC_TYPE_AAC)
            .with_sample_rate(48000)
            .with_channels(Channels::FRONT_LEFT | Channels::FRONT_RIGHT),
        &DecoderOptions::default(),
    )
    .unwrap();

    let mut prog = progress::Progress::new(audio).await.unwrap();

    // let mut samples = VecDeque::new();
    let mut left_samples = Vec::new();
    let mut right_samples = Vec::new();
    while let Some((sample_meta, data)) = prog.next_sample().await {
        if sample_meta.duration != 1024 {
            // TODO acc decode bug, duration must == 1024
            break;
        }
        let packet = Packet::new_from_slice(0, 0, sample_meta.duration as u64, data);

        let result = decoder.decode(&packet).unwrap();
        let mut sample = SampleBuffer::<f32>::new(sample_meta.duration as u64, *result.spec());
        sample.copy_interleaved_ref(result);

        for (index, &sample) in sample.samples().iter().enumerate() {
            if index % 2 == 0 {
                left_samples.push(sample);
            } else {
                right_samples.push(sample);
            }
        }
    }
    crate::loginfo("decoded".to_string());

    let ctx = AudioContext::new().unwrap();

    let opt = AudioBufferOptions::new(left_samples.len() as u32, 48000.);
    opt.set_number_of_channels(2);
    let audio_buffer = AudioBuffer::new(&opt).unwrap();
    audio_buffer.copy_to_channel(&left_samples, 0).unwrap();
    audio_buffer.copy_to_channel(&right_samples, 1).unwrap();

    let opt = GainOptions::new();
    opt.set_gain(0.3);
    let gain = GainNode::new_with_options(&ctx, &opt).unwrap();

    let source = ctx.create_buffer_source().unwrap();
    source.set_buffer(Some(&audio_buffer));

    source
        .connect_with_audio_node(&gain)
        .unwrap()
        .connect_with_audio_node(&ctx.destination())
        .unwrap();
    let err = source.start().err();
    crate::loginfo(format!("{err:?}"));
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
