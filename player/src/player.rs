use crate::{bili_api::AudioInfo, demux, errors};
use std::{collections::HashMap, sync::Arc};
use symphonia_core::{
    audio::{Channels, SampleBuffer},
    codecs::{CODEC_TYPE_AAC, CodecParameters, Decoder, DecoderOptions},
    formats::Packet,
};
use tokio_with_wasm::alias::sync::{
    RwLock,
    mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{AudioBuffer, AudioBufferOptions, AudioContext, GainNode, GainOptions};

enum AudioPlayerEvent {
    DownloadedFragment(usize),
    Suspend,
}

struct DownloadFragmentEvent {
    metadata: Arc<AudioPlayerMetadata>,
    cache: Arc<RwLock<HashMap<usize, Vec<SampleBuffer<f32>>>>>,
    index: usize,
}

enum AudioFragmentDownloaderEvent {
    DownloadFragment(DownloadFragmentEvent),
}

struct AudioPlayerMetadata {
    audio_info: AudioInfo,
    metadata: demux::metadata::MP4Metadata,
}

type CacheType = Arc<RwLock<HashMap<usize, Vec<SampleBuffer<f32>>>>>;

pub struct AudioPlayer {
    ctx: AudioContext,
    metadata: Option<Arc<AudioPlayerMetadata>>,

    cache: CacheType,

    play_fragment_cursor: usize,
    play_sample_cursor: usize,
    suspend: bool,
    is_loop: bool,

    send: UnboundedSender<AudioPlayerEvent>,
    recv: UnboundedReceiver<AudioPlayerEvent>,
}

impl AudioPlayer {
    pub async fn new() -> Result<Self, errors::Error> {
        let (send, recv) = unbounded_channel::<AudioPlayerEvent>();
        let ctx = AudioContext::new().unwrap();

        Ok(Self {
            ctx,
            metadata: None,

            cache: Arc::new(RwLock::new(HashMap::new())),

            play_fragment_cursor: 0,
            play_sample_cursor: 0,
            suspend: true,
            is_loop: false,

            send,
            recv,
        })
    }

    pub async fn set_audio(&mut self, audio_info: &AudioInfo) -> Result<(), errors::Error> {
        let metadata = demux::metadata::MP4Metadata::parse(audio_info).await?;
        self.metadata = Some(Arc::new(AudioPlayerMetadata {
            audio_info: audio_info.clone(),
            metadata,
        }));

        self.cache.write().await.clear();

        self.play_sample_cursor = 0;
        self.play_sample_cursor = 0;
        self.suspend = true;

        Ok(())
    }

    pub fn set_loop(&mut self, is_loop: bool) {
        self.is_loop = is_loop;
    }

    pub async fn start(&mut self) {
        let Some(metadata) = &self.metadata else {
            return;
        };

        let decoder = symphonia_codec_aac::AacDecoder::try_new(
            CodecParameters::new()
                .for_codec(CODEC_TYPE_AAC)
                .with_sample_rate(metadata.metadata.sample_rate())
                .with_channels(Channels::FRONT_LEFT | Channels::FRONT_RIGHT),
            &DecoderOptions::default(),
        )
        .unwrap();
        let (fragment_downloader_send, fragment_downloader_recv) =
            unbounded_channel::<AudioFragmentDownloaderEvent>();
        let fragment_downloader_send_event = self.send.clone();
        tokio_with_wasm::alias::spawn(async move {
            Self::download_fragment_thread_main(
                decoder,
                fragment_downloader_send_event,
                fragment_downloader_recv,
            )
            .await;
        });
        self.send_download_fragment(&fragment_downloader_send, 0)
            .await;

        while let Some(event) = self.recv.recv().await {
            let stop = match event {
                AudioPlayerEvent::Suspend => self.on_suspend(&fragment_downloader_send).await,
                AudioPlayerEvent::DownloadedFragment(fragment_index) => {
                    self.send_download_fragment(&fragment_downloader_send, fragment_index + 1)
                        .await;

                    if self.suspend && self.play_fragment_cursor == fragment_index {
                        crate::loginfo("occur KA".to_string());
                        self.on_suspend(&fragment_downloader_send).await
                    } else {
                        false
                    }
                }
            };

            if stop {
                if self.is_loop {
                    self.play_sample_cursor = 0;
                    self.play_fragment_cursor = 0;
                    let _ = self.send.send(AudioPlayerEvent::Suspend);
                    crate::loginfo("done".to_string());
                    continue;
                }
                break;
            }
        }
    }

    async fn send_download_fragment(
        &self,
        fragment_downloader_send: &UnboundedSender<AudioFragmentDownloaderEvent>,
        index: usize,
    ) {
        let Some(metadata) = self.metadata.clone() else {
            return;
        };

        let _ = fragment_downloader_send.send(AudioFragmentDownloaderEvent::DownloadFragment(
            DownloadFragmentEvent {
                metadata,
                cache: self.cache.clone(),
                index,
            },
        ));
    }

    async fn on_suspend(
        &mut self,
        fragment_downloader_send: &UnboundedSender<AudioFragmentDownloaderEvent>,
    ) -> bool {
        let Some(metadata) = self.metadata.as_ref() else {
            self.suspend = true;
            return false;
        };
        if self.play_fragment_cursor >= metadata.metadata.get_segments().len() {
            return true;
        }

        let Some(audio_buffer) = self.load_samples(50).await else {
            self.suspend = true;
            self.send_download_fragment(fragment_downloader_send, self.play_fragment_cursor)
                .await;
            return false;
        };

        let opt = GainOptions::new();
        opt.set_gain(0.3);
        let gain = GainNode::new_with_options(&self.ctx, &opt).unwrap();

        let send_suspend = self.send.clone();
        let cb = Closure::<dyn Fn()>::new(move || {
            let _ = send_suspend.send(AudioPlayerEvent::Suspend);
        });
        let source = self.ctx.create_buffer_source().unwrap();
        let _ = source.add_event_listener_with_callback("ended", cb.as_ref().unchecked_ref());
        cb.forget();

        source.set_buffer(Some(&audio_buffer));
        source
            .connect_with_audio_node(&gain)
            .unwrap()
            .connect_with_audio_node(&self.ctx.destination())
            .unwrap();
        let _ = source.start();

        false
    }

    async fn load_samples(&mut self, count: usize) -> Option<AudioBuffer> {
        let metadata = self.metadata.as_ref()?;
        let cache = self.cache.read().await;

        let mut left_samples = Vec::new();
        let mut right_samples = Vec::new();
        for _ in 0..count {
            if self.play_fragment_cursor >= metadata.metadata.get_segments().len() {
                break;
            }
            let Some(samples) = cache.get(&self.play_fragment_cursor) else {
                break;
            };
            let Some(sample) = samples.get(self.play_sample_cursor) else {
                self.play_fragment_cursor += 1;
                self.play_sample_cursor = 0;

                crate::loginfo(format!(
                    "| incr frag {} {} |",
                    self.play_fragment_cursor,
                    metadata.metadata.get_segments().len()
                ));
                continue;
            };
            for (index, &sample) in sample.samples().iter().enumerate() {
                if index % 2 == 0 {
                    left_samples.push(sample);
                } else {
                    right_samples.push(sample);
                }
            }

            self.play_sample_cursor += 1;
        }
        if left_samples.is_empty() {
            return None;
        }

        let opt = AudioBufferOptions::new(
            left_samples.len() as u32,
            metadata.metadata.sample_rate() as f32,
        );
        opt.set_number_of_channels(2);
        let audio_buffer = AudioBuffer::new(&opt).unwrap();
        audio_buffer.copy_to_channel(&left_samples, 0).unwrap();
        audio_buffer.copy_to_channel(&right_samples, 1).unwrap();

        Some(audio_buffer)
    }

    async fn download_fragment_thread_main(
        mut decoder: symphonia_codec_aac::AacDecoder,
        send: UnboundedSender<AudioPlayerEvent>,
        mut recv: UnboundedReceiver<AudioFragmentDownloaderEvent>,
    ) {
        while let Some(event) = recv.recv().await {
            match event {
                AudioFragmentDownloaderEvent::DownloadFragment(event) => {
                    for offset in 0..10 {
                        let Some(index) = Self::download_fragment(
                            &mut decoder,
                            &event.metadata,
                            &event.cache,
                            event.index + offset,
                        )
                        .await
                        else {
                            break;
                        };
                        let _ = send.send(AudioPlayerEvent::DownloadedFragment(index));
                    }
                }
            }
        }
    }

    async fn download_fragment(
        decoder: &mut symphonia_codec_aac::AacDecoder,
        metadata: &AudioPlayerMetadata,
        cache: &CacheType,
        mut index: usize,
    ) -> Option<usize> {
        let index = ({
            let mut download_fragment_index = None;
            while index < metadata.metadata.get_segments().len() {
                let cache = cache.read().await;
                if !cache.contains_key(&index) {
                    download_fragment_index = Some(index);
                    break;
                }
                index += 1;
            }
            download_fragment_index
        })?;

        let segment = metadata.metadata.get_segments().get(index)?;
        let Ok(fragment) = demux::fragment::MP4Fragment::parse(
            &metadata.metadata,
            &metadata.audio_info,
            segment.data_range,
        )
        .await
        else {
            return None;
        };

        let mut samples = Vec::with_capacity(fragment.get_sample_count());
        for sample_meta in fragment.get_samples() {
            if sample_meta.duration != 1024 {
                continue;
            }

            let data = &fragment.get_data()[sample_meta.range.0..sample_meta.range.1];
            let packet = Packet::new_from_slice(0, 0, sample_meta.duration as u64, data);
            let decoded_packet = match decoder.decode(&packet) {
                Ok(decoded_packet) => decoded_packet,
                Err(_err) => {
                    continue;
                }
            };
            let mut sample =
                SampleBuffer::<f32>::new(sample_meta.duration as u64, *decoded_packet.spec());
            sample.copy_interleaved_ref(decoded_packet);

            samples.push(sample);
        }

        {
            let mut cache = cache.write().await;
            cache.insert(index, samples);
        }

        Some(index)
    }
}
