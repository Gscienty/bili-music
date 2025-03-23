use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const TFDT: u32 = utils::box_type_u32(['t', 'f', 'd', 't']);
#[derive(Debug)]
pub struct TrackFragmentBaseMediaDecodeTimeBox {
    base_media_decode_time: u64,
}

impl ParseBox for TrackFragmentBaseMediaDecodeTimeBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        _typ: utils::BoxType,
        _size: usize,
    ) -> Result<Self, errors::Error> {
        let (version, _) = stream.read_box_version_flag_header().await?;

        let base_media_decode_time = if version == 1 {
            stream.read_u64().await?
        } else {
            stream.read_u32().await? as u64
        };

        Ok(Self {
            base_media_decode_time,
        })
    }
}
