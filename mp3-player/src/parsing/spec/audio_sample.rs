use crate::{
    errors,
    parsing::{ParseBox, mp4box, utils},
};

use super::esds::{ESDS, ElementaryStreamDescriptorBox};

pub const MP4A: u32 = utils::box_type_u32(['m', 'p', '4', 'a']);
#[derive(Debug)]
pub struct AudioSampleBox {
    data_reference_header: u16,
    channel_count: u16,
    sample_size: u16,
    sample_rate: u32,
    footer: Box<mp4box::MP4Box>,
}

impl AudioSampleBox {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            ESDS => mp4box::MP4Box::ElementaryStreamDescriptor(
                ElementaryStreamDescriptorBox::parse(stream, typ, size).await?,
            ),
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown dref box type: {typ}"
                )));
            }
        };

        Ok(child)
    }
}

impl ParseBox for AudioSampleBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let data_reference_header = stream.read_sample_header().await?;
        for _ in 0..2 {
            let _ = stream.read_u32().await?;
        }
        let channel_count = stream.read_u16().await?;
        let sample_size = stream.read_u16().await?;
        for _ in 0..2 {
            let _ = stream.read_u16().await?;
        }
        let sample_rate = stream.read_u32().await? / (1 << 16);

        size -= 28;
        let (typ, box_size, hdr_size) = stream.read_box_common_header(size).await?;
        let footer = Box::new(Self::parse_child(stream, typ, box_size - hdr_size).await?);

        Ok(Self {
            data_reference_header,
            channel_count,
            sample_size,
            sample_rate,
            footer,
        })
    }
}
