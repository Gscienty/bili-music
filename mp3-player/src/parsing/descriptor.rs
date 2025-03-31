use crate::errors;

use super::utils;

#[derive(Debug, Clone)]
pub enum MPEG4Descriptor {
    ES(ESDescriptor),
    DecoderConfig(DecoderConfigDescriptor),
    DecoderSpecificInfo(DecoderSpecificInfo),
    SLConfig(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct ESDescriptor {
    es_id: u16,
    depends_on_es_id: u16,
    url: String,
    ocr_es_id: u16,
}

#[derive(Debug, Clone)]
pub struct DecoderConfigDescriptor {
    oti: u8,
    stream_type: u8,
    up_stream: bool,
    buffer_size: u32,
    max_bitrate: u32,
    avg_bitrate: u32,
}

#[derive(Debug, Clone)]
pub struct DecoderSpecificInfo {
    object_type: u8,
    sampling_frequency_index: u32,
    channel_configuration: u8,
}

impl MPEG4Descriptor {
    pub(super) async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        tag: u8,
        size: usize,
    ) -> Result<(Self, usize), errors::Error> {
        match tag {
            0x03 => Self::parse_es(stream).await,
            0x04 => Self::parse_decoder_config(stream).await,
            0x05 => Self::parse_specific_info(stream).await,
            0x06 => {
                // SL Config Descriptor, ignore now
                let mut data = Vec::with_capacity(size);
                for _ in 0..size {
                    data.push(stream.read_u8().await?);
                }
                Ok((Self::SLConfig(data), size))
            }
            _ => Err(errors::Error::IOError(format!(
                "mpeg4 not implemented, tag: {tag}"
            ))),
        }
    }

    async fn parse_es(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
    ) -> Result<(Self, usize), errors::Error> {
        let es_id = stream.read_u16().await?;
        let flags = stream.read_u8().await?;

        let mut consumed_bytes = 3;
        let depends_on_es_id = if (flags & 0x80) != 0 {
            consumed_bytes += 2;
            stream.read_u16().await?
        } else {
            0
        };

        let url = if (flags & 0x40) != 0 {
            let l = stream.read_u8().await? as usize;
            let mut url_buf = Vec::with_capacity(l);
            for _ in 0..l {
                url_buf.push(stream.read_u8().await?);
            }

            consumed_bytes += 1 + l;
            std::str::from_utf8(&url_buf)
                .map_err(|err| errors::Error::IOError("not utf-8".to_string()))?
                .to_owned()
        } else {
            String::new()
        };

        let ocr_es_id = if (flags & 0x20) != 0 {
            consumed_bytes += 2;
            stream.read_u16().await?
        } else {
            0
        };

        Ok((
            Self::ES(ESDescriptor {
                es_id,
                depends_on_es_id,
                url,
                ocr_es_id,
            }),
            consumed_bytes,
        ))
    }

    async fn parse_decoder_config(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
    ) -> Result<(Self, usize), errors::Error> {
        let oti = stream.read_u8().await?;
        let tmp = stream.read_u8().await?;

        let up_stream = ((tmp >> 1) & 0x01) != 0;
        let stream_type = tmp >> 2;
        let buffer_size = stream.read_u24().await?;
        let max_bitrate = stream.read_u32().await?;
        let avg_bitrate = stream.read_u32().await?;

        Ok((
            Self::DecoderConfig(DecoderConfigDescriptor {
                oti,
                up_stream,
                stream_type,
                buffer_size,
                max_bitrate,
                avg_bitrate,
            }),
            13,
        ))
    }

    async fn parse_specific_info(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
    ) -> Result<(Self, usize), errors::Error> {
        let mut stream = utils::BitStream::new(stream);

        let mut object_type = stream.read_u8(5).await?;
        if object_type == 31 {
            object_type = 32 + stream.read_u8(6).await?;
        }
        let sampling_frequency_index = stream.read_u32(4).await?;
        if sampling_frequency_index == 0x0f {
            let sampling_frequency_index = stream.read_u32(24).await?;
            let channel_configuration = stream.read_u8(4).await?;

            Ok((
                Self::DecoderSpecificInfo(DecoderSpecificInfo {
                    object_type,
                    sampling_frequency_index,
                    channel_configuration,
                }),
                stream.get_consumed_bytes(),
            ))
        } else {
            let channel_configuration = stream.read_u8(4).await?;

            Ok((
                Self::DecoderSpecificInfo(DecoderSpecificInfo {
                    object_type,
                    sampling_frequency_index,
                    channel_configuration,
                }),
                stream.get_consumed_bytes(),
            ))
        }
    }
}
