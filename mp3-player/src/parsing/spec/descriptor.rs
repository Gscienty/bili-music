use crate::{errors, parsing::utils};

#[derive(Debug, Clone)]
pub struct ESDescriptor {
    es_id: u16,
    depends_on_es_id: u16,
    url: String,
    ocr_es_id: u16,

    decoder_config: DecoderConfigDescriptor,
    sl_config: SLConfigDescriptor,
}

#[derive(Debug, Clone)]
pub struct SLConfigDescriptor {
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct DecoderConfigDescriptor {
    oti: u8,
    stream_type: u8,
    up_stream: bool,
    buffer_size: u32,
    max_bitrate: u32,
    avg_bitrate: u32,

    decoder_specific: DecoderSpecificInfo,
}

#[derive(Debug, Clone)]
pub struct DecoderSpecificInfo {
    object_type: u8,
    sampling_frequency_index: u32,
    channel_configuration: u8,
}

pub(super) async fn parse_descriptor_header(
    stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
) -> Result<(u8, usize, usize), errors::Error> {
    let mut hdr_size = 0;

    let tag = stream.read_u8().await?;
    let mut byte_read = stream.read_u8().await?;
    hdr_size += 2;

    let mut descriptor_size = 0usize;
    while (byte_read & 0x80) != 0 {
        descriptor_size = (descriptor_size << 7) + (byte_read & 0x7f) as usize;
        byte_read = stream.read_u8().await?;
        hdr_size += 1;
    }
    descriptor_size = (descriptor_size << 7) + (byte_read & 0x7f) as usize;

    Ok((tag, descriptor_size, hdr_size))
}

impl ESDescriptor {
    pub async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        size: usize,
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

        let mut decoder_config = None;
        let mut sl_config = None;

        while size - consumed_bytes > 0 {
            let (tag, desc_size, hdr_size) = parse_descriptor_header(stream).await?;
            consumed_bytes += hdr_size;

            match tag {
                0x04 => {
                    let (cfg, desc_size) =
                        DecoderConfigDescriptor::parse(stream, desc_size).await?;
                    decoder_config = Some(cfg);
                    consumed_bytes += desc_size;
                }
                0x06 => {
                    let (cfg, desc_size) = SLConfigDescriptor::parse(stream, desc_size).await?;
                    sl_config = Some(cfg);
                    consumed_bytes += desc_size;
                }
                _ => {
                    stream.skip_bytes(desc_size).await?;
                    consumed_bytes += desc_size;
                }
            }
        }

        let Some(decoder_config) = decoder_config else {
            return Err(errors::Error::IOError(
                "not found decoder config".to_string(),
            ));
        };
        let Some(sl_config) = sl_config else {
            return Err(errors::Error::IOError("not found sl config".to_string()));
        };

        Ok((
            Self {
                es_id,
                depends_on_es_id,
                url,
                ocr_es_id,

                decoder_config,
                sl_config,
            },
            consumed_bytes,
        ))
    }

    pub fn get_decoder_config(&self) -> &DecoderConfigDescriptor {
        &self.decoder_config
    }
}

impl DecoderConfigDescriptor {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        size: usize,
    ) -> Result<(Self, usize), errors::Error> {
        let oti = stream.read_u8().await?;
        let tmp = stream.read_u8().await?;

        let up_stream = ((tmp >> 1) & 0x01) != 0;
        let stream_type = tmp >> 2;
        let buffer_size = stream.read_u24().await?;
        let max_bitrate = stream.read_u32().await?;
        let avg_bitrate = stream.read_u32().await?;

        let mut consumed_bytes = 13;
        let mut decoder_specific = None;
        while size - consumed_bytes > 0 {
            let (tag, desc_size, hdr_size) = parse_descriptor_header(stream).await?;
            consumed_bytes += hdr_size;

            match tag {
                0x05 => {
                    let (cfg, desc_size) = DecoderSpecificInfo::parse(stream).await?;
                    decoder_specific = Some(cfg);
                    consumed_bytes += desc_size;
                }
                _ => {
                    stream.skip_bytes(desc_size).await?;
                    consumed_bytes += desc_size;
                }
            }
        }

        let Some(decoder_specific) = decoder_specific else {
            return Err(errors::Error::IOError(
                "cannot found decoder specific".to_string(),
            ));
        };

        Ok((
            Self {
                oti,
                up_stream,
                stream_type,
                buffer_size,
                max_bitrate,
                avg_bitrate,

                decoder_specific,
            },
            consumed_bytes,
        ))
    }

    pub fn get_decoder_specific(&self) -> &DecoderSpecificInfo {
        &self.decoder_specific
    }

    pub const fn get_avg_bitrate(&self) -> u32 {
        self.avg_bitrate
    }
}

impl DecoderSpecificInfo {
    async fn parse(
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
                Self {
                    object_type,
                    sampling_frequency_index,
                    channel_configuration,
                },
                stream.get_consumed_bytes(),
            ))
        } else {
            let channel_configuration = stream.read_u8(4).await?;

            Ok((
                Self {
                    object_type,
                    sampling_frequency_index,
                    channel_configuration,
                },
                stream.get_consumed_bytes(),
            ))
        }
    }

    pub const fn get_object_type(&self) -> u8 {
        self.object_type
    }

    pub const fn get_sampling_frequency_index(&self) -> u32 {
        self.sampling_frequency_index
    }

    pub const fn get_channel_configuration(&self) -> u8 {
        self.channel_configuration
    }
}

impl SLConfigDescriptor {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        size: usize,
    ) -> Result<(Self, usize), errors::Error> {
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(stream.read_u8().await?);
        }

        Ok((Self { data }, size))
    }
}
