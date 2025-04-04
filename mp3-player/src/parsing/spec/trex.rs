use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const TREX: u32 = utils::box_type_u32(['t', 'r', 'e', 'x']);
#[derive(Debug)]
pub struct TrackExtendsBox {
    track_id: u32,
    default_sample_description_index: u32,
    default_sample_duration: u32,
    default_sample_size: u32,
    default_sample_flags: u32,
}

impl TrackExtendsBox {
    pub const fn get_default_sample_duration(&self) -> u32 {
        self.default_sample_duration
    }
}

impl ParseBox for TrackExtendsBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        _size: usize,
    ) -> Result<Self, errors::Error> {
        let _ = stream.read_box_version_flag_header().await?;

        let track_id = stream.read_u32().await?;
        let default_sample_description_index = stream.read_u32().await?;
        let default_sample_duration = stream.read_u32().await?;
        let default_sample_size = stream.read_u32().await?;
        let default_sample_flags = stream.read_u32().await?;

        Ok(Self {
            track_id,
            default_sample_description_index,
            default_sample_duration,
            default_sample_size,
            default_sample_flags,
        })
    }
}
