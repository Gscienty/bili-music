use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const SMHD: u32 = utils::box_type_u32(['s', 'm', 'h', 'd']);
#[derive(Debug)]
pub struct SoundMediaHeaderBox {
    balance: u16,
}

impl ParseBox for SoundMediaHeaderBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        _size: usize,
    ) -> Result<Self, errors::Error> {
        let _ = stream.read_box_version_flag_header().await?;

        let balance = stream.read_u16().await?;
        let _ = stream.read_u16().await?;

        Ok(Self { balance })
    }
}
