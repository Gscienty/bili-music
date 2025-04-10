use crate::{
    bili_api::{self, StreamAudioInfo},
    errors,
    parsing::{
        self,
        container::{MDIA, MINF, MVEX, STBL, TRAK},
        mp4box,
        spec::{
            ftyp,
            sidx::{CompressedSegmentIndexBox, Reference},
            stsd::STSD,
            trex::TREX,
        },
        utils,
    },
};

use super::{
    audio_object_type::AudioObjectType,
    channel_config::ChannelConfig,
    sampling_frequence_index::{self, SamplingFrequenceIndex},
};

#[derive(Debug)]
pub struct MP4Metadata {
    file_type: ftyp::FileTypeBox,

    default_sample_duration: u32,
    audio_object_type: AudioObjectType,
    sampling_frequence_index: SamplingFrequenceIndex,
    channel_config: ChannelConfig,

    bitrate: u32,

    data_start_offset: usize,
    url: String,
    segments: Vec<SegmentIndex>,
}

#[derive(Debug)]
pub struct SegmentIndex {
    pub(crate) data_range: (usize, usize),
    pub(crate) time_range: (u64, u64),
}

impl MP4Metadata {
    pub async fn parse(audio: &bili_api::StreamAudioInfo) -> Result<Self, errors::Error> {
        let data_start_offset = audio.data_start_offset();

        let mut stream = bili_api::fetch_m4s_chunk(&audio.base_url, audio.initialization).await?;

        let mp4box::MP4Box::FileType(file_type) = parsing::parse_box(&mut stream).await? else {
            return Err(errors::Error::IOError("cannot get FTYP".to_string()));
        };
        let moov = loop {
            match parsing::parse_box(&mut stream).await? {
                mp4box::MP4Box::FreeSpace(..) => continue,
                mp4box::MP4Box::CompressedMovie(moov) => break moov,
                _ => return Err(errors::Error::IOError("cannot find moov box".to_string())),
            }
        };
        let Some(mp4box::MP4Box::Track(track)) = moov.get(TRAK) else {
            return Err(errors::Error::IOError("cannot find TRAK".to_string()));
        };
        let Some(mp4box::MP4Box::Media(media)) = track.get(MDIA) else {
            return Err(errors::Error::IOError("cannot find MDIA".to_string()));
        };
        let Some(mp4box::MP4Box::MediaInformation(media_information)) = media.get(MINF) else {
            return Err(errors::Error::IOError("cannot find MINF".to_string()));
        };
        let Some(mp4box::MP4Box::SampleTable(sample_table)) = media_information.get(STBL) else {
            return Err(errors::Error::IOError("cannot find STBL".to_string()));
        };
        let Some(mp4box::MP4Box::SampleDescription(sample_description)) = sample_table.get(STSD)
        else {
            return Err(errors::Error::IOError("cannot find STSD".to_string()));
        };
        let Some(audio_sample) = sample_description.get_audio_sample() else {
            return Err(errors::Error::IOError(
                "cannot find Audio Codec".to_string(),
            ));
        };
        let conf = audio_sample
            .get_elementary_stream_descriptor()
            .get_es_descriptor()
            .get_decoder_config();
        let bitrate = conf.get_avg_bitrate();
        let spec = conf.get_decoder_specific();
        let audio_object_type = spec.get_object_type().try_into()?;
        let sampling_frequence_index = spec.get_sampling_frequency_index().try_into()?;
        let channel_config = spec.get_channel_configuration().try_into()?;

        let Some(mp4box::MP4Box::MovieExtends(movie_extends)) = moov.get(MVEX) else {
            return Err(errors::Error::IOError("cannot find MVEX".to_string()));
        };
        let Some(mp4box::MP4Box::TrackExtends(track_extends)) = movie_extends.get(TREX) else {
            return Err(errors::Error::IOError("cannot find MVEX".to_string()));
        };
        let default_sample_duration = track_extends.get_default_sample_duration();

        let mut stream = bili_api::fetch_m4s_chunk(&audio.base_url, audio.index_range).await?;
        let mp4box::MP4Box::CompressedSegmentIndex(segments_indices) =
            parsing::parse_box(&mut stream).await?
        else {
            return Err(errors::Error::IOError("want sidx".to_string()));
        };

        let mut segments = Vec::with_capacity(segments_indices.count());
        let mut data_offset = data_start_offset + segments_indices.get_first_offset();
        let mut time_offset = 0;
        for reference in segments_indices.get_references().iter() {
            segments.push(SegmentIndex {
                data_range: (
                    data_offset + 1,
                    data_offset + reference.get_references_size(),
                ),
                time_range: (
                    time_offset,
                    time_offset + reference.get_subsegment_duration(),
                ),
            });

            data_offset += reference.get_references_size();
            time_offset += reference.get_subsegment_duration();
        }

        Ok(Self {
            file_type,

            default_sample_duration,
            audio_object_type,
            sampling_frequence_index,
            channel_config,

            bitrate,

            data_start_offset,
            url: audio.base_url.to_owned(),
            segments,
        })
    }

    pub const fn get_major_brand(&self) -> utils::BoxType {
        self.file_type.major_brand
    }

    pub const fn get_minor_version(&self) -> u32 {
        self.file_type.minor_version
    }

    pub fn get_compatible_brands(&self) -> &[utils::BoxType] {
        self.file_type.compatible_brands.as_slice()
    }

    pub fn get_segments(&self) -> &[SegmentIndex] {
        &self.segments
    }

    pub const fn get_default_sample_duration(&self) -> u32 {
        self.default_sample_duration
    }

    pub const fn get_bitrate(&self) -> u32 {
        self.bitrate
    }

    pub const fn get_channel_configuration(&self) -> ChannelConfig {
        self.channel_config
    }
}
