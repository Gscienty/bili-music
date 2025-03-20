use crate::{
    errors,
    parsing::{ParseBox, mp4box, utils},
};

use super::audio_sample::{AudioSampleBox, MP4A};

pub const STSD: u32 = utils::box_type_u32(['s', 't', 's', 'd']);
#[derive(Debug)]
pub struct SampleDescriptionBox {}

impl SampleDescriptionBox {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            MP4A => mp4box::MP4Box::AudioSample(AudioSampleBox::parse(stream, typ, size).await?),
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown dref box type: {typ}"
                )));
            }
        };

        Ok(child)
    }
}

impl ParseBox for SampleDescriptionBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let _ = stream.read_box_version_flag_header().await?;
        let entry_count = stream.read_u32().await?;
        size -= 8;
        for _ in 0..entry_count {
            let (typ, box_size, hdr_size) = stream.read_box_common_header(size).await?;
            let child = Self::parse_child(stream, typ, box_size - hdr_size).await?;
            size -= box_size;
        }

        Ok(Self {})
    }
}
