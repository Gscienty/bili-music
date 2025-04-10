use std::cmp::Ordering;

use crate::{bili_api::StreamAudioInfo, demux, errors};

#[derive(Debug)]
pub struct Progress {
    audio: StreamAudioInfo,

    metadata: demux::metadata::MP4Metadata,

    fragment_index: usize,
    fragment: Option<demux::fragment::MP4Fragment>,
    should_upgrade_fragment: bool,

    sample_index: usize,
}

impl Progress {
    pub async fn new(audio: &StreamAudioInfo) -> Result<Self, errors::Error> {
        let metadata = demux::metadata::MP4Metadata::parse(audio).await?;
        Ok(Self {
            audio: audio.clone(),

            metadata,
            fragment_index: 0,
            fragment: None,
            should_upgrade_fragment: true,
            sample_index: 0,
        })
    }

    pub async fn reset_time(&mut self, enterpoint: u64) -> Result<(), errors::Error> {
        let fragment_index = self
            .metadata
            .get_segments()
            .binary_search_by(|segment| {
                if segment.time_range.0 <= enterpoint && enterpoint < segment.time_range.1 {
                    Ordering::Equal
                } else if enterpoint < segment.time_range.0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .map_err(|err| errors::Error::InternalError(err.to_string()))?;
        self.fragment_index = fragment_index;

        self.should_upgrade_fragment = true;
        self.upgrade_fragment().await?;

        if let Some(fragment) = &self.fragment {
            let sample_index = fragment
                .get_samples()
                .binary_search_by(|sample| {
                    if sample.start_time <= enterpoint
                        && enterpoint < sample.start_time + sample.duration as u64
                    {
                        Ordering::Equal
                    } else if enterpoint < sample.start_time {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .map_err(|err| errors::Error::InternalError(err.to_string()))?;

            self.sample_index = sample_index;
        }
        Ok(())
    }

    async fn upgrade_fragment(&mut self) -> Result<(), errors::Error> {
        if !self.should_upgrade_fragment {
            return Ok(());
        }
        self.should_upgrade_fragment = false;

        crate::loginfo(format!(
            "| {} / {} |",
            self.fragment_index + 1,
            self.metadata.get_segments().len()
        ));
        let Some(reference) = &self.metadata.get_segments().get(self.fragment_index) else {
            self.fragment = None;

            return Err(errors::Error::InternalError(
                "out of references limit".to_string(),
            ));
        };
        let fragment =
            demux::fragment::MP4Fragment::parse(&self.metadata, &self.audio, reference.data_range)
                .await?;
        self.fragment = Some(fragment);

        Ok(())
    }

    pub async fn next_sample(&mut self) -> Option<(&demux::sample::MP4Sample, &[u8])> {
        self.upgrade_fragment().await.ok()?;
        let Some(fragment) = &self.fragment else {
            return None;
        };
        let (sample, data) = fragment.get_sample(self.sample_index)?;

        if self.sample_index + 1 >= fragment.get_sample_count() {
            self.sample_index = 0;
            self.fragment_index += 1;
            self.should_upgrade_fragment = true;
        } else {
            self.sample_index += 1;
        }

        Some((sample, data))
    }
}
