use crate::{
    errors,
    parsing::{
        self,
        container::{MDIA, MINF, STBL, TRAK},
        mp4box,
        spec::{audio_sample::AudioSampleBox, hdlr::HDLR, mdhd::MDHD, stsd::STSD, tkhd::TKHD},
        utils,
    },
};

#[derive(Debug)]
pub struct TrackInitData {
    track_id: u32,
    duration: u64,
    timescale: u32,
    handler_type: utils::BoxType,
    name: String,
    codec: AudioSampleBox,
}

impl TrackInitData {
    pub const fn get_track_id(&self) -> u32 {
        self.track_id
    }

    pub const fn get_duration(&self) -> u64 {
        self.duration
    }

    pub const fn get_timescale(&self) -> u32 {
        self.timescale
    }

    pub async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
    ) -> Result<Self, errors::Error> {
        let moov = loop {
            match parsing::parse_box(stream).await? {
                mp4box::MP4Box::FreeSpace(..) => continue,
                mp4box::MP4Box::CompressedMovie(moov) => break moov,
                _ => return Err(errors::Error::IOError("cannot find moov box".to_string())),
            }
        };
        let Some(mp4box::MP4Box::Track(track)) = moov.get(TRAK) else {
            return Err(errors::Error::IOError("cannot find trak".to_string()));
        };
        let Some(mp4box::MP4Box::TrackHeader(track_header)) = track.get(TKHD) else {
            return Err(errors::Error::IOError("cannot find tkhd".to_string()));
        };

        let Some(mp4box::MP4Box::Media(media)) = track.get(MDIA) else {
            return Err(errors::Error::IOError("cannot find mdia".to_string()));
        };
        let Some(mp4box::MP4Box::MediaHeader(media_header)) = media.get(MDHD) else {
            return Err(errors::Error::IOError("cannot find mdhd".to_string()));
        };
        let Some(mp4box::MP4Box::Handler(handler)) = media.get(HDLR) else {
            return Err(errors::Error::IOError("cannot find hdlr".to_string()));
        };

        let Some(mp4box::MP4Box::MediaInformation(media_information)) = media.get(MINF) else {
            return Err(errors::Error::IOError("cannot find minf".to_string()));
        };
        let Some(mp4box::MP4Box::SampleTable(sample_table)) = media_information.get(STBL) else {
            return Err(errors::Error::IOError("cannot find stbl".to_string()));
        };
        let Some(mp4box::MP4Box::SampleDescription(sample_description)) = sample_table.get(STSD)
        else {
            return Err(errors::Error::IOError("cannot find stsd".to_string()));
        };

        let track_id = track_header.get_track_id();
        let duration = track_header.get_duration();
        let timescale = media_header.get_timescale();
        let handler_type = handler.get_handler_type();
        let name = handler.get_name().to_owned();
        let Some(codec) = sample_description.get_audio_codec() else {
            return Err(errors::Error::InternalError(
                "only support audio codec".to_string(),
            ));
        };

        Ok(Self {
            track_id,
            duration,
            timescale,
            handler_type,
            name,
            codec,
        })
    }
}
