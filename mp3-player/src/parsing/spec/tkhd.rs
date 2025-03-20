use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const TKHD: u32 = utils::box_type_u32(['t', 'k', 'h', 'd']);
#[derive(Debug)]
pub struct TrackHeaderBox {
    creation_time: u64,
    modification_time: u64,
    timescale: u32,
    duration: u64,

    layer: u16,
    alternate_group: u16,
    volume: u16,
    matrix: [u32; 9],
    width: u32,
    height: u32,
}

impl ParseBox for TrackHeaderBox {
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
            let _ = stream.read_u32().await?;
            let duration = stream.read_u64().await?;

            (creation_time, modification_time, timescale, duration)
        } else {
            let creation_time = stream.read_u32().await? as u64;
            let modification_time = stream.read_u32().await? as u64;
            let timescale = stream.read_u32().await?;
            let _ = stream.read_u32().await?;
            let duration = stream.read_u32().await? as u64;

            (creation_time, modification_time, timescale, duration)
        };

        for _ in 0..2 {
            let _ = stream.read_u32().await?;
        }
        let layer = stream.read_u16().await?;
        let alternate_group = stream.read_u16().await?;
        let volume = stream.read_u16().await? >> 8;
        let _ = stream.read_u16().await?;
        let mut matrix = [0u32; 9];
        for value in matrix.each_mut() {
            *value = stream.read_u32().await?;
        }
        let width = stream.read_u32().await?;
        let height = stream.read_u32().await?;

        Ok(Self {
            creation_time,
            modification_time,
            timescale,
            duration,

            layer,
            alternate_group,
            volume,
            matrix,
            width,
            height,
        })
    }
}
