use std::iter::zip;

use futures::SinkExt;

use crate::{
    bili_api::{self, StreamAudioInfo},
    errors,
    parsing::{
        self,
        container::{ContainerBox, MOOF, TRAF},
        mp4box,
        spec::{tfdt::TFDT, tfhd::TFHD, trun::TRUN},
    },
};

use super::{metadata::MP4Metadata, sample::MP4Sample};

#[derive(Debug)]
pub struct MP4Fragment {
    samples: Vec<MP4Sample>,
    data: Vec<u8>,
}

impl MP4Fragment {
    pub async fn parse(
        metadata: &MP4Metadata,
        audio: &StreamAudioInfo,
        range: (usize, usize),
    ) -> Result<Self, errors::Error> {
        let mut stream = bili_api::fetch_m4s_chunk(&audio.base_url, range).await?;

        let mp4box::MP4Box::CompressedMovieFragment(fragment) =
            parsing::parse_box(&mut stream).await?
        else {
            return Err(errors::Error::IOError("not found MOOF".to_string()));
        };
        let samples = Self::parse_samples(metadata, &fragment)?;

        let mp4box::MP4Box::MediaData(media_data) = parsing::parse_box(&mut stream).await? else {
            return Err(errors::Error::IOError("not found MDAT".to_string()));
        };
        let data: Vec<u8> = media_data.into();

        Ok(Self { samples, data })
    }

    pub fn get_samples(&self) -> &[MP4Sample] {
        &self.samples
    }

    pub fn get_sample_count(&self) -> usize {
        self.samples.len()
    }

    pub fn get_sample(&self, index: usize) -> Option<(&MP4Sample, &[u8])> {
        self.samples
            .get(index)
            .map(|sample| (sample, &self.data[sample.range.0..sample.range.1]))
    }

    fn parse_samples(
        metadata: &MP4Metadata,
        fragment: &ContainerBox<MOOF>,
    ) -> Result<Vec<MP4Sample>, errors::Error> {
        let Some(mp4box::MP4Box::TrackFragment(track_fragment)) = fragment.get(TRAF) else {
            return Err(errors::Error::IOError("not found TRAF".to_string()));
        };
        let base_start_time =
            if let Some(mp4box::MP4Box::TrackFragmentBaseMediaDecodeTime(base_decode_time)) =
                track_fragment.get(TFDT)
            {
                base_decode_time.get_base_media_decode_time()
            } else {
                0
            };
        let default_sample_duration =
            if let Some(mp4box::MP4Box::TrackFragmentHeader(track_fragment_header)) =
                track_fragment.get(TFHD)
            {
                let duration = track_fragment_header.get_default_sample_duration();
                if duration != 0 {
                    duration
                } else {
                    metadata.get_default_sample_duration()
                }
            } else {
                metadata.get_default_sample_duration()
            };

        let Some(mp4box::MP4Box::TrackRun(track_run)) = track_fragment.get(TRUN) else {
            return Err(errors::Error::IOError("not found TRUN".to_string()));
        };

        let mut sample_range = Vec::with_capacity(track_run.get_sample_count());
        let mut offset = 0usize;
        for &sample_size in track_run.get_sample_size().iter() {
            sample_range.push((offset, offset + sample_size as usize));
            offset += sample_size as usize;
        }

        let mut sample_time = Vec::with_capacity(track_run.get_sample_count());
        let mut offset = 0u64;
        if track_run.get_sample_duration().is_empty() {
            for _ in 0..track_run.get_sample_count() {
                sample_time.push((base_start_time + offset, default_sample_duration));
                offset += default_sample_duration as u64;
            }
        } else {
            for &duration in track_run.get_sample_duration().iter() {
                sample_time.push((base_start_time + offset, duration));
                offset += duration as u64;
            }
        }

        Ok(zip(sample_range, sample_time)
            .map(|(range, (start_time, duration))| MP4Sample::new(start_time, duration, range))
            .collect())
    }
}
