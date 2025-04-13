use std::time::Duration;

use crate::{
    errors,
    parsing::{
        container::{ContainerBox, MDIA, TRAK},
        mp4box,
        spec::{hdlr::HDLR, mdhd::MDHD, tkhd::TKHD},
        utils,
    },
};

#[derive(Debug)]
pub struct MP4Track {
    track_id: u32,
    handler_type: utils::BoxType,
    duration: Duration,
}

impl TryFrom<&ContainerBox<TRAK>> for MP4Track {
    type Error = errors::Error;

    fn try_from(value: &ContainerBox<TRAK>) -> Result<Self, Self::Error> {
        let Some(mp4box::MP4Box::TrackHeader(header)) = value.get(TKHD) else {
            return Err(errors::Error::IOError("cannot find TKHD".to_string()));
        };
        let track_id = header.get_track_id();

        let Some(mp4box::MP4Box::Media(media)) = value.get(MDIA) else {
            return Err(errors::Error::IOError("cannot find MDIA".to_string()));
        };
        let Some(mp4box::MP4Box::Handler(handler)) = media.get(HDLR) else {
            return Err(errors::Error::IOError("cannot find MDIA".to_string()));
        };
        let handler_type = handler.get_handler_type();

        let Some(mp4box::MP4Box::MediaHeader(media_header)) = media.get(MDHD) else {
            return Err(errors::Error::IOError("cannot find MDHD".to_string()));
        };
        let duration =
            Duration::from_micros(media_header.duration * 1000000 / media_header.timescale as u64);

        Ok(Self {
            track_id,
            handler_type,
            duration,
        })
    }
}
