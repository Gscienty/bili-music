use crate::errors;

use super::utils;

#[derive(Debug)]
pub enum MPEG4Descriptor {
    Descriptor(Descriptor),
}

#[derive(Debug)]
pub struct Descriptor {
    tag: u8,
    data: Vec<u8>,
}

impl MPEG4Descriptor {
    async fn parse_descriptor(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        tag: u8,
        size: usize,
    ) -> Result<Descriptor, errors::Error> {
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(stream.read_u8().await?);
        }

        Ok(Descriptor { tag, data })
    }

    pub(super) async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
    ) -> Result<Self, errors::Error> {
        let tag = stream.read_u8().await?;
        let mut byte_read = stream.read_u8().await?;

        let mut size = 0usize;
        while (byte_read & 0x80) != 0 {
            size = (size << 7) + (byte_read & 0x7f) as usize;
            byte_read = stream.read_u8().await?;
        }
        size = (size << 7) + (byte_read & 0x7f) as usize;

        let descriptor = match tag {
            0x03..=0x06 => {
                return Err(errors::Error::IOError(format!(
                    "mpeg4 not implemented, tag: {tag}"
                )));
            }
            _ => Self::Descriptor(Self::parse_descriptor(stream, tag, size).await?),
        };

        Ok(descriptor)
    }
}
