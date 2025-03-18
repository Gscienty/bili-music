use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const MVHD: u32 = utils::box_type_u32(['m', 'v', 'h', 'd']);
#[derive(Debug)]
pub struct MovieHeaderBox {
    creation_time: u64,
    modification_time: u64,
    timescale: u32,
    duration: u64,
    rate: u32,
    volume: u16,
    matrix: [u32; 9],
    next_track_id: u32,
}

impl ParseBox for MovieHeaderBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
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

        let rate = stream.read_u32().await?;
        let volume = stream.read_u16().await? >> 8;
        let _ = stream.read_u16().await?;
        for _ in 0..2 {
            let _ = stream.read_u32().await?;
        }
        let mut matrix = [0u32; 9];
        for value in matrix.each_mut() {
            *value = stream.read_u32().await?;
        }
        for _ in 0..6 {
            let _ = stream.read_u32().await?;
        }
        let next_track_id = stream.read_u32().await?;

        Ok(Self {
            creation_time,
            modification_time,
            timescale,
            duration,

            rate,
            volume,
            matrix,
            next_track_id,
        })
    }
}
