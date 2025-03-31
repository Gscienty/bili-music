use crate::{
    bili_api::{self, StreamAudioInfo},
    errors,
    parsing::{
        self,
        container::TRAF,
        mp4box,
        spec::{sidx::CompressedSegmentIndexBox, tfdt::TFDT, tfhd::TFHD, trun::TRUN},
        utils,
    },
};

use super::init::TrackInitData;

pub struct FragmentSet {
    data_start_offset: usize,
    url: String,
    segments_indices: CompressedSegmentIndexBox,
}

#[derive(Debug)]
pub struct Fragment {
    base_media_decode_time: u64,
    sample_duration: u64,
    duration: u64,
    data_offset: u32,

    data: Vec<u8>,
}

impl FragmentSet {
    pub async fn init(audio_info: &StreamAudioInfo) -> Result<Self, errors::Error> {
        let mut stream = bili_api::fetch_m4s_chunk(&audio_info.base_url, audio_info.index_range)
            .await
            .unwrap();

        let mp4box::MP4Box::CompressedSegmentIndex(segments_indices) =
            parsing::parse_box(&mut stream).await?
        else {
            return Err(errors::Error::IOError("want sidx".to_string()));
        };

        Ok(Self {
            data_start_offset: audio_info.data_start_offset(),
            url: audio_info.base_url.to_owned(),
            segments_indices,
        })
    }

    pub async fn get_fragment(
        &self,
        fragment_index: usize,
        track: &TrackInitData,
    ) -> Result<Fragment, errors::Error> {
        let Some(range) = self
            .segments_indices
            .get_range(self.data_start_offset, fragment_index)
        else {
            return Err(errors::Error::IOError(
                "out of segments indices range".to_string(),
            ));
        };

        let mut stream = bili_api::fetch_m4s_chunk(&self.url, range).await?;
        Fragment::parse(&mut stream, track).await
    }
}

impl Fragment {
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        track: &TrackInitData,
    ) -> Result<Self, errors::Error> {
        let mp4box::MP4Box::CompressedMovieFragment(moof) = parsing::parse_box(stream).await?
        else {
            return Err(errors::Error::IOError("want moof".to_string()));
        };
        let Some(mp4box::MP4Box::TrackFragment(track_fragment)) = moof.get(TRAF) else {
            return Err(errors::Error::IOError("cannot find traf".to_string()));
        };
        let Some(mp4box::MP4Box::TrackFragmentBaseMediaDecodeTime(base_media_decode_time)) =
            track_fragment.get(TFDT)
        else {
            return Err(errors::Error::IOError("cannot find tfdt".to_string()));
        };
        let Some(mp4box::MP4Box::TrackFragmentHeader(track_fragment_header)) =
            track_fragment.get(TFHD)
        else {
            return Err(errors::Error::IOError("cannot find tfhd".to_string()));
        };
        let Some(mp4box::MP4Box::TrackRun(track_run)) = track_fragment.get(TRUN) else {
            return Err(errors::Error::IOError("cannot find trun".to_string()));
        };

        let track_id = track_fragment_header.get_track_id();
        if track_id != track.get_track_id() {
            return Err(errors::Error::IOError(format!(
                "track id not match: tfhd: {track_id}, init_data: {}",
                track.get_track_id(),
            )));
        }
        let default_sample_duration = track_fragment_header.get_default_sample_duration();
        let sample_duration = if default_sample_duration == 0 {
            track.get_duration()
        } else {
            default_sample_duration as u64
        };
        crate::loginfo(format!("sample duration: {sample_duration}"));
        let base_media_decode_time = base_media_decode_time.get_base_media_decode_time();
        let sample_count = track_run.get_sample_count();
        let data_offset = track_run.get_data_offset();

        let mp4box::MP4Box::MediaData(mdat) = parsing::parse_box(stream).await? else {
            return Err(errors::Error::IOError("want mdat".to_string()));
        };

        let timescale = track.get_timescale() as u64;

        Ok(Self {
            base_media_decode_time: base_media_decode_time / timescale,
            sample_duration: sample_duration / timescale,
            duration: (sample_duration * sample_count as u64) / timescale,
            data_offset,

            data: mdat.into(),
        })
    }
}
