use crate::{
    bili_api::{self, StreamAudioInfo},
    errors,
    parsing::{
        self,
        container::{MVEX, TRAK},
        mp4box,
        spec::{
            ftyp,
            sidx::{CompressedSegmentIndexBox, Reference},
            trex::TREX,
        },
        utils,
    },
};

#[derive(Debug)]
pub struct MP4Header {
    file_type: ftyp::FileTypeBox,

    default_sample_duration: u32,

    data_start_offset: usize,
    url: String,
    segments_indices: CompressedSegmentIndexBox,
}

impl MP4Header {
    pub async fn parse(audio: &StreamAudioInfo) -> Result<Self, errors::Error> {
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
        // let Some(mp4box::MP4Box::Track(track)) = moov.get(TRAK) else {
        //     return Err(errors::Error::IOError("cannot find trak".to_string()));
        // };
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

        Ok(Self {
            file_type,

            default_sample_duration,

            data_start_offset: audio.data_start_offset(),
            url: audio.base_url.to_owned(),
            segments_indices,
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

    pub fn get_reference(&self, index: usize) -> Option<((usize, usize), &Reference)> {
        self.segments_indices
            .get_reference(self.data_start_offset, index)
    }

    pub const fn get_default_sample_duration(&self) -> u32 {
        self.default_sample_duration
    }
}
