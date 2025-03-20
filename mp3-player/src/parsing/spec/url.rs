use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const URL: u32 = utils::box_type_u32(['u', 'r', 'l', ' ']);
#[derive(Debug)]
pub struct DataEntryUrlBox {
    location: Vec<u8>,
}

impl ParseBox for DataEntryUrlBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        _size: usize,
    ) -> Result<Self, errors::Error> {
        let (_, flag) = stream.read_box_version_flag_header().await?;
        let mut location = Vec::new();
        if flag[0] != 0x00 || flag[1] != 0x00 || flag[2] != 0x01 {
            loop {
                let b = stream.read_u8().await?;
                if b == 0 {
                    break;
                }
                location.push(b);
            }
        }
        Ok(Self { location })
    }
}
