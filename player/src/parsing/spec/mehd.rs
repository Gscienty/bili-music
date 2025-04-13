use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const MEHD: u32 = utils::box_type_u32(['m', 'e', 'h', 'd']);
#[derive(Debug)]
pub struct MovieExtendsHeaderBox {
    fragment_duration: u64,
}

impl ParseBox for MovieExtendsHeaderBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        _size: usize,
    ) -> Result<Self, errors::Error> {
        let (mut version, flag) = stream.read_box_version_flag_header().await?;
        if (flag[2] & 0x01) != 0 {
            version = 1;
        }

        let fragment_duration = if version == 1 {
            stream.read_u64().await?
        } else {
            stream.read_u32().await? as u64
        };

        Ok(Self { fragment_duration })
    }
}
