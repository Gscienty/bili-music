use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const MDHD: u32 = utils::box_type_u32(['m', 'd', 'h', 'd']);
#[derive(Debug)]
pub struct MediaHeaderBox {
    creation_time: u64,
    modification_time: u64,
    pub(crate) timescale: u32,
    pub(crate) duration: u64,

    language: u16,
}

impl ParseBox for MediaHeaderBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        _size: usize,
    ) -> Result<Self, errors::Error> {
        let (version, _) = stream.read_box_version_flag_header().await?;
        let (creation_time, modification_time, timescale, duration) = if version == 1 {
            let creation_time = stream.read_u64().await?;
            let modification_time = stream.read_u64().await?;
            let timescale = stream.read_u32().await?;
            let duration = stream.read_u64().await?;

            (creation_time, modification_time, timescale, duration)
        } else {
            let creation_time = stream.read_u32().await? as u64;
            let modification_time = stream.read_u32().await? as u64;
            let timescale = stream.read_u32().await?;
            let duration = stream.read_u32().await? as u64;

            (creation_time, modification_time, timescale, duration)
        };

        let language = stream.read_u16().await?;
        let _ = stream.read_u16().await?;

        Ok(Self {
            creation_time,
            modification_time,
            timescale,
            duration,
            language,
        })
    }
}
