use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const MFHD: u32 = utils::box_type_u32(['m', 'f', 'h', 'd']);
#[derive(Debug)]
pub struct MovieFragmentHeaderBox {
    sequence_number: u32,
}

impl ParseBox for MovieFragmentHeaderBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        _typ: utils::BoxType,
        _size: usize,
    ) -> Result<Self, errors::Error> {
        let _ = stream.read_box_version_flag_header().await?;
        let sequence_number = stream.read_u32().await?;

        Ok(Self { sequence_number })
    }
}
